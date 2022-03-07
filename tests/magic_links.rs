#[cfg(test)]
//  pub async fn revoke(&self) -> Result<reqwest::Response, crate::errors::StytchErrorTypes> {
//         let url = format!("{}/email/revoke_invite", self.magic_link_url());
//         let data = serde_json::json!({
//             "email": self.email,
//         });
//         let res = self.base.post(url, data.to_string()).await;
//         match res {
//             Ok(res) => {
//                 if res.status() == 200 {
//                     Ok(res)
//                 } else {
//                     let error = serde_json::from_str::<Error>(&res.text().await.unwrap());
//                     Err(StytchErrorTypes::StytchError(error.unwrap()))
//                 }
//             }
//             Err(e) => Err(StytchErrorTypes::ReqwestError(e)),
//         }
//     }
