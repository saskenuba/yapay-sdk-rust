use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::common_types::{TransactionTrace, YapayCustomer, YapayProduct, YapayTransaction};
use crate::CanValidate;

pub mod creditcard;

/// Wrapper for Transactions endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionResponseWrapper<T> {
    pub transaction: T,
}

/// The standard way to send requests.
#[derive(Validate, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentRequestRoot<T>
where
    T: CanValidate,
{
    pub token_account: String,
    pub customer: YapayCustomer,
    #[serde(rename = "transaction_product")]
    pub items: Vec<YapayProduct>,
    pub transaction: YapayTransaction,
    #[serde(rename = "transaction_trace")]
    pub trace: TransactionTrace,
    #[validate]
    pub payment: T,
}

impl<T> CanValidate for PaymentRequestRoot<T> where T: CanValidate {}

impl<T> PaymentRequestRoot<T>
where
    T: CanValidate,
{
    pub fn new(
        token_account: String,
        customer: YapayCustomer,
        items: Vec<YapayProduct>,
        transaction: YapayTransaction,
        payment: T,
    ) -> Self {
        Self {
            token_account,
            customer,
            items,
            transaction,
            trace: Default::default(),
            payment,
        }
    }
}
