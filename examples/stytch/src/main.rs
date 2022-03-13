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
    let params = OTPsWhatsAppSendParams::new(Some(5), None);
    let phone_no = "+";

    match client
        .otp()
        .whatsapp("+254791559129".into())
        .send(params)
        .await
    {
        Ok(res) => println!("Response : {:?}", res),
        Err(e) => println!("Error {}", e),
    }
}
