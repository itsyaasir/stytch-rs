use std::collections::HashMap;

use crate::model::client;

use super::base::Base;

#[derive(Debug, Clone)]
pub struct MagicLinks {
    pub email: Email,
}

impl MagicLinks {
    // pub fn new(email: String) -> Self {
    //     Self { email }
    // }

    // pub fn get_url(&self, base: &Base) -> String {
    //     format!("{}{}", base.get_url("magic_links"), self.email)
    // }

    // pub fn create(
    //     &self,
    //     user_id: String,
    //     expiration_minutes: Option<i32>,
    //     attributes: Option<HashMap<String, String>>,
    // ) -> Result<(), String> {
    //     let attributes = attributes.unwrap_or(HashMap::new());
    //     let data = serde_json::json!({
    //         "user_id": user_id,
    //         "attributes": attributes,
    //     });
    //     let data = serde_json::to_string(&data).unwrap();
    // }
}

#[derive(Debug, Clone)]
pub struct Email {
    pub email: String,
    pub base: Base,
    pub client: client::Client,
}

impl Email {
    pub fn new(email: String, base: &Base, client: client::Client) -> Self {
        Self {
            email,
            base: base.clone(),
            client: client.clone(),
        }
    }
    pub fn magic_link_url(&self) -> String {
        self.base.get_url("magic_links")
    }

    pub async fn login_or_create(
        &self,
        email: String,
        login_magic_link_url: Option<String>,
        signup_magic_link_url: Option<String>,
        login_expiration_minutes: Option<i32>,
        signup_expiration_minutes: Option<i32>,
        attributes: Option<HashMap<String, String>>,
        create_user_as_pending: Option<bool>,
    ) -> Result<(), String> {
        let attributes = attributes.unwrap_or(HashMap::new());
        let mut data = serde_json::json!({
            "email": email,
            "attributes": attributes,
            "create_user_as_pending": create_user_as_pending
        });
        if login_magic_link_url.is_some() {
            let data = data.as_object_mut().unwrap();
            data.insert(
                "login_magic_link_url".to_string(),
                serde_json::Value::String(login_magic_link_url.unwrap()),
            );
        }
        if signup_magic_link_url.is_some() {
            let data = data.as_object_mut().unwrap();
            data.insert(
                "signup_magic_link_url".to_string(),
                serde_json::Value::String(signup_magic_link_url.unwrap()),
            );
        }
        if login_expiration_minutes.is_some() {
            let data = data.as_object_mut().unwrap();
            data.insert(
                "login_expiration_minutes".to_string(),
                serde_json::Value::String(login_expiration_minutes.unwrap().to_string()),
            );
        }
        if signup_expiration_minutes.is_some() {
            let data = data.as_object_mut().unwrap();
            data.insert(
                "signup_expiration_minutes".to_string(),
                serde_json::Value::String(signup_expiration_minutes.unwrap().to_string()),
            );
        }
        let data = serde_json::to_string(&data).unwrap();
        let url = format!("{}/email/login_or_create", self.magic_link_url());
        let res = self.base.post(url, data).await;
        println!("{:?}", res);
        Ok(())
    }
}
