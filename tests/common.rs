use yapay_sdk_rust::{YapaySDK, YapaySDKBuilder};

pub fn setup_sdk() -> YapaySDK {
    YapaySDKBuilder::with_token(env!("TOKEN_ACCOUNT"))
}
