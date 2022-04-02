use stytch::{Attributes, Environment, OTPsEmailSendParams, Stytch};

#[tokio::main]
async fn main() {
    // Fetch Project ID and Secret
    let project_id = dotenv::var("PROJECT_ID").unwrap();
    let secret = dotenv::var("SECRET").unwrap();

    // client
    let client = Stytch::new(project_id, secret, Environment::Test);
    // Attributes
    let attr = Attributes {
        ip_address: "".into(),
        user_agent: "".into(),
    };
    // Params
    let params = OTPsEmailSendParams::new(Some(5), None);

    // magic links
    // let res = client
    //     .magic_links()
    //     .email("test@test.com".into())
    //     .login_or_create(params)
    //     .await;
    let res = client
        .otp()
        .email("goodwonder5@gmail.com".into())
        .send(params)
        .await;

    match res {
        Ok(res) => println!("Response: {:?}", res),
        Err(e) => println!("Error: {}", e),
    }
}
