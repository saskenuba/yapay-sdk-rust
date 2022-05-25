//! You can setup your webhooks only

use serde::{Deserialize, Serialize};

use crate::common_types::CLEAN_WEBHOOK_REGEX;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookRoot {
    pub token_transaction: String,
    pub transaction: WebhookTransaction,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookTransaction {
    pub order_number: String,
    pub free: String,
    pub transaction_id: String,
    pub status_name: String,
    pub status_id: String,
    pub date_transaction: String,
    pub split: String,
    pub price_payment: String,
    pub date_payment: String,
    pub seller_token: String,
    pub transaction_token: String,
    pub token_transaction: String,
    pub price_seller: String,
    pub price_original: String,
    pub price_additional: String,
    pub price_discount: String,
    pub shipping_price: String,
    pub shipping_type: String,
    pub payment_method_id: String,
    pub payment_method_name: String,
    pub customer: Customer,
    pub company: Company,
    pub payment: Payment,
    // Broken due to non-indexed
    // pub products: Vec<Product>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub code: Option<String>,
    pub description: Option<String>,
    pub extra: Option<String>,
    pub price_unit: Option<String>,
    pub quantity: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Customer {
    pub name: String,
    pub cpf: String,
    pub cnpj: String,
    pub email: String,
    pub token: String,
    pub address: Address,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub number: String,
    pub neighborhood: String,
    pub postal_code: String,
    pub completion: String,
    pub city: String,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    pub cnpj: String,
    pub cpf: String,
    pub contact: String,
    pub url: String,
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    pub price_payment: String,
    pub payment_response: String,
    pub payment_response_code: String,
    pub url_payment: String,
    pub date_approval: String,
    pub selling_message: String,
    pub number_of_voucher_sales: String,
    pub split: String,
    pub date_payment: String,
    pub payment_method_id: String,
    pub payment_method_name: String,
    pub card_id: String,
    pub number_proccess: String,
}

pub fn clean_non_indexed(raw: &str) -> String {
    // let l_idx = raw.find("&transaction[products][]").unwrap();
    // let r_idx = raw.rfind("&transaction[products][]").unwrap();

    let mut locs = vec![];
    for needle in CLEAN_WEBHOOK_REGEX.find_iter(raw) {
        let start = needle.start();
        let finish = needle.end();
        locs.push((start, finish));
    }

    let first_idx = locs[0].0 + 1;
    let last_idx = locs.last().map(|&(_, l)| l).unwrap();

    let first_chunk = &raw[..first_idx];
    let last_chunk = &raw[last_idx..];

    first_chunk.to_string() + last_chunk
}

#[cfg(test)]
mod tests {
    use serde_qs::Config;

    use super::*;

    // Just a heads-up.
    // Yapay posts the webhook with broken indexing on `products` field.
    // So we just clean it up... don't forget it!

    // Raw body of a POST received from Yapay after cleaning up non-indexed array
    const fn post_cleaned() -> &'static str {
        r#"token_transaction=938ad7ab4b1c6bb227f03b1b0fe08f67&transaction[order_number]=c2357843-db24-4a06-b&transaction[free]=&transaction[transaction_id]=670863&transaction[status_name]=Aprovada&transaction[status_id]=6&transaction[date_transaction]=2022-05-24T23%3A09%3A21&transaction[split]=3&transaction[price_payment]=104.01&transaction[date_payment]=2022-05-24T23%3A09%3A21&transaction[seller_token]=73047784b36c659&transaction[transaction_token]=938ad7ab4b1c6bb227f03b1b0fe08f67&transaction[token_transaction]=938ad7ab4b1c6bb227f03b1b0fe08f67&transaction[price_seller]=95.51&transaction[price_original]=100.0&transaction[price_additional]=0.0&transaction[price_discount]=0.0&transaction[shipping_price]=0&transaction[shipping_type]=&transaction[payment_method_id]=4&transaction[payment_method_name]=Mastercard&transaction[customer][name]=Rufino%20Beltrano&transaction[customer][cpf]=41810420814&transaction[customer][cnpj]=&transaction[customer][email]=saskenuba%40gmail.com&transaction[customer][token]=63db358c1adbc36&transaction[company][name]=Yapay%20Sandbox&transaction[company][cnpj]=&transaction[company][cpf]=90357966678&transaction[company][contact]=1112312312&transaction[company][url]=&transaction[company][token]=73047784b36c659&transaction[payment][price_payment]=104.01&transaction[payment][payment_response]=&transaction[payment][payment_response_code]=&transaction[payment][url_payment]=&transaction[payment][date_approval]=24%2F05%2F2022%20-%2023%3A09%3A21&transaction[payment][selling_message]=Mensagem%20de%20venda%20fake&transaction[payment][number_of_voucher_sales]=03076032815324372004&transaction[payment][split]=3&transaction[payment][date_payment]=24%2F05%2F2022&transaction[payment][payment_method_id]=4&transaction[payment][payment_method_name]=Mastercard&transaction[payment][card_id]=64296&transaction[payment][number_proccess]=705036&transaction[customer][address][street]=Av%20Bartholomeu%20de%20carlos&transaction[customer][address][number]=901&transaction[customer][address][neighborhood]=Jardim%20Flor%20da%20Montanha&transaction[customer][address][postal_code]=07097420&transaction[customer][address][completion]=&transaction[customer][address][city]=Guarulhos&transaction[customer][address][state]=SP"#
    }

    // Raw body of a POST received from Yapay
    const fn post_not_cleaned() -> &'static str {
        r#"token_transaction=938ad7ab4b1c6bb227f03b1b0fe08f67&transaction[order_number]=c2357843-db24-4a06-b&transaction[free]=&transaction[transaction_id]=670863&transaction[status_name]=Aprovada&transaction[status_id]=6&transaction[date_transaction]=2022-05-24T23%3A09%3A21&transaction[split]=3&transaction[price_payment]=104.01&transaction[date_payment]=2022-05-24T23%3A09%3A21&transaction[seller_token]=73047784b36c659&transaction[transaction_token]=938ad7ab4b1c6bb227f03b1b0fe08f67&transaction[token_transaction]=938ad7ab4b1c6bb227f03b1b0fe08f67&transaction[price_seller]=95.51&transaction[price_original]=100.0&transaction[price_additional]=0.0&transaction[price_discount]=0.0&transaction[shipping_price]=0&transaction[shipping_type]=&transaction[payment_method_id]=4&transaction[payment_method_name]=Mastercard&transaction[products][][code]=100brl&transaction[products][][description]=100%20reais%20de%20cr%C3%A9dito%20na%20carteira%20MercadoSkin.&transaction[products][][extra]=&transaction[products][][price_unit]=100.0&transaction[products][][quantity]=1.0&transaction[customer][name]=Rufino%20Beltrano&transaction[customer][cpf]=41810420814&transaction[customer][cnpj]=&transaction[customer][email]=saskenuba%40gmail.com&transaction[customer][token]=63db358c1adbc36&transaction[company][name]=Yapay%20Sandbox&transaction[company][cnpj]=&transaction[company][cpf]=90357966678&transaction[company][contact]=1112312312&transaction[company][url]=&transaction[company][token]=73047784b36c659&transaction[payment][price_payment]=104.01&transaction[payment][payment_response]=&transaction[payment][payment_response_code]=&transaction[payment][url_payment]=&transaction[payment][date_approval]=24%2F05%2F2022%20-%2023%3A09%3A21&transaction[payment][selling_message]=Mensagem%20de%20venda%20fake&transaction[payment][number_of_voucher_sales]=03076032815324372004&transaction[payment][split]=3&transaction[payment][date_payment]=24%2F05%2F2022&transaction[payment][payment_method_id]=4&transaction[payment][payment_method_name]=Mastercard&transaction[payment][card_id]=64296&transaction[payment][number_proccess]=705036&transaction[customer][address][street]=Av%20Bartholomeu%20de%20carlos&transaction[customer][address][number]=901&transaction[customer][address][neighborhood]=Jardim%20Flor%20da%20Montanha&transaction[customer][address][postal_code]=07097420&transaction[customer][address][completion]=&transaction[customer][address][city]=Guarulhos&transaction[customer][address][state]=SP"#
    }

    #[test]
    fn t_clean_post() {
        assert_eq!(post_cleaned(), clean_non_indexed(post_not_cleaned()));
    }

    #[test]
    fn t_notification() {
        let cfg = Config::new(10, false);
        let result = cfg
            .deserialize_str::<WebhookRoot>(&*clean_non_indexed(post_not_cleaned()))
            .unwrap();

        eprintln!("result = {:#?}", result);
    }
}
