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

let mp_sdk: YapaySDK = YapaySDKBuilder::with_token("YAPAY_ACCOUNT_TOKEN");

```

Once the token is inserted, you can call methods on [`crate::YapaySDK`]



# Creating a Checkout link

You can easily retrieve a checkout link with the method below.

```rust
use std::num::NonZeroU8;

use uuid::Uuid;
use yapay_sdk_rust::checkout::CheckoutPreferences;
use yapay_sdk_rust::common_types::YapayProduct;
use yapay_sdk_rust::{YapayEnv, YapaySDKBuilder};

#[tokio::main]
async fn async_main() {
    let yapay_sdk = YapaySDKBuilder::with_token("YAPAY_ACCOUNT_TOKEN");

    let product = YapayProduct::new(
        "note-100sk".to_string(),
        "Notebook Cinza".to_string(),
        NonZeroU8::new(1).unwrap(),
        2453.50,
    );

    let order_number = Uuid::new_v4().to_string();
    let checkout_preferences =
        CheckoutPreferences::new(order_number, vec![product]).set_notification_url();

    let checkout_url = yapay_sdk
        .create_checkout_page(YapayEnv::PRODUCTION, checkout_preferences)
        .expect("Failed to checkout options. Something is wrong.")
        .await
        .unwrap();
}
```

# Other Examples

Check out the `tests` folder inside our repository to check for more examples.

# License
Project is licensed under the permissive MIT license.

<!-- cargo-rdme end -->
