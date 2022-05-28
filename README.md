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

let yapay_sdk: YapaySDK = YapaySDKBuilder::with_token(env!("YAPAY_ACCOUNT_TOKEN"));

```

Once the token is inserted, you can call methods on [`crate::YapaySDK`]



# Creating a Checkout link

You can easily retrieve a checkout link with the method below.

```rust
use std::num::NonZeroU8;

use uuid::Uuid;
use yapay_sdk_rust::{
    CheckoutPreferences, PaymentCreditCard, YapayEnv, YapayProduct, YapaySDKBuilder,
};

#[tokio::main]
async fn async_main() {
    // your token, can come from environment or else
    let yapay_token = "YAPAY_ACCOUNT_TOKEN";
    let yapay_sdk = YapaySDKBuilder::with_token(yapay_token);

    let product = YapayProduct::new(
        "note-100sk".to_string(),
        "Notebook Cinza".to_string(),
        NonZeroU8::new(1).unwrap(),
        2453.50,
    );

    let order_number = Uuid::new_v4().to_string();
    let checkout_preferences = CheckoutPreferences::new(order_number, vec![product])
        .expect("Validation failed.")
        .set_notification_url("https://your-notifications-url.com")
        .expect("Notifications URL failed to validate.")
        .set_available_payment_methods(&PaymentCreditCard::payment_methods_all());

    let checkout_url = yapay_sdk
        .create_checkout_page(YapayEnv::PRODUCTION, checkout_preferences)
        .await
        .expect("Something went wrong creating the checkout.");
}
```

# Other Examples

Check out the `tests` folder inside our repository to check for more examples.

# License
Project is licensed under the permissive MIT license.

<!-- cargo-rdme end -->
