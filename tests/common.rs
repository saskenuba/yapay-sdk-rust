use std::num::NonZeroU8;

use yapay_sdk_rust::{
    AddressType, CustomerAddress, CustomerPhoneContact, PhoneContactType, YapayCustomer,
    YapayProduct, YapaySDK, YapaySDKBuilder,
};

pub fn setup_sdk() -> YapaySDK {
    YapaySDKBuilder::with_token(&env!("TOKEN_SANDBOX"))
}

pub fn valid_customer() -> YapayCustomer {
    YapayCustomer::new(
        "Rufino Beltrano".to_string(),
        "41810420814".to_string(),
        "saskenuba@gmail.com".to_string(),
        "11/01/2000".to_string(),
        vec![CustomerPhoneContact {
            type_contact: PhoneContactType::Residencial,
            number_contact: "11976914920".to_string(),
        }],
        vec![CustomerAddress {
            type_address: AddressType::Cobranca,
            postal_code: "07097420".to_string(),
            street: "Av Bartholomeu de carlos".to_string(),
            number: "901".to_string(),
            completion: "".to_string(),
            neighborhood: "Jardim Flor da Montanha".to_string(),
            city: "Guarulhos".to_string(),
            state: "SP".to_string(),
        }],
    )
    .unwrap()
}

pub fn valid_product() -> YapayProduct {
    YapayProduct::new(
        "wallet-100-brl".to_string(),
        "100 reais de crédito na carteira MercadoSkin.".to_string(),
        NonZeroU8::new(1).unwrap(),
        100_f64,
    )
}

pub fn valid_products() -> Vec<YapayProduct> {
    vec![
        YapayProduct::new(
            "".to_string(),
            "Maquina de lavar".to_string(),
            NonZeroU8::new(1).unwrap(),
            100_f64,
        ),
        YapayProduct::new(
            "".to_string(),
            "Notebook prata".to_string(),
            NonZeroU8::new(1).unwrap(),
            100_f64,
        ),
    ]
}
