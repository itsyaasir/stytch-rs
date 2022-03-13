# mpesa-rust

<p>
   <a href="https://crates.io/crates/mpesa" target="_blank">
     <img alt="Version" src="https://img.shields.io/crates/v/stytch-rs" />
   </a>
  <a href="https://docs.rs/stytch-rs" target="_blank">
    <img alt="Documentation" src="https://docs.rs/stytch-rs/badge.svg" />
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
stytch = "0.1.0"


In your lib or binary crate:

```rs
use stytch::*;
```

## Usage

### Creating a `Stytch` client

You will first need to create an instance of the `Stytch` instance (the client). You are required to provide a **PROJECT_ID** and
**SECRET**

_NOTE_:

```rust
let project_id = dotenv::var("PROJECT_ID").unwrap();
let secret = dotenv::var("SECRET").unwrap();

    // Client
let client = Stytch::new(project_id, secret, Environment::Test);
```

More will be added progressively, pull requests welcome

## Author

**Yasir Shariff**

- Twitter: [@itsyaasir\_](https://twitter.com/itsyaasir)
- Not affiliated with Stytch.

Copyright © 2021 [Yasir Shariff](https://github.com/itsyaasir).<br />
This project is [MIT](LICENSE) licensed.
