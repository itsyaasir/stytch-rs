use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    errors::{Error, StytchErrorTypes},
    model::{
        client::Stytch,
        magic_link_model::{InviteParams, LoginOrCreateParams},
    },
};

use super::base::Base;

#[derive(Debug, Clone)]
pub struct MagicLinks<'a> {
    client: &'a Stytch,
}

impl<'a> MagicLinks<'a> {
    pub fn new(client: &'a Stytch) -> Self {
        Self { client }
    }

    pub fn email(&self, email: String) -> Email {
        Email {
            email,
            base: Base::new(self.client),
            client: self.client,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Email<'a> {
    pub email: String,
    pub base: Base,
    pub client: &'a Stytch,
}

impl<'a> Email<'a> {
    pub fn magic_link_url(&self) -> String {
        self.base.get_url("magic_links")
    }

    // Set login_magic_link_url

    pub async fn login_or_create(
        &self,
        params: LoginOrCreateParams,
    ) -> Result<reqwest::Response, crate::errors::StytchErrorTypes> {
        let params = LoginOrCreateParams {
            // if login_magic_link_url is not set, set it to ""
            login_magic_link_url: Some(params.login_magic_link_url.unwrap_or("".to_string())),
            signup_magic_link_url: Some(params.signup_magic_link_url.unwrap_or("".to_string())),
            login_expiration_minutes: Some(params.login_expiration_minutes.unwrap_or(5)),
            signup_expiration_minutes: Some(params.signup_expiration_minutes.unwrap_or(5)),
            attributes: Some(params.attributes.unwrap_or(HashMap::new())),
            create_user_as_pending: Some(params.create_user_as_pending.unwrap_or(false)),
        };

        let data = serde_json::json!({
            "email": self.email,
            "login_magic_link_url": params.login_magic_link_url.unwrap(),
            "signup_magic_link_url": params.signup_magic_link_url.unwrap(),
            "login_expiration_minutes": params.login_expiration_minutes.unwrap(),
            "signup_expiration_minutes": params.signup_expiration_minutes.unwrap(),
            "attributes": params.attributes.unwrap(),
            "create_user_as_pending": params.create_user_as_pending.unwrap(),
        });
        let url = format!("{}/email/login_or_create", self.magic_link_url());
        let res = self.base.post(url, data.to_string()).await;
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    Ok(res)
                } else {
                    let error = serde_json::from_str::<Error>(&res.text().await.unwrap());
                    Err(StytchErrorTypes::StytchError(error.unwrap()))
                }
            }
            Err(e) => Err(StytchErrorTypes::ReqwestError(e)),
        }
    }

    pub async fn invite(
        &self,
        params: InviteParams,
    ) -> Result<reqwest::Response, crate::errors::StytchErrorTypes> {
        let params = InviteParams {
            invite_magic_link_url: Some(params.invite_magic_link_url.unwrap_or("".to_string())),
            invite_expiration_minutes: Some(params.invite_expiration_minutes.unwrap_or(5)),
            attributes: Some(params.attributes.unwrap_or(HashMap::new())),
            first_name: Some(params.first_name.unwrap_or("".to_string())),
            last_name: Some(params.last_name.unwrap_or("".to_string())),
            middle_name: Some(params.middle_name.unwrap_or("".to_string())),
        };

        let data = serde_json::json!({
            "email": self.email,
            "invite_magic_link_url": params.invite_magic_link_url.unwrap(),
            "invite_expiration_minutes": params.invite_expiration_minutes.unwrap(),
            "attributes": params.attributes.unwrap(),
            "name" : {
                "first_name": params.first_name.unwrap(),
                "last_name": params.last_name.unwrap(),
                "middle_name": params.middle_name.unwrap(),
            }
        });
        let url = format!("{}/email/invite", self.magic_link_url());
        let res = self.base.post(url, data.to_string()).await;
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    Ok(res)
                } else {
                    let error = serde_json::from_str::<Error>(&res.text().await.unwrap());
                    Err(StytchErrorTypes::StytchError(error.unwrap()))
                }
            }
            Err(e) => Err(StytchErrorTypes::ReqwestError(e)),
        }
    }

    pub async fn revoke(&self) -> Result<reqwest::Response, crate::errors::StytchErrorTypes> {
        let url = format!("{}/email/revoke_invite", self.magic_link_url());
        let data = serde_json::json!({
            "email": self.email,
        });
        let res = self.base.post(url, data.to_string()).await;
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    Ok(res)
                } else {
                    let error = serde_json::from_str::<Error>(&res.text().await.unwrap());
                    Err(StytchErrorTypes::StytchError(error.unwrap()))
                }
            }
            Err(e) => Err(StytchErrorTypes::ReqwestError(e)),
        }
    }
}
