use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginOrCreateParams {
    pub login_magic_link_url: Option<String>,
    pub signup_magic_link_url: Option<String>,
    pub login_expiration_minutes: Option<i32>,
    pub signup_expiration_minutes: Option<i32>,
    pub attributes: Option<HashMap<String, String>>,
    pub create_user_as_pending: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InviteParams {
    pub invite_magic_link_url: Option<String>,
    pub invite_expiration_minutes: Option<i32>,
    pub attributes: Option<HashMap<String, String>>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
}
