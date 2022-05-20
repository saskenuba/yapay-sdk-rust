use serde::{Deserialize, Serialize};

/// Wrapper for Transactions endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationResponseWrapper<T> {
    pub payment_methods: Vec<T>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulatePayload {
    pub token_account: String,
    pub price: String,
    pub type_response: String,
}

impl SimulatePayload {
    pub fn new<T>(token_account: String, total_amount: T) -> SimulatePayload
    where
        T: ToString,
    {
        Self {
            token_account,
            price: total_amount.to_string(),
            type_response: "J".to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentTaxResponse {
    pub splittings: Vec<SplitResponse>,
    pub price_customer: String,
    pub price_seller: String,
    pub price_original: String,
    pub split: i64,
    pub payment_method_name: String,
    pub payment_method_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitResponse {
    pub split: i64,
    pub value_split: String,
    pub value_transaction: String,

    ///
    pub addition_retention: String,

    /// Percent added per installment.
    // #[serde(
    //     default,
    //     deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string"
    // )]
    pub split_rate: serde_json::Value,

    /// The base value the seller will receive after the retention fee.
    pub price_seller: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common_types::ResponseRoot;

    // #[test]
    // fn t_simulate_response() {
    //     let jd = serde_json::from_str::<ResponseRoot<PaymentTaxResponse>>(include_str!(
    //         "../tests/assets/simulate_payment_response.json"
    //     ))
    //     .unwrap();
    //
    //     // assert!(jd.is_ok());
    // }
}
