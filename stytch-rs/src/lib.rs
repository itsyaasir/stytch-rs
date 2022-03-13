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

//!      Client
//! let client = Stytch::new(project_id, secret, Environment::Test);
//! ```

pub mod api;
pub mod errors;
pub mod model;

pub use api::*;
pub use errors::*;
pub use model::*;
