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
        // Extract attributes from params and assign to Attributes struct
        let attributes = params.attributes;

        // Hashmap
        let mut attributes_map: HashMap<String, String> = HashMap::new();

        // Create a new HashMap to store the attributes
        if attributes.is_some() {
            let attributes = attributes.unwrap();

            attributes_map.insert("ip_address".to_string(), attributes.ip_address);
            attributes_map.insert("user_agent".to_string(), attributes.user_agent);
        }

        let data = serde_json::json!({
            "email": self.email,
            "login_magic_link_url": params.login_magic_link_url,
            "signup_magic_link_url": params.signup_magic_link_url,
            "login_expiration_minutes": params.login_expiration_minutes.unwrap_or(5),
            "signup_expiration_minutes": params.signup_expiration_minutes.unwrap_or(5),
            "attributes": attributes_map,
            "create_user_as_pending": params.create_user_as_pending.unwrap_or(false),
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
        // Extract attributes from params and assign to Attributes struct
        let attributes = params.attributes;

        let mut attributes_map = HashMap::new();
        if attributes.is_some() {
            let attributes = attributes.unwrap();

            attributes_map.insert("ip_address".to_string(), attributes.ip_address);
            attributes_map.insert("user_agent".to_string(), attributes.user_agent);
        }

        let data = serde_json::json!({
            "email": self.email,
            "invite_magic_link_url": params.invite_magic_link_url,
            "invite_expiration_minutes": params.invite_expiration_minutes.unwrap(),
            "attributes": attributes_map,
            "name" : {
                "first_name": params.first_name.unwrap_or("".to_string()),
                "last_name": params.last_name.unwrap_or("".to_string()),
                "middle_name": params.middle_name.unwrap_or("".to_string()),
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
