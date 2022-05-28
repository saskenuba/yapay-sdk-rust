use std::borrow::Cow;
use std::fmt::Display;
use std::num::NonZeroU8;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use time::macros::format_description;
use time::{Date, OffsetDateTime};
use validator::Validate;

use crate::errors::InvalidError;
use crate::helpers::format_available_payment_method;
use crate::{CanValidate, SDKError};

lazy_static! {
    static ref REGEX_BIRTH_DATE: Regex = Regex::new(r"\d{2}/\d{2}/\d{4}$").unwrap();
    pub static ref CLEAN_WEBHOOK_REGEX: Regex =
        Regex::new(r"\&transaction\[products\]\[\].*?\&").unwrap();
}

/// Transaction Status
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug, strum::Display)]
pub enum TransactionStatus {
    #[serde(rename = "4")]
    AguardandoPagamento,
    #[serde(rename = "6")]
    Aprovada,
    #[serde(rename = "7")]
    Cancelada,
    #[serde(rename = "89")]
    Reprovada,

    /// The code 24 is for both Chargeback and Contestacao.
    /// Come on Yapay!
    #[serde(rename = "24")]
    Contestacao,
    #[serde(rename = "89")]
    Monitoring,
    // #[serde(rename = "W")]
    // Chargeback,
}

#[derive(Validate, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YapayCustomer {
    #[validate]
    pub contacts: Vec<CustomerPhoneContact>,
    #[validate]
    pub addresses: Vec<CustomerAddress>,
    #[validate(length(min = 1))]
    pub name: String,
    /// Format of DD/MM/YYYY
    #[validate(regex = "REGEX_BIRTH_DATE")]
    pub birth_date: String,
    /// Only numbers.
    #[validate(length(equal = 11), custom = "crate::helpers::validate_cpf")]
    pub cpf: String,
    pub cnpj: Option<String>,
    #[validate(email)]
    pub email: String,
}

impl YapayCustomer {
    pub fn new(
        name: String,
        cpf: String,
        email: String,
        birth_date: String,
        phones: Vec<CustomerPhoneContact>,
        address: Vec<CustomerAddress>,
    ) -> Result<Self, InvalidError> {
        let customer = Self {
            contacts: phones,
            addresses: address,
            name,
            birth_date,
            cpf,
            cnpj: None,
            email,
        };

        match customer.validate() {
            Ok(_) => Ok(customer),
            Err(e) => Err(InvalidError::ValidatorLibError(e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CustomerResponse {
    pub name: String,
    pub company_name: String,
    pub trade_name: String,
    pub cnpj: String,
}

#[derive(Validate, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerPhoneContact {
    pub type_contact: PhoneContactType,
    #[validate(length(min = 8, max = 15))]
    pub number_contact: String,
}

#[derive(Validate, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerAddress {
    pub type_address: AddressType,
    /// CEP somente números.
    #[validate(length(max = 8))]
    pub postal_code: String,
    pub street: String,
    pub number: String,
    pub completion: String,
    pub neighborhood: String,
    pub city: String,
    /// Somente a sigla do estado. Exemplo: SP
    #[validate(length(equal = 2))]
    pub state: String,
}

#[derive(Validate, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YapayProduct {
    pub code: String,
    #[validate(length(max = 50))]
    pub sku_code: String,
    pub description: String,
    pub quantity: String,
    pub price_unit: String,
    pub extra: Option<String>,
}

impl YapayProduct {
    #[must_use]
    pub fn new(
        sku_or_code: String,
        description: String,
        quantity: NonZeroU8,
        price_unit: f64,
    ) -> Self {
        Self {
            code: sku_or_code.clone(),
            sku_code: sku_or_code,
            description,
            quantity: quantity.get().to_string(),
            price_unit: price_unit.to_string(),
            extra: None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionResponseCommon {}

/// A payment transaction used on requests.
///
/// Use the available builder methods:
///
/// [`YapayTransaction::online_goods`] if there are no shipping address;
///
/// [`YapayTransaction::physical_goods`] if there IS a shipping address;
#[derive(Validate, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YapayTransaction {
    pub available_payment_methods: String,
    /// When [`Option::none`], this param will be the same as the [`transaction_id`] on
    /// `TransactionResponse`. You must ensure that his field is not repeated ever.
    #[validate(length(max = 20))]
    pub order_number: Option<String>,
    pub customer_ip: String,
    pub shipping_type: Option<String>,
    pub shipping_price: Option<String>,
    pub price_discount: String,
    /// URL in your server to receive IPN (Instant Payment Notification).
    pub url_notification: String,
    pub free: String,
}

impl YapayTransaction {
    /// An online transaction does not include a shipping address.
    ///
    /// `notification_url` should be an URL in your server to receive IPN (Instant Payment
    /// Notification).
    pub fn online_goods(
        order_number: String,
        customer_ip: String,
        available_payment_methods: Option<String>,
        notification_url: Option<&str>,
    ) -> Result<Self, SDKError> {
        let transaction = Self {
            available_payment_methods: available_payment_methods
                .unwrap_or("2,3,4,5,6,7,14,15,16,18,19,21,22,23".to_string()),
            order_number: Some(order_number),
            customer_ip,
            shipping_type: None,
            shipping_price: None,
            price_discount: "".to_string(),
            url_notification: notification_url.unwrap_or("").to_string(),
            free: "".to_string(),
        };

        if let Err(err) = transaction.validate() {
            return Err(InvalidError::ValidatorLibError(err).into());
        }
        Ok(transaction)
    }

    /// A physical product include a shipping address.
    pub fn physical_goods() {}
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionTrace {
    pub estimated_date: String,
}

/// Represents a card that was previously used to create a payment, and it was saved.
#[derive(Validate, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YaypaySavedCardData {
    /// Parte do sistema anti-fraude. Obrigatório nos cartões.
    ///
    /// Veja mais sobre em:
    /// [Yapay Fingerprint](https://intermediador.dev.yapay.com.br/#/transacao-fingerprint)
    pub finger_print: String,

    /// The card token UUID that was return after a payment request.
    ///
    /// Example: a66cf237-3541-45d1-ab9c-a6b6e3f795f5
    pub card_token: String,

    #[validate(length(max = 4))]
    pub card_cvv: String,
    #[validate(length(min = 1, max = 2))]
    pub split: String,
}

#[derive(Validate, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[validate(schema(function = "validate_card_exp"))]
pub struct YapayCardData {
    /// Parte do sistema anti-fraude. Obrigatório nos cartões.
    ///
    /// Veja mais sobre em:
    /// [Yapay Fingerprint](https://intermediador.dev.yapay.com.br/#/transacao-fingerprint)
    pub finger_print: String,
    pub payment_method_id: PaymentCreditCard,
    pub card_name: String,
    pub card_number: String,

    /// Month in format of MM.
    #[validate(length(equal = 2))]
    pub card_expdate_month: String,

    /// Year in format of YYYY.
    #[validate(length(equal = 4))]
    pub card_expdate_year: String,

    #[validate(length(max = 4))]
    pub card_cvv: String,
    #[validate(length(min = 1, max = 2))]
    pub split: String,
}

impl YapayCardData {
    pub fn new(
        cc: PaymentCreditCard,
        cc_owner_name: String,
        cc_number: String,
        cc_exp_mm: String,
        cc_exp_yyyy: String,
        cc_cvv: String,
        installments: i8,
    ) -> Result<Self, SDKError> {
        let payment = Self {
            finger_print: "".to_string(),
            payment_method_id: cc,
            card_name: cc_owner_name,
            card_number: cc_number,
            card_expdate_month: cc_exp_mm,
            card_expdate_year: cc_exp_yyyy,
            card_cvv: cc_cvv,
            split: installments.to_string(),
        };

        if let Err(err) = payment.validate() {
            Err(InvalidError::ValidatorLibError(err).into())
        } else {
            Ok(payment)
        }
    }
}

impl CanValidate for YapayCardData {}

pub fn validate_card_exp(card_data: &YapayCardData) -> Result<(), validator::ValidationError> {
    let now = OffsetDateTime::now_utc().date();
    let res = validate_card_expiration(
        now,
        &*card_data.card_expdate_month,
        &*card_data.card_expdate_year,
    );

    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(validator::ValidationError {
            code: Cow::from("exp_month and exp_year"),
            message: Some(Cow::from(err.to_string())),
            params: Default::default(),
        }),
    }
}

/// Month: 2 char
/// Year: 4 char
pub fn validate_card_expiration(
    time_cmp: Date,
    exp_month: &str,
    exp_year: &str,
) -> Result<(), SDKError> {
    let card_expiration = Date::parse(
        &*format!("01-{}-{}", exp_month, exp_year),
        format_description!("[day]-[month]-[year]"),
    )
    .unwrap();

    if card_expiration >= time_cmp {
        Ok(())
    } else {
        Err(InvalidError::CreditCardExpired.into())
    }
}

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum AddressType {
    #[serde(rename = "B")]
    Cobranca,
    #[serde(rename = "D")]
    Entrega,
}

/// Tabela de Contact
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug, strum::Display)]
pub enum PhoneContactType {
    #[serde(rename = "H")]
    Residencial,
    #[serde(rename = "M")]
    Celular,
    #[serde(rename = "W")]
    Comercial,
}

/// Tabela de Contact
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum PaymentType {
    Card(PaymentCreditCard),
    BankTransfer(PaymentOtherMethods),
}

#[derive(strum::Display, EnumIter, Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum PaymentOtherMethods {
    #[serde(rename = "6")]
    #[strum(serialize = "6")]
    Boleto,
    #[serde(rename = "27")]
    #[strum(serialize = "27")]
    PIX,
    /// Transferência online Itaú Shopline
    #[serde(rename = "7")]
    #[strum(serialize = "7")]
    BankTransferItauShopline,
    /// Transferência online Banco do Brasil
    #[serde(rename = "23")]
    #[strum(serialize = "23")]
    BankTransferBB,
}

#[derive(strum::Display, EnumIter, Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum PaymentCreditCard {
    #[serde(rename = "3")]
    #[strum(serialize = "3")]
    Visa,
    #[serde(rename = "4")]
    #[strum(serialize = "4")]
    MasterCard,
    #[serde(rename = "5")]
    #[strum(serialize = "5")]
    Amex,
    #[serde(rename = "16")]
    #[strum(serialize = "16")]
    Elo,
    #[serde(rename = "20")]
    #[strum(serialize = "20")]
    HiperCard,
    #[serde(rename = "25")]
    #[strum(serialize = "25")]
    HiperItau,
}

/// Methods that takes this trait, you should pass either `OtherMethods` or `CreditCard`.
pub trait AsPaymentMethod: Display + IntoEnumIterator {
    fn payment_methods_all() -> String {
        format_available_payment_method(&<Self as IntoEnumIterator>::iter().collect::<Vec<_>>())
    }
}
impl AsPaymentMethod for PaymentOtherMethods {}
impl AsPaymentMethod for PaymentCreditCard {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub first_six_digits: String,
    pub last_four_digits: String,
    pub expiration_month: i64,
    pub expiration_year: i64,

    pub card_number_length: i64,
    pub security_code_length: i64,

    #[serde(with = "time::serde::rfc3339")]
    pub date_created: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub date_last_updated: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub date_due: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseRoot<T> {
    pub message_response: ResponseMessage,
    pub data_response: T,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use time::macros::format_description;
    use time::Date;

    use crate::common_types::{
        validate_card_expiration, AsPaymentMethod, PaymentCreditCard, PaymentOtherMethods,
    };
    use crate::helpers::format_available_payment_method;

    #[test]
    fn cc_valid_date() {
        let fmt = format_description!("[year]/[month padding:zero]/[day]");
        let datetime = Date::parse("2022/05/01", &fmt).unwrap();

        let res = validate_card_expiration(datetime, "05", "2022");
        assert!(res.is_ok());
    }

    #[test]
    fn cc_invalid_date() {
        let fmt = format_description!("[year]/[month padding:zero]/[day]");
        let datetime = Date::parse("2022/05/01", &fmt).unwrap();

        let res = validate_card_expiration(datetime, "06", "2021");
        assert!(res.is_err());
    }

    #[test]
    fn t_cc_methods() {
        let res = PaymentCreditCard::payment_methods_all();
        assert_eq!(res, "3,4,5,16,20,25".to_string());
    }

    #[test]
    fn t_specific_methods() {
        let res = format_available_payment_method(&[
            PaymentOtherMethods::Boleto,
            PaymentOtherMethods::BankTransferBB,
        ]);
        assert_eq!(res, "6,23".to_string());
    }
}
