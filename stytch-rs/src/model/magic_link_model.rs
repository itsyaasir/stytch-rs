use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// `LoginOrCreateParams` is used as parameter to create a magic link for logging in or creating a user.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginOrCreateParams {
    pub login_magic_link_url: String,
    pub signup_magic_link_url: String,
    pub login_expiration_minutes: Option<i32>,
    pub signup_expiration_minutes: Option<i32>,
    pub attributes: Option<Attributes>,
    pub create_user_as_pending: Option<bool>,
}

impl LoginOrCreateParams {
    pub fn new(
        login_magic_link_url: String,
        signup_magic_link_url: String,
        login_expiration_minutes: Option<i32>,
        signup_expiration_minutes: Option<i32>,
        attributes: Option<Attributes>,
        create_user_as_pending: Option<bool>,
    ) -> Self {
        Self {
            login_magic_link_url,
            signup_magic_link_url,
            login_expiration_minutes,
            signup_expiration_minutes,
            attributes,
            create_user_as_pending,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attributes {
    pub ip_address: String,
    pub user_agent: String,
}

impl Attributes {
    pub fn new(ip_address: String, user_agent: String) -> Self {
        Self {
            ip_address,
            user_agent,
        }
    }

    pub fn ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = ip_address;
        self
    }

    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    pub fn none() -> HashMap<String, String> {
        HashMap::new()
    }
}

/// `LoginOrCreateResponse` is returned from `login_or_create`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginOrCreateResponse {
    pub request_id: String,
    pub status_code: i32,
    pub user_id: String,
    pub email_id: String,
    pub user_created: bool,
}

/// `InviteParams` is used as parameter to create a magic link for inviting a user.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InviteParams {
    pub invite_magic_link_url: String,
    pub invite_expiration_minutes: Option<i32>,
    pub attributes: Option<Attributes>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
}

/// `InviteResponse` is returned from `invite` method
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InviteResponse {
    pub request_id: String,
    pub status_code: i32,
    pub user_id: String,
    pub email_id: String,
}

/// `RevokeParams` is used as parameter to revoke a magic link.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RevokeInviteResponse {
    pub request_id: String,
    pub status_code: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthenticateResponse {
    pub request_id: String,
    pub status_code: i32,
    pub user_id: String,
    pub method_id: String,
    pub session_token: String,
    pub session: Option<String>,
}

// Session struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub started_at: String,
    pub last_accessed_at: String,
    pub expires_at: String,
    pub attributes: Attributes,
    // pub authentication_factors: Vec<AuthenticationFactor>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MagicLinksCreateParams {
    pub user_id: String,
    pub expiration_minutes: Option<i32>,
    pub attributes: Option<Attributes>,
}

//
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MagicLinksCreateResponse {
    pub request_id: String,
    pub status_code: i32,
    pub user_id: String,
    pub token: String,
}

