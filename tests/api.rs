use futures::TryFutureExt;
use uuid::Uuid;
use yapay_sdk_rust::checkout::CheckoutPreferences;
use yapay_sdk_rust::common_types::{PaymentCreditCard, YapayCardData, YapayTransaction};
use yapay_sdk_rust::YapayEnv;

mod common;

#[tokio::test]
async fn cc_payment() {
    let yapay_sdk = common::setup_sdk();

    let customer = common::valid_customer();
    let card_data = YapayCardData::new(
        PaymentCreditCard::MasterCard,
        "Joaquim Silva".to_string(),
        "3456 8564 1548 7894".to_string(),
        "06".to_string(),
        "2022".to_string(),
        "411".to_string(),
        3,
    )
    .unwrap();

    let wallet_credit = common::valid_product();

    let transaction = YapayTransaction::online_goods(
        Uuid::new_v4().to_string()[..20].to_string(),
        "127.0.0.1".to_string(),
        Some(PaymentCreditCard::payment_methods_all()),
        Some("https://webhook.site/966172d0-ed8f-497f-afcf-eec1727a628b"),
    )
    .unwrap();

    let res = yapay_sdk
        .create_credit_card_payment(customer, transaction, vec![wallet_credit], card_data)
        .unwrap()
        .execute(YapayEnv::SANDBOX)
        .await
        .unwrap();

    eprintln!("res = {:#?}", res);
}

#[tokio::test]
async fn t_simulate_payment_conditions() -> anyhow::Result<()> {
    let yapay_sdk = common::setup_sdk();

    let res = yapay_sdk
        .simulate_payment(100_f64)
        .execute(YapayEnv::SANDBOX)
        .await;
    assert!(res.map(|_| true).unwrap_or(false));

    Ok(())
}

#[tokio::test]
async fn t_checkout() -> anyhow::Result<()> {
    let yapay_sdk = common::setup_sdk();
    let sample_products = common::valid_products();

    let checkout_preferences =
        CheckoutPreferences::new(Uuid::new_v4().to_string(), sample_products)?;

    let res: Result<_, _> = yapay_sdk
        .create_checkout_page(YapayEnv::SANDBOX, checkout_preferences)
        .inspect_ok(|redirect_url| eprintln!("redirect_url = {:#?}", redirect_url))
        .await;

    assert!(res.is_ok());
    eprintln!("checkout_url = {}", res.unwrap());

    Ok(())
}
