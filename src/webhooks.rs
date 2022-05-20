//! You can setup your webhooks only

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookRequest {
    /// token da transacao
    token_transaction: String,
    transaction: WebhookTransactionInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookTransactionInfo {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {}
