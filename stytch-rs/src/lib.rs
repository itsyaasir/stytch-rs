//! ## About

//! An unofficial Rust wrapper around the [Stytch](https://stytch.com/docs) for accessing Stytch services.

//! ## Install

// !`Cargo.toml`

//! ```toml
//! [dependencies]
//! stytch = "0.1.0"

//! In your lib or binary crate:

//! ```rs
//! use stytch::Stytch;
//! ```

//! ## Usage

//! ### Creating a `Stytch` client

//! You will first need to create an instance of the `Stytch` instance (the client). You are required to provide a **PROJECT_ID** and
//! **SECRET**

//! _NOTE_:

//! ```rust
//! let project_id = dotenv::var("PROJECT_ID").unwrap();
//! let secret = dotenv::var("SECRET").unwrap();
//! If you intend to use Test Environment, you can use the following:
//!
//!      Client
//! let client = Stytch::new(project_id, secret, Environment::Test);
//! ```
//!
//! If you intend to use Live Environment, you can use the following:
//!  ```rust
//! let project_id = dotenv::var("PROJECT_ID").unwrap();
//! let secret = dotenv::var("SECRET").unwrap();
//! If you intend to use Test Environment, you can use the following:
//!
//!      Client
//! let client = Stytch::new(project_id, secret, Environment::Live);
//! ```
//!
//! ### Services
//! * Magic Links
//! Login or Create
//! ```rust
//! use stytch::{Environment, Stytch, LoginOrCreateParams, Attributes};
//!    let attr = Attributes {
//!        ip_address: "0.0.0.0",
//!        user_agent:"user agent "
//!    }
//!     LoginOrCreateParams
//!    let params = LoginOrCreateParams::new("login_magic_url_here","signup_magic_url_here","login_expiration_minutes", "signup_expiration_minutes",attr,
//!    "create_user_as_pending");

//!    magic links
//!    let res = client
//!    .magic_links()
//!    .email("test@test.com")
//!    .login_or_create(params)
//!    .await
//! ```

pub mod api;
pub mod errors;
pub mod model;

pub use api::*;
pub use errors::*;
pub use model::*;
