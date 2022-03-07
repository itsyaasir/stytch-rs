use crate::{
    api::base::Base,
    errors::errors::{Error, StytchErrorTypes},
    model::{client::Stytch, magic_link_model::AuthenticateResponse},
};

use super::email::Email;

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

    pub async fn authenticate(&self, token: String) -> Result<String, StytchErrorTypes> {
        let base = Base::new(self.client);
        let url = format!("{}/authenticate", base.get_url("magic_links"));
        let data = serde_json::json!({
            "token": token,
        });
        let res = base.post(url, data.to_string()).await;
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    // let res =
                    //     serde_json::from_str::<AuthenticateResponse>(&res.text().await.unwrap())
                    //         .expect("Could not parse AuthenticateResponse");
                    Ok(res.text().await.unwrap())
                } else {
                    let error = serde_json::from_str::<Error>(&res.text().await.unwrap());
                    Err(StytchErrorTypes::StytchError(error.unwrap()))
                }
            }
            Err(e) => Err(StytchErrorTypes::ReqwestError(e)),
        }
    }
}
