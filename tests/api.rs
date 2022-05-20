use yapay_sdk_rust::common_types::{YapayCardData, YapayCustomer, YapayTransaction};
use yapay_sdk_rust::YapayEnv;

mod common;

#[test]
fn t_boleto_payment() {
    let yapay_sdk = common::setup_sdk();

    // let opts = YapayCardData::new();

    let customer = YapayCustomer::new(
        "Rufino Beltrano".to_string(),
        "20430213320".to_string(),
        "teste@teste.com.br".to_string(),
        "11/01/2000".to_string(),
        vec![],
        vec![],
    );

    // let transaction = YapayTransaction {
    //     available_payment_methods: "".to_string(),
    //     order_number: None,
    //     customer_ip: "".to_string(),
    //     shipping_type: "".to_string(),
    //     shipping_price: "".to_string(),
    //     price_discount: "".to_string(),
    //     url_notification: "".to_string(),
    //     free: "".to_string()
    // };
    //
    // let res = yapay_sdk
    //     .create_credit_card_payment(customer, , vec![], Default::default())
    //     .unwrap()
    //     .execute()
    //     .await;
}

#[test]
fn t_creditcard_payment() {}

#[tokio::test]
async fn t_simulate_payment_conditions() -> anyhow::Result<()> {
    let yapay_sdk = common::setup_sdk();

    let res = yapay_sdk
        .simulate_payment(100_f64)
        .execute(YapayEnv::PRODUCTION)
        .await;
    assert!(res.map(|_| true).unwrap_or(false));

    Ok(())
}
