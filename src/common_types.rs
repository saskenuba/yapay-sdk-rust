use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use time::macros::format_description;
use time::{Date, OffsetDateTime};
use validator::Validate;

use crate::errors::InvalidError;
use crate::{CanValidate, SDKError};

lazy_static! {
    static ref REGEX_BIRTH_DATE: Regex = Regex::new(r"\d{2}/\d{2}/\d{4}$").unwrap();
}

#[derive(Validate, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YapayCustomer {
    pub contacts: Vec<CustomerPhoneContact>,
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
    pub type_address: String,
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
    pub description: String,
    pub quantity: String,
    pub price_unit: String,
    pub code: String,
    #[validate(length(max = 50))]
    pub sku_code: String,
    pub extra: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionResponseCommon {}

#[derive(Validate, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YapayTransaction {
    pub available_payment_methods: String,
    /// When [`Option::none`], this param will be the same as the [`transaction_id`] on
    /// `TransactionResponse`. You must ensure that his field is not repeated ever.
    #[validate(length(max = 20))]
    pub order_number: Option<String>,
    pub customer_ip: String,
    pub shipping_type: String,
    pub shipping_price: String,
    pub price_discount: String,
    pub url_notification: String,
    pub free: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionTrace {
    pub estimated_date: String,
}

#[derive(Validate, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[validate(schema(function = "validate_card_exp"))]
pub struct YapayCardData {
    /// Parte do sistema anti-fraude. Obrigatório nos cartões.
    ///
    /// Veja mais sobre em:
    /// [Yapay Fingerprint](https://intermediador.dev.yapay.com.br/#/transacao-fingerprint)
    pub finger_print: String,
    pub payment_method_id: String,
    pub card_name: String,
    pub card_number: String,
    #[validate(length(equal = 2))]
    pub card_expdate_month: String,
    #[validate(length(equal = 4))]
    pub card_expdate_year: String,
    #[validate(length(max = 4))]
    pub card_cvv: String,
    #[validate(length(min = 1, max = 2))]
    pub split: String,
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
pub enum AddressPaymentType {
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
    BankTransfer(BankTransfer),
}

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum BankTransfer {
    #[serde(rename = "7")]
    ItauShopline,
    #[serde(rename = "23")]
    BancoDoBrasil,
}

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum CreditCard {
    #[serde(rename = "3")]
    Visa,
    #[serde(rename = "4")]
    MasterCard,
    #[serde(rename = "5")]
    Amex,
    #[serde(rename = "16")]
    Elo,
    #[serde(rename = "20")]
    HiperCard,
    #[serde(rename = "25")]
    HiperItau,
}

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

mod tests {
    #[cfg(test)]
    use time::macros::format_description;
    use time::Date;

    use crate::common_types::validate_card_expiration;

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
}
