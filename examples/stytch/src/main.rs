use stytch_rs::{
    errors::StytchErrorTypes,
    model::{
        client::{Environment, Stytch},
        otp_model::{OTPsSMSLoginOrCreateParams, OTPsSMSSendParams, OTPsWhatsAppSendParams},
    },
};

#[tokio::main]
async fn main() {
    // Extract the environment variables from .env file
    let project_id = dotenv::var("PROJECT_ID").unwrap();
    let secret = dotenv::var("SECRET").unwrap();

    // Client
    let client = Stytch::new(project_id, secret, Environment::Test);

    // OTP
    let params = OTPsSMSSendParams::new(5, None);
    let phone_no = "+100000000";

    match client.otp().sms(phone_no.into()).send(params).await {
        Ok(res) => println!("Response : {:?}", res),
        Err(e) => println!("Error {}", e),
    }
}
