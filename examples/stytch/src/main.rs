use std::io::Error;

use stytch_rs::model::client::{Environment, Stytch};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    std::env::set_var("RUST_LOG", "actix_web=info");

    // Extract the environment variables from .env file
    let project_id = dotenv::var("PROJECT_ID").unwrap();
    let secret = dotenv::var("SECRET").unwrap();

    // Client
    let client = Stytch::new(project_id, secret, Environment::Test);

    // let params = LoginOrCreateParams {
    //     login_magic_link_url: Some("http://localhost:3000/authenticate".to_string()),
    //     signup_magic_link_url: Some("http://localhost:3000/authenticate".to_string()),
    //     login_expiration_minutes: Some(10),
    //     signup_expiration_minutes: Some(10),
    //     attributes: None,
    //     create_user_as_pending: None,
    // };
    // println!("Client 1");
    // match client
    //     .magic_links()
    //     .email("goodwonder5@gmail.com".to_string())
    //     .login_or_create(params)
    //     .await
    // {
    //     Ok(_) => println!("Success"),
    //     Err(e) => println!("Error: {}", e),
    // }

    // let invite_params = InviteParams {
    //     invite_magic_link_url: Some("http://localhost:3000/authenticate".to_string()),
    //     invite_expiration_minutes: Some(10),
    //     attributes: None,
    //     first_name: Some("Yasir".to_string()),
    //     last_name: Some("Shariff".to_string()),
    //     middle_name: Some("Adan".to_string()),
    // };

    // println!("Client 2");

    // match client
    //     .magic_links()
    //     .email("goodwonder5@gmail.com".to_string())
    //     .invite(invite_params)
    //     .await
    // {
    //     Ok(_) => println!("Success"),
    //     Err(e) => println!("Error: {}", e),
    // }

    let res = client
        .magic_links()
        .email("goodwonder5@gmail.com".to_string())
        .revoke()
        .await?;
    println!("{:?}", res);
    Ok(())
}
