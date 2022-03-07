use std::collections::HashMap;

use crate::{
    api::base::Base,
    errors::{Error, StytchErrorTypes},
    model::{
        client::Stytch,
        magic_link_model::{
            InviteParams, InviteResponse, LoginOrCreateParams, LoginOrCreateResponse,
            RevokeInviteResponse,
        },
    },
};

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
    ) -> Result<LoginOrCreateResponse, StytchErrorTypes> {
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
                    let res =
                        serde_json::from_str::<LoginOrCreateResponse>(&res.text().await.unwrap())
                            .expect("Could not parse LoginOrCreateResponse");
                    Ok(res)
                } else {
                    let error = serde_json::from_str::<Error>(&res.text().await.unwrap());
                    Err(StytchErrorTypes::StytchError(error.unwrap()))
                }
            }
            Err(e) => Err(StytchErrorTypes::ReqwestError(e)),
        }
    }

    pub async fn invite(&self, params: InviteParams) -> Result<InviteResponse, StytchErrorTypes> {
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
                    let res = serde_json::from_str::<InviteResponse>(&res.text().await.unwrap())
                        .expect("Could not parse InviteResponse");
                    Ok(res)
                } else {
                    let error = serde_json::from_str::<Error>(&res.text().await.unwrap());
                    Err(StytchErrorTypes::StytchError(error.unwrap()))
                }
            }
            Err(e) => Err(StytchErrorTypes::ReqwestError(e)),
        }
    }

    pub async fn revoke(&self) -> Result<RevokeInviteResponse, StytchErrorTypes> {
        let url = format!("{}/email/revoke_invite", self.magic_link_url());
        let data = serde_json::json!({
            "email": self.email,
        });
        let res = self.base.post(url, data.to_string()).await;
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    let res =
                        serde_json::from_str::<RevokeInviteResponse>(&res.text().await.unwrap())
                            .expect("Could not parse RevokeInviteResponse");
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
