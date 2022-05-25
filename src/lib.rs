//! An open source, strongly-typed SDK for the Yapay API.
//!
//! It will try to hold your hand and reduce the possibility of errors, providing the correct API
//! surface.
//!
//! ### Note
//!
//! The library is still under development and its public API is subject to change.
//!
//! # Installation
//!
//! Added the following into your Cargo.toml:
//!
//! ```toml
//! yapay_sdk_rust = "0.1"
//! ```
//!
//! # Usage
//!
//! The client is built using the
//! [`YapaySDKBuilder::with_token`](crate::YapaySDKBuilder) `with_token`
//! method.
//!
//! ```rust
//! # fn main() {
//! use yapay_sdk_rust::{YapaySDK, YapaySDKBuilder};
//!
//! let mp_sdk: YapaySDK = YapaySDKBuilder::with_token("YAPAY_ACCOUNT_TOKEN");
//!
//! # }
//! ```
//!
//! Once the token is inserted, you can call methods on [`crate::YapaySDK`]
//!
//!
//!
//! # Creating a Checkout link
//!
//! You can easily retrieve a checkout link with the method below.
//!
//! ```no_run
//! use std::num::NonZeroU8;
//!
//! use uuid::Uuid;
//! use yapay_sdk_rust::checkout::CheckoutPreferences;
//! use yapay_sdk_rust::common_types::YapayProduct;
//! use yapay_sdk_rust::{YapayEnv, YapaySDKBuilder};
//!
//! #[tokio::main]
//! async fn async_main() {
//!     let yapay_sdk = YapaySDKBuilder::with_token("YAPAY_ACCOUNT_TOKEN");
//!
//!     let product = YapayProduct::new(
//!         "note-100sk".to_string(),
//!         "Notebook Cinza".to_string(),
//!         NonZeroU8::new(1).unwrap(),
//!         2453.50,
//!     );
//!
//!     let order_number = Uuid::new_v4().to_string();
//!     let checkout_preferences =
//!         CheckoutPreferences::new(order_number, vec![product]).set_notification_url();
//!
//!     let checkout_url = yapay_sdk
//!         .create_checkout_page(YapayEnv::PRODUCTION, checkout_preferences)
//!         .expect("Failed to checkout options. Something is wrong.")
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! # Other Examples
//!
//! Check out the `tests` folder inside our repository to check for more examples.
//!
//! # License
//! Project is licensed under the permissive MIT license.

pub mod checkout;
pub mod common_types;
pub mod errors;
pub mod helpers;
pub mod simulation;
pub mod transaction;
pub mod webhooks;

use std::marker::PhantomData;

use common_types::ResponseRoot;
use futures::TryFutureExt;
use reqwest::header::{CONTENT_TYPE, LOCATION};
use reqwest::redirect::Policy;
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;
use validator::Validate;

use crate::checkout::CheckoutPreferences;
use crate::common_types::{YapayCardData, YapayCustomer, YapayProduct, YapayTransaction};
use crate::errors::{ApiError, InvalidError, SDKError};
use crate::simulation::{PaymentTaxResponse, SimulatePayload, SimulationResponseWrapper};
use crate::transaction::creditcard::TransactionResponse;
use crate::transaction::{PaymentRequestRoot, TransactionResponseWrapper};

const API_PROD_BASE: &str = "https://api.intermediador.yapay.com.br/api";
const API_TEST_BASE: &str = "https://api.intermediador.sandbox.yapay.com.br/api";

const CHECKOUT_PROD_BASE: &str = "https://tc.intermediador.yapay.com.br/payment/transaction";
const CHECKOUT_TEST_BASE: &str =
    "https://tc-intermediador-sandbox.yapay.com.br/payment/transaction";

pub trait AccessToken: erased_serde::Serialize {
    fn set_token(&mut self, token: String);
}

pub trait CanValidate: Serialize + Validate {}

erased_serde::serialize_trait_object!(AccessToken);

#[derive(Debug, Eq, PartialEq)]
pub enum YapayEnv {
    PRODUCTION,
    SANDBOX,
}

impl YapayEnv {
    pub const fn checkout_link(self) -> &'static str {
        match self {
            Self::PRODUCTION => CHECKOUT_PROD_BASE,
            Self::SANDBOX => CHECKOUT_TEST_BASE,
        }
    }

    pub const fn api_link(self) -> &'static str {
        match self {
            Self::PRODUCTION => API_PROD_BASE,
            Self::SANDBOX => API_TEST_BASE,
        }
    }
}

///
#[derive(Debug)]
pub struct YapaySDKBuilder {}

impl YapaySDKBuilder {
    /// Creates an [`YapaySDK`] ready to request the API.
    pub fn with_token<T>(account_token: &T) -> YapaySDK
    where
        T: ToString,
    {
        let http_client = Client::builder()
            .cookie_store(true)
            .redirect(Policy::none())
            .build()
            .expect("Failed to create client.");

        YapaySDK {
            http_client,
            account_token: account_token.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct YapaySDK {
    pub(crate) http_client: Client,
    pub(crate) account_token: String,
}

pub struct SDKJsonRequest<'a, RP> {
    http_client: &'a Client,
    method: Method,
    endpoint: &'a str,
    payload: String,
    response_type: PhantomData<RP>,
}

impl<'a, RP> SDKJsonRequest<'a, RP> {
    #[must_use]
    pub fn from_sdk(sdk: &'a YapaySDK, method: Method, endpoint: &'a str, payload: String) -> Self {
        Self {
            http_client: &sdk.http_client,
            method,
            endpoint,
            response_type: Default::default(),
            payload,
        }
    }
}

impl<'a, RP> SDKJsonRequest<'a, RP> {
    /// Injects bearer token, and return response
    pub async fn execute(self, yapay_env: YapayEnv) -> Result<RP, SDKError>
    where
        RP: DeserializeOwned,
    {
        let api_endpoint = format!("{}{}", yapay_env.api_link(), self.endpoint);
        println!("api endpoint: {:?}", api_endpoint);

        let request = self
            .http_client
            .request(self.method, api_endpoint)
            .body(self.payload)
            .header(CONTENT_TYPE, "application/json")
            .build()
            .unwrap();
        eprintln!("request = {:#?}", request);

        let response = self
            .http_client
            .execute(request)
            .and_then(|c| c.text())
            .await?;
        eprintln!("response = {}", response);

        // matches errors due to wrong payloads etc
        let error_jd = serde_json::from_str::<ApiError>(&*response);
        if let Ok(err) = error_jd {
            eprintln!("err = {:#?}", err);
            return Err(SDKError::PayloadError(err));
        }

        let jd = &mut serde_json::Deserializer::from_str(&*response);
        let res: Result<RP, _> = serde_path_to_error::deserialize(jd);

        match res {
            Ok(deserialized_resp) => Ok(deserialized_resp),
            Err(wow) => {
                println!("{:?}", wow.path());
                eprintln!("Error = {:#?}", wow);
                Err(SDKError::GenericError)
            }
        }
    }
}

// pub type WebhookResponse = ResponseRoot<TransactionResponseWrapper<Webhook>>;
pub type CardTransactionResponse = ResponseRoot<TransactionResponseWrapper<TransactionResponse>>;
pub type SimulationResponse = ResponseRoot<SimulationResponseWrapper<PaymentTaxResponse>>;

impl YapaySDK {
    pub async fn create_checkout_page(
        &self,
        yapay_env: YapayEnv,
        checkout_preferences: CheckoutPreferences,
    ) -> Result<String, SDKError> {
        let querystring = checkout_preferences.to_form(&*self.account_token);
        let request = self
            .http_client
            .request(Method::POST, yapay_env.checkout_link())
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(querystring)
            .build()
            .unwrap();

        let response = self.http_client.execute(request).await.unwrap();

        response
            .headers()
            .get(LOCATION)
            .and_then(|hdr| hdr.to_str().ok())
            .map(ToString::to_string)
            .ok_or(SDKError::GenericError)
    }

    /// Returns an error if it fails to validate any of its arguments.
    pub fn create_credit_card_payment(
        &self,
        customer: YapayCustomer,
        transaction: YapayTransaction,
        products: Vec<YapayProduct>,
        cc_payment_data: YapayCardData,
    ) -> Result<SDKJsonRequest<CardTransactionResponse>, SDKError> {
        let request_payload = PaymentRequestRoot::new(
            self.account_token.clone(),
            customer,
            products,
            transaction,
            cc_payment_data,
        );

        if let Err(errs) = request_payload.validate() {
            return Err(InvalidError::ValidatorLibError(errs).into());
        }

        let payload = serde_json::to_string(&request_payload).expect("Safe to unwrap.");
        eprintln!(
            "payload = {}",
            serde_json::to_string_pretty(&request_payload).unwrap()
        );

        Ok(SDKJsonRequest::from_sdk(
            self,
            Method::POST,
            "/v3/transactions/payment",
            payload,
        ))
    }

    #[must_use]
    pub fn simulate_payment(&self, total_amount: f64) -> SDKJsonRequest<SimulationResponse> {
        let request_payload = SimulatePayload::new(self.account_token.clone(), total_amount);
        let payload = serde_json::to_string(&request_payload).unwrap();

        eprintln!(
            "payload = {}",
            serde_json::to_string_pretty(&request_payload).unwrap()
        );

        SDKJsonRequest::from_sdk(
            self,
            Method::POST,
            "/v1/transactions/simulate_splitting",
            payload,
        )
    }
}
