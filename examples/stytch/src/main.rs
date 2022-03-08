

use stytch_rs::model::{
    client::{Environment, Stytch},
    otp_model::OTPsSMSLoginOrCreateParams,
};

#[tokio::main]
async fn main() {
    // Extract the environment variables from .env file
    let project_id = dotenv::var("PROJECT_ID").unwrap();
    let secret = dotenv::var("SECRET").unwrap();

    // Client
    let client = Stytch::new(project_id, secret, Environment::Test);

    // OTP
    let params = OTPsSMSLoginOrCreateParams::new(Some(5), None, Some(true));

    match client
        .otp()
        .sms("+10000000000".into())
        .login_or_create(params)
        .await
    {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}
