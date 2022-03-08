use serde::{Deserialize, Serialize};

use super::magic_link_model::Attributes;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OTPsSMSSendParams {
    pub expiration_minutes: i32,
    pub attributes: Option<Attributes>,
}

impl OTPsSMSSendParams {
    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// use stytch_rs::model::otp_model::OTPsSMSSendParams;
    ///
    /// assert_eq!(OTPsSMSSendParams::new(expiration_minutes, attributes), );
    /// ```
    pub fn new(expiration_minutes: i32, attributes: Option<Attributes>) -> Self {
        Self {
            expiration_minutes,
            attributes,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OTPsSMSSendResponse {
    pub request_id: String,
    pub status_code: i32,
    pub user_id: String,
    pub phone_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OTPsSMSLoginOrCreateParams {
    pub expiration_minutes: Option<i32>,
    pub attributes: Option<Attributes>,
    pub create_user_as_pending: Option<bool>,
}

impl OTPsSMSLoginOrCreateParams {
    pub fn new(
        expiration_minutes: Option<i32>,
        attributes: Option<Attributes>,
        create_user_as_pending: Option<bool>,
    ) -> Self {
        Self {
            expiration_minutes,
            attributes,
            create_user_as_pending,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OTPsSMSLoginOrCreateResponse {
    pub request_id: String,
    pub status_code: i32,
    pub user_id: String,
    pub phone_id: String,
    pub user_created: bool,
}
