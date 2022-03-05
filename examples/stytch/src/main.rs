use actix_web::{dev::Server, web, App, HttpServer};
use stytch_rs::{
    api::{base::Base, magic_links::Email},
    model::client::{Client, Environment},
};
#[actix_web::main]
async fn main() {
    let project_id = "".to_string();
    let secret = "".to_string();

    let client = Client::new(project_id, secret, Environment::Test);

    let base = Base::new(&client);

    let email = Email::new("goodwonder5@gmail.com".to_owned(), &base, client);

    match email
        .login_or_create(
            "goodwonder5@gmail.com".to_string(),
            Some("http://localhost:3000/authenticate".to_string()),
            Some("http://localhost:3000/authenticate".to_string()),
            None,
            None,
            None,
            Some(true),
        )
        .await
    {
        Ok(_) => {
            println!("ok");
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}
