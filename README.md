# Stytch Rust

<p>
   <a href="https://crates.io/crates/mpesa" target="_blank">
     <img alt="Version" src="https://img.shields.io/crates/v/stytch" />
   </a>
  <a href="https://docs.rs/stytch-rs" target="_blank">
    <img alt="Documentation" src="https://docs.rs/stytch/badge.svg" />
  </a>
  <a href="LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
</p>

## About

An unofficial Rust wrapper around the [Stytch](https://stytch.com/docs) for accessing Stytch services.

## Install

`Cargo.toml`

```toml
[dependencies]
stytch = "0.2.1"

In your lib or binary crate:

```rs
use stytch::Stytch;
```

## Usage

### Creating a `Stytch` client

You will first need to create an instance of the `Stytch` instance (the client). You are required to provide a **PROJECT_ID** and
**SECRET**

If you intend to use Test Environment, use the following:

```rust
let project_id = dotenv::var("PROJECT_ID").unwrap();
let secret = dotenv::var("SECRET").unwrap();
    //  Client
let client = Stytch::new(project_id, secret, Environment::Test);
```

If you intend to use Live Environment, you can use the following:

 ```rust
let project_id = dotenv::var("PROJECT_ID").unwrap();
let secret = dotenv::var("SECRET").unwrap();
     // Client
let client = Stytch::new(project_id, secret, Environment::Live);
```

### Services

## Magic Links

* Login or Create

```rust
use stytch::{Environment, Stytch, LoginOrCreateParams, Attributes};
    // Attributes
   let attr = Attributes {
       ip_address: "0.0.0.0".into(),
       user_agent:"user agent".into()
   };
  // Login or Create Paramters
   let params = LoginOrCreateParams::new("login_magic_url_here",
   "signup_magic_url_here",
   Some(5),
   Some(5),
   Some(attr),
   Some(true));

   // magic links
   let res = client
   .magic_links()
   .email("test@test.com".into())
   .login_or_create(params)
   .await;
```

### More Examples will be added

## Author

**Yasir Shariff**

* Twitter: [@itsyaasir](https://twitter.com/itsyaasir)
* Not affiliated with Stytch.

Copyright © 2021 [Yasir Shariff](https://github.com/itsyaasir).<br />
This project is [MIT](LICENSE) licensed.
