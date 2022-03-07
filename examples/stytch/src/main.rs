use stytch_rs::model::{
    client::{Environment, Stytch},
    magic_link_model::{InviteParams, LoginOrCreateParams},
};

#[tokio::main]
async fn main() {
    // Load .env file
    std::env::set_var("RUST_LOG", "actix_web=info");

    // Extract the environment variables from .env file
    let project_id = dotenv::var("PROJECT_ID").unwrap();
    let secret = dotenv::var("SECRET").unwrap();

    // Client
    let client = Stytch::new(project_id, secret, Environment::Test);

    let params = LoginOrCreateParams {
        login_magic_link_url: Some("http://localhost:3000/authenticate".to_string()),
        signup_magic_link_url: Some("http://localhost:3000/authenticate".to_string()),
        login_expiration_minutes: Some(30),
        signup_expiration_minutes: Some(10),
        attributes: None,
        create_user_as_pending: None,
    };
    println!("Client 1");
    match client
        .magic_links()
        .email("goodwonder5@gmail.com".to_string())
        .login_or_create(params)
        .await
    {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }

    // // Get token key
    match client
        .magic_links()
        .authenticate("bvmkRgzO-WQM8hAswNWseGstD5A09jeu0gpRa91A_w_p".into())
        .await
    {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }

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

    // match client
    //     .magic_links()
    //     .email("goodwonder5@gmail.com".into())
    //     .revoke()
    //     .await
    // {
    //     Ok(res) => println!("Success: {:?}", res),
    //     Err(e) => println!("Error: {}", e),
    // }
}
