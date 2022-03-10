use serde::{Deserialize, Serialize};

use crate::{
    api::base::Base,
    errors::{Error, StytchErrorTypes},
    model::{
        client::Stytch,
        otp_model::{
            OTPsSMSLoginOrCreateResponse, OTPsWhatsAppLoginOrCreateParams,
            OTPsWhatsAppLoginOrCreateResponse, OTPsWhatsAppSendParams, OTPsWhatsAppSendResponse,
        },
    },
};

#[derive(Debug)]
pub struct Whatsapp<'a> {
    pub phone_number: String,
    pub base: Base,
    pub client: &'a Stytch,
}

impl<'a> Whatsapp<'a> {
    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// use stytch_rs::api::otp::whatsapp::whatsapp::Whatsapp;
    ///
    /// let whatsapp = ;
    /// assert_eq!(whatsapp.send(params), );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn send(
        &self,
        params: OTPsWhatsAppSendParams,
    ) -> Result<OTPsWhatsAppSendResponse, StytchErrorTypes> {
        let url = format!("{}/whatsapp/send", self.base.get_url("otps"));

        let data = serde_json::json!({
            "phone_number": self.phone_number,
            "expiration_minutes": params.expiration_minutes,
            "attributes": self.base.get_attributes(params.attributes)
        }
        );

        let res = self.base.post(url, data.to_string()).await;
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    let res = serde_json::from_str::<OTPsWhatsAppSendResponse>(
                        &res.text().await.unwrap(),
                    )
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

    pub async fn login_or_create(
        &self,

        params: OTPsWhatsAppLoginOrCreateParams,
    ) -> Result<OTPsWhatsAppLoginOrCreateResponse, StytchErrorTypes> {
        let url = format!("{}/whatsapp/login_or_create", self.base.get_url("otps"));
        println!("{}", url);

        let data = serde_json::json!({
            "phone_number": self.phone_number,
            "expiration_minutes": params.expiration_minutes,
            "create_user_as_pending": params.create_user_as_pending,
            "attributes":  self.base.get_attributes(params.attributes),
        });
        let res = self.base.post(url, data.to_string()).await;
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    let res = serde_json::from_str::<OTPsWhatsAppLoginOrCreateResponse>(
                        &res.text().await.unwrap(),
                    )
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
}
