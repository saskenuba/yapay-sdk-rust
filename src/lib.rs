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
//! let mp_sdk: YapaySDK = YapaySDKBuilder::with_token("MP_ACCESS_TOKEN");
//!
//! # }
//! ```
//!
//! Once the token is inserted, you can call methods on [`crate::YapaySDK`]
//!
//!
//!
//! # Creating a CheckoutPro Preference
//! ```no_run
//! use yapay_sdk_rust::common_types::{CheckoutProPayer, Item};
//! use yapay_sdk_rust::preferences::requests::CheckoutProPreferences;
//! use yapay_sdk_rust::transaction::requests::DocumentType;
//! use yapay_sdk_rust::YapaySDKBuilder;
//!
//! #[tokio::main]
//! async fn async_main() {
//!     let mp_sdk = YapaySDKBuilder::with_token("MP_ACCESS_TOKEN");
//!
//!     let sample_item =
//!         Item::minimal_item("Sample item".to_string(), "".to_string(), 15.00, 1).unwrap();
//!
//!     let preferences = CheckoutProPreferences::new()
//!         .set_items(vec![sample_item])
//!         .set_payer(CheckoutProPayer::minimal_payer(
//!             "fulano@beltrano.com.br".to_string(),
//!             DocumentType::CPF,
//!             41810524485,
//!         ));
//!
//!     mp_sdk
//!         .create_preferences_checkout_pro(preferences)
//!         .expect("Failed to validate checkout preference. Something is wrong.")
//!         .execute()
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

pub mod common_types;
pub mod errors;
pub mod helpers;
pub mod simulation;
pub mod transaction;
pub mod webhooks;

use std::fmt::format;
use std::marker::PhantomData;

use common_types::ResponseRoot;
use futures::TryFutureExt;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use validator::Validate;

use crate::common_types::{YapayCardData, YapayCustomer, YapayProduct, YapayTransaction};
use crate::errors::{ApiError, InvalidError, SDKError};
use crate::simulation::{PaymentTaxResponse, SimulatePayload, SimulationResponseWrapper};
use crate::transaction::creditcard::TransactionResponse;
use crate::transaction::{PaymentRequestRoot, TransactionResponseWrapper};

const API_PROD_BASE: &str = "https://api.intermediador.yapay.com.br/api";
const API_TEST_BASE: &str = "https://api.intermediador.sandbox.yapay.com.br/api";

pub trait AccessToken: erased_serde::Serialize {
    fn set_token(&mut self, token: String);
}

pub trait CanValidate: Serialize + Validate {}

erased_serde::serialize_trait_object!(AccessToken);

///
#[derive(Debug)]
pub struct YapaySDKBuilder {}

impl YapaySDKBuilder {
    /// Creates an [`YapaySDK`] ready to request the API.
    pub fn with_token<T>(account_token: T) -> YapaySDK
    where
        T: ToString,
    {
        let http_client = Client::builder()
            .cookie_store(true)
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

pub struct SDKRequest<'a, RP> {
    http_client: &'a Client,
    access_token: &'a str,
    method: Method,
    endpoint: &'a str,
    payload: String,
    response_type: PhantomData<RP>,
}

impl<'a, RP> SDKRequest<'a, RP> {
    pub fn from_sdk(sdk: &'a YapaySDK, method: Method, endpoint: &'a str, payload: String) -> Self {
        Self {
            http_client: &sdk.http_client,
            access_token: &*sdk.account_token,
            method,
            endpoint,
            response_type: Default::default(),
            payload,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum YapayEnv {
    PRODUCTION,
    TEST,
}

impl YapayEnv {
    pub fn link(self) -> &'static str {
        match self {
            YapayEnv::PRODUCTION => API_PROD_BASE,
            YapayEnv::TEST => API_TEST_BASE,
        }
    }
}

impl<'a, RP> SDKRequest<'a, RP> {
    /// Injects bearer token, and return response
    pub async fn execute(self, yapay_env: YapayEnv) -> Result<RP, SDKError>
    where
        RP: DeserializeOwned,
    {
        let api_endpoint = format!("{}{}", yapay_env.link(), self.endpoint);
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

pub type CardTransactionResponse = ResponseRoot<TransactionResponseWrapper<TransactionResponse>>;
pub type SimulationResponse = ResponseRoot<SimulationResponseWrapper<PaymentTaxResponse>>;

impl YapaySDK {
    /// Returns an error if it fails to validate any of its arguments.
    pub fn create_credit_card_payment(
        &self,
        customer: YapayCustomer,
        transaction: YapayTransaction,
        products: Vec<YapayProduct>,
        cc_payment_data: YapayCardData,
    ) -> Result<SDKRequest<CardTransactionResponse>, SDKError> {
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

        let payload = serde_json::to_string(&request_payload).unwrap();

        Ok(SDKRequest::from_sdk(
            self,
            Method::POST,
            "/v3/transactions/payment",
            payload,
        ))
    }

    pub fn simulate_payment(&self, total_amount: f64) -> SDKRequest<SimulationResponse> {
        let request_payload = SimulatePayload::new(self.account_token.clone(), total_amount);
        let payload = serde_json::to_string(&request_payload).unwrap();

        eprintln!(
            "payload = {}",
            serde_json::to_string_pretty(&request_payload).unwrap()
        );

        SDKRequest::from_sdk(
            self,
            Method::POST,
            "/v1/transactions/simulate_splitting",
            payload,
        )
    }
}
