use stytch::{Attributes, Environment, LoginOrCreateParams, Stytch};

#[tokio::main]
async fn main() {
    // Fetch Project ID and Secret
    let project_id = dotenv::var("PROJECT_ID").unwrap();
    let secret = dotenv::var("SECRET").unwrap();

    // client
    let client = Stytch::new(project_id, secret, Environment::Live);
    // Attributes
    let attr = Attributes {
        ip_address: "0.0.0.0".into(),
        user_agent: "user agent".into(),
    };
    // LoginOrCreateParams
    let params = LoginOrCreateParams::new(
        "login_magic_url_here".to_string(),
        "signup_magic_url_here".to_string(),
        Some(5),
        Some(5),
        Some(attr),
        Some(true),
    );

    // magic links
    let res = client
        .magic_links()
        .email("test@test.com".into())
        .login_or_create(params)
        .await;

    match res {
        Ok(res) => println!("Response: {:?}", res),
        Err(e) => println!("Error: {}", e),
    }
}
