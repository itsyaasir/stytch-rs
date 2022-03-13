pub mod base;
pub mod magiclink;
pub mod otp;

pub use base::Base;
pub use magiclink::{email::Email, magic_links::MagicLinks};
pub use otp::otp::Otp;
