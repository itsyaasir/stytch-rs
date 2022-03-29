use crate::{Base, OTPsEmailSendParams, OTPsEmailSendResponse, Stytch, StytchErrorTypes};

#[derive(Debug, Clone)]
pub struct Email<'a> {
    pub email: String,
    pub base: Base,
    pub client: &'a Stytch,
}

// Implementation
impl<'a> Email<'a> {
    // Send otp
    pub async fn send(
        &self,
        params: OTPsEmailSendParams,
    ) -> Result<OTPsEmailSendResponse, StytchErrorTypes> {
        let url = format!("{}/email/send", self.base.get_url("otps"));
        let data = serde_json::json!(
            {
                "email":self.email,
                "expiration_minutes":params.expiration_minutes,
                "attributes":self.base.get_attributes(params.attributes)
            }
        );
        let res = self.base.post(url, data.to_string()).await;
        //
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    let res = serde_json::from_str(&res.text().await.unwrap())
                        .expect("Could not parse OTPsEmailResponse");
                    Ok(res)
                } else {
                    let error = serde_json::from_str(&res.text().await.unwrap());
                    Err(StytchErrorTypes::StytchError(error.unwrap()))
                }
            }
            Err(e) => Err(StytchErrorTypes::ReqwestError(e)),
        }
    }
}
