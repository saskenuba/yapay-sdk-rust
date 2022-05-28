use reqwest::IntoUrl;
use serde::Serialize;
use validator::Validate;

use crate::common_types::AsPaymentMethod;
use crate::helpers::format_available_payment_method;
use crate::{InvalidError, SDKError, YapayProduct};

/// Preferences to create your checkout.
///
///
/// [`notification_url`] is optional, but highly recommended way to receive payment updates.
///
///
/// [`available_payment_methods`] is used to restrict payment options, and can be used with the
/// `set_available_payment_methods` builder.
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

    ///
    available_payment_methods: Option<String>,
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
            available_payment_methods: None,
        };

        if let Err(err) = builder.validate() {
            return Err(InvalidError::ValidatorLibError(err).into());
        }

        Ok(builder)
    }

    /// Restricts payment methods to those used on `methods` parameter.
    ///
    ///
    /// # Usage
    ///
    /// ```
    /// use std::num::NonZeroU8;
    ///
    /// use yapay_sdk_rust::checkout::CheckoutPreferences;
    /// use yapay_sdk_rust::common_types::{AsPaymentMethod, PaymentCreditCard, YapayProduct};
    ///
    /// let product = YapayProduct::new(
    ///     "sample".to_string(),
    ///     "a sample product".to_string(),
    ///     NonZeroU8(1).unwrap(),
    ///     10_f64,
    /// );
    ///
    /// let preferences = CheckoutPreferences::new("order_number".to_string(), vec![product])
    ///     .unwrap()
    ///     .set_available_payment_methods(&[PaymentCreditCard::payment_methods_all()]);
    ///
    /// // now you can only pay with credit cards
    /// ```
    pub fn set_available_payment_methods<PM>(mut self, methods: &[PM]) -> Self
    where
        PM: AsPaymentMethod,
    {
        self.available_payment_methods = Some(format_available_payment_method(methods));
        self
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

    /// Sets the `url_process`, which will redirect the user after payment.
    /// This is the standard way of redirecting, it doesn't matter if the transaction failed, or was
    /// a success.
    ///
    ///
    /// You can use this to trigger a response to your server that the user has finished a payment,
    /// and now needs to wait for a definitive response.
    pub fn set_process_url<U>(mut self, url: U) -> Result<Self, SDKError>
    where
        U: IntoUrl,
    {
        let res = url
            .into_url()
            .map(|a| a.as_str().to_string())
            .map_err::<SDKError, _>(|e| InvalidError::URLError(e).into())?;

        self.url_process = Some(res);
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

        if let Some(payment_methods) = self.available_payment_methods {
            base_vec.push(("available_payment_methods", payment_methods));
        }

        if let Some(url) = self.url_process {
            base_vec.push(("url_process", url));
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
