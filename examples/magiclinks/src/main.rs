use stytch::Stytch;

#[tokio::main]
async fn main() {
    // Fetch Project ID and Secret
    let project_id = dotenv::var("PROJECT_ID").unwrap();
    let secret = dotenv::var("SECRET").unwrap();

    // client
    let client = Stytch::new(project_id, secret);
}
