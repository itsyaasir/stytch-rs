use stytch::{Environment, Stytch, LoginOrCreateParams, Attributes};

#[tokio::main]
async fn main() {
    // Fetch Project ID and Secret
    let project_id = dotenv::var("PROJECT_ID").unwrap();
    let secret = dotenv::var("SECRET").unwrap();

    // client
    let client = Stytch::new(project_id, secret, Environment::Live);
    // Attributes
    let attr = Attributes {
        ip_address: "0.0.0.0",
        user_agent:"user agent "
    }
    // LoginOrCreateParams
    let params = LoginOrCreateParams::new("login_magic_url_here","signup_magic_url_here","login_expiration_minutes", "signup_expiration_minutes",attr,
    "create_user_as_pending");

    // magic links
    let res = client
    .magic_links()
    .email("test@test.com")
    .login_or_create(params)
    .await

    match res {
        Ok(res) => println!("Response: {}",res)
        Err(e) => println!("Error: {}",e)
    }
}
