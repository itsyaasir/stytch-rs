use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// `LoginOrCreateParams` is used as parameter to create a magic link for logging in or creating a user.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginOrCreateParams {
    pub login_magic_link_url: Option<String>,
    pub signup_magic_link_url: Option<String>,
    pub login_expiration_minutes: Option<i32>,
    pub signup_expiration_minutes: Option<i32>,
    pub attributes: Option<HashMap<String, String>>,
    pub create_user_as_pending: Option<bool>,
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
    pub invite_magic_link_url: Option<String>,
    pub invite_expiration_minutes: Option<i32>,
    pub attributes: Option<HashMap<String, String>>,
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

// type MagicLinksAuthenticateResponse struct {
// 	RequestID    string  `json:"request_id,omitempty"`
// 	StatusCode   int     `json:"status_code,omitempty"`
// 	UserID       string  `json:"user_id,omitempty"`
// 	MethodID     string  `json:"method_id,omitempty"`
// 	SessionToken string  `json:"session_token,omitempty"`
// 	Session      Session `json:"session,omitempty"`
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthenticateResponse {
    pub request_id: String,
    pub status_code: i32,
    pub user_id: String,
    pub method_id: String,
    pub session_token: String,
    pub session: String,
}
