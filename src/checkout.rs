use reqwest::IntoUrl;
use serde::Serialize;
use validator::Validate;

use crate::{InvalidError, SDKError, YapayProduct};

#[derive(Validate, Default, Debug, Clone, PartialEq, Serialize)]
pub struct CheckoutPreferences {
    order_number: String,

    transaction_products: Vec<YapayProduct>,

    /// URL para redirecionamento caso concluída a transação com sucesso
    url_success: Option<String>,
    /// URL para redirecionamento caso pedido esteja aguardando a confirmação de pagamento
    url_process: Option<String>,
    /// URL para redirecionamento caso concluída a transação mas ocorreu falha no pagamento
    url_cancel: Option<String>,

    /// URL para onde deve notificado após mudanças no status de pagamento.
    notification_url: Option<String>,
}

impl CheckoutPreferences {
    pub fn new(order_number: String, products: Vec<YapayProduct>) -> Result<Self, SDKError> {
        let builder = Self {
            order_number,
            transaction_products: products,
            url_success: None,
            url_process: None,
            url_cancel: None,
            notification_url: None,
        };

        if let Err(err) = builder.validate() {
            return Err(InvalidError::ValidatorLibError(err).into());
        }

        Ok(builder)
    }

    pub fn set_notification_url<U>(mut self, url: U) -> Result<Self, SDKError>
    where
        U: IntoUrl,
    {
        let res = url
            .into_url()
            .map(|a| a.as_str().to_string())
            .map_err::<SDKError, _>(|e| InvalidError::URLError(e).into())?;

        self.notification_url = Some(res);
        Ok(self)
    }

    pub fn to_form(self, token: &str) -> String {
        let mut base_vec = vec![
            ("token_account", token.to_string()),
            ("order_number", self.order_number),
        ];

        for item in self.transaction_products {
            base_vec.push(("transaction_product[][description]", item.description));
            base_vec.push(("transaction_product[][quantity]", item.quantity));
            base_vec.push(("transaction_product[][price_unit]", item.price_unit));
        }

        if let Some(url) = self.notification_url {
            base_vec.push(("notification_url", url));
        }

        let mut querystring = String::new();
        base_vec
            .into_iter()
            .enumerate()
            .for_each(|(idx, (key, val))| {
                if idx == 0 {
                    querystring.push_str(&*format!("{}={}", key, val));
                } else {
                    querystring.push_str(&*format!("&{}={}", key, val));
                }
            });

        querystring
    }
}
