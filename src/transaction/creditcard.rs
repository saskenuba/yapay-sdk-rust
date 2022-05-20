use serde::{Deserialize, Serialize};

use crate::common_types::{
    CustomerResponse, TransactionTrace, YapayCardData, YapayCustomer, YapayProduct,
    YapayTransaction,
};
use crate::AccessToken;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentCreditCardPayload {
    pub token_account: String,
    pub customer: YapayCustomer,
    pub transaction_product: Vec<YapayProduct>,
    pub transaction: YapayTransaction,
    pub transaction_trace: TransactionTrace,
    pub payment: YapayCardData,
}

impl AccessToken for PaymentCreditCardPayload {
    fn set_token(&mut self, token: String) {
        self.token_account = token.to_string();
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TransactionResponse {
    pub order_number: String,
    pub free: String,
    pub transaction_id: i64,
    pub status_name: String,
    pub status_id: i64,
    pub token_transaction: String,
    pub payment: CreditCardPaymentResponse,
    pub customer: CustomerResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditCardPaymentResponse {
    pub price_payment: String,
    pub price_original: String,
    pub payment_response: String,
    pub payment_response_code: String,
    pub url_payment: String,
    pub tid: String,
    pub split: i64,
    pub payment_method_id: i64,
    pub payment_method_name: String,
    pub linha_digitavel: serde_json::Value,
    pub card_token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CardTransactionResponse, PaymentRequestRoot};

    #[test]
    fn t_credit_card_request() {
        let jd = serde_json::from_str::<PaymentRequestRoot<YapayCardData>>(include_str!(
            "../../tests/assets/creditcard_request.json"
        ))
        .unwrap();

        // assert!(jd.is_ok());
    }

    #[test]
    fn t_credit_card_response() {
        let jd = serde_json::from_str::<CardTransactionResponse>(include_str!(
            "../../tests/assets/creditcard_response.json"
        ))
        .unwrap();

        // assert!(jd.is_ok());
    }
}
