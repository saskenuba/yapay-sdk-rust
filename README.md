# Yapay SDK


[![Crate version on crates.io](https://img.shields.io/crates/v/yapay-sdk-rust)](https://crates.io/crates/yapay-sdk-rust)
[![Crate documentation on docs.rs](https://img.shields.io/docsrs/yapay-sdk-rust)](https://docs.rs/yapay-sdk-rust)
![Crate license](https://img.shields.io/crates/l/yapay-sdk-rust)


<!-- cargo-rdme start -->

An open source, strongly-typed SDK for the Yapay API.

It will try to hold your hand and reduce the possibility of errors, providing the correct API
surface.

### Note

The library is still under development and its public API is subject to change.

# Installation

Added the following into your Cargo.toml:

```toml
yapay_sdk_rust = "0.1"
```

# Usage

The client is built using the
[`YapaySDKBuilder::with_token`](https://docs.rs/yapay-sdk-rust/latest/yapay_sdk_rust/struct.YapaySDKBuilder.html) `with_token`
method.

```rust
use yapay_sdk_rust::{YapaySDK, YapaySDKBuilder};

let mp_sdk: YapaySDK = YapaySDKBuilder::with_token("MP_ACCESS_TOKEN");

```

Once the token is inserted, you can call methods on [`crate::YapaySDK`]



# Creating a CheckoutPro Preference
```rust
use yapay_sdk_rust::common_types::{CheckoutProPayer, Item};
use yapay_sdk_rust::payments::requests::DocumentType;
use yapay_sdk_rust::preferences::requests::CheckoutProPreferences;
use yapay_sdk_rust::YapaySDKBuilder;

#[tokio::main]
async fn async_main() {
    let mp_sdk = YapaySDKBuilder::with_token("MP_ACCESS_TOKEN");

    let sample_item =
        Item::minimal_item("Sample item".to_string(), "".to_string(), 15.00, 1).unwrap();

    let preferences = CheckoutProPreferences::new()
        .set_items(vec![sample_item])
        .set_payer(CheckoutProPayer::minimal_payer(
            "fulano@beltrano.com.br".to_string(),
            DocumentType::CPF,
            41810524485,
        ));

    mp_sdk
        .create_preferences_checkout_pro(preferences)
        .expect("Failed to validate checkout preference. Something is wrong.")
        .execute()
        .await
        .unwrap();
}
```

# Other Examples

Check out the `tests` folder inside our repository to check for more examples.

# License
Project is licensed under the permissive MIT license.

<!-- cargo-rdme end -->
