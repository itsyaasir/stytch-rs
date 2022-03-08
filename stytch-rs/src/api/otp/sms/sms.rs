//  phone_number,
//             base: Base::new(&self.client),
//             client: &self.client,

use std::collections::HashMap;

use crate::model::magic_link_model::Attributes;
use crate::{
    api::base::Base,
    errors::{Error, StytchErrorTypes},
    model::{
        client::Stytch,
        otp_model::{
            OTPsSMSLoginOrCreateParams, OTPsSMSLoginOrCreateResponse, OTPsSMSSendParams,
            OTPsSMSSendResponse,
        },
    },
};

#[derive(Debug, Clone)]
pub struct Sms<'a> {
    pub phone_number: String,
    pub base: Base,
    pub client: &'a Stytch,
}

impl<'a> Sms<'a> {
    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// use stytch_rs::api::otp::sms::sms::Sms;
    ///
    /// let sms = ;
    /// assert_eq!(sms.send(params), );
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
        params: OTPsSMSSendParams,
    ) -> Result<OTPsSMSSendResponse, StytchErrorTypes> {
        let url = format!("{}/sms/send", self.base.get_url("otps"));
        println!("{}", url);

        let data = serde_json::json!({
            "phone_number": self.phone_number,
            "expiration_minutes": params.expiration_minutes,
            "attributes": self.base.get_attributes(params.attributes),
        });
        let res = self.base.post(url, data.to_string()).await;
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    let res =
                        serde_json::from_str::<OTPsSMSSendResponse>(&res.text().await.unwrap())
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

    /// .
    ///
    /// # Examples
    ///
    /// ```
    /// use stytch_rs::api::otp::sms::sms::Sms;
    ///
    /// let sms = ;
    /// assert_eq!(sms.login_or_create(params), );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub async fn login_or_create(
        &self,

        params: OTPsSMSLoginOrCreateParams,
    ) -> Result<OTPsSMSLoginOrCreateResponse, StytchErrorTypes> {
        let url = format!("{}/sms/login_or_create", self.base.get_url("otps"));
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
                    let res = serde_json::from_str::<OTPsSMSLoginOrCreateResponse>(
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
