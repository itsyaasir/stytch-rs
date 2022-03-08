use std::collections::HashMap;

use crate::{
    api::base::Base,
    errors::errors::{Error, StytchErrorTypes},
    model::{
        client::Stytch,
        magic_link_model::{
            AuthenticateResponse, MagicLinksCreateParams, MagicLinksCreateResponse,
        },
    },
};

use super::email::Email;

// Macros
// #[macro_export]
// macro_rules! create_attribute {
//     ($key:expr, $value:expr) => {
//         if $value.is_some() {
//             let mut map = HashMap::new();
//             map.insert($key, $value.unwrap());
//             attributes.push(map);
//         }
//     };
// }
#[derive(Debug, Clone)]
pub struct MagicLinks<'a> {
    client: &'a Stytch,
}

impl<'a> MagicLinks<'a> {
    pub fn new(client: &'a Stytch) -> Self {
        Self { client }
    }

    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// use stytch_rs::api::magiclink::magic_links::MagicLinks;
    ///
    /// let magic_links = ;
    /// assert_eq!(magic_links.email(email), );
    /// ```
    pub fn email(&self, email: String) -> Email {
        Email {
            email,
            base: Base::new(self.client),
            client: self.client,
        }
    }

    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// use stytch_rs::api::magiclink::magic_links::MagicLinks;
    ///
    /// let magic_links = ;
    /// assert_eq!(magic_links.authenticate(token), );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn authenticate(
        &self,
        token: String,
    ) -> Result<AuthenticateResponse, StytchErrorTypes> {
        let base = Base::new(self.client);
        let url = format!("{}/authenticate", base.get_url("magic_links"));
        let data = serde_json::json!({
            "token": token,
        });
        let res = base.post(url, data.to_string()).await;
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    let res =
                        serde_json::from_str::<AuthenticateResponse>(&res.text().await.unwrap())
                            .expect("Could not parse AuthenticateResponse");
                    Ok(res)
                } else {
                    let error = serde_json::from_str::<Error>(&res.text().await.unwrap());
                    Err(StytchErrorTypes::StytchError(error.unwrap()))
                }
            }
            Err(e) => Err(StytchErrorTypes::ReqwestError(e)),
        }
    }

    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// use stytch_rs::api::magiclink::magic_links::MagicLinks;
    ///
    /// assert_eq!(MagicLinks::create(params), );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    // create
    pub async fn create(
        &self,
        params: MagicLinksCreateParams,
    ) -> Result<MagicLinksCreateResponse, StytchErrorTypes> {
        let base = Base::new(self.client);
        let url = format!("{}/", base.get_url("magic_links"));

        let data = serde_json::json!({
            "user_id": params.user_id,
            "expiration_minutes": params.expiration_minutes,
            "attributes": base.get_attributes(params.attributes),
        });
        let res = base.post(url, data.to_string()).await;
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    let res = serde_json::from_str::<MagicLinksCreateResponse>(
                        &res.text().await.unwrap(),
                    )
                    .expect("Could not parse MagicLinksCreateResponse");
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
