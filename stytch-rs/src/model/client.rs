//  Stytch API Python client.
use crate::api::magic_links::MagicLinks;

#[derive(Debug, Clone)]
pub struct Client {
    pub project_id: String,
    pub secret: String,
    pub environment: Environment,
}

// Make the client available to the rest of the program.

// Implement the `Client` struct. and add MagicLinks
impl Client {
    pub fn new(project_id: String, secret: String, environment: Environment) -> Self {
        Self {
            project_id,
            secret,
            environment,
        }
    }

    // pub fn magic_links(&self, email: String) -> MagicLinks {
    //     MagicLinks { email }
    // }
}

// Two types of environment
#[derive(Debug, Clone)]
pub enum Environment {
    Test,
    Live,
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
            Environment::Test => "test",
            Environment::Live => "live",
        }
    }

    pub fn base_url(&self) -> String {
        match self {
            Environment::Test => "https://test.stytch.com/v1/".to_string(),
            Environment::Live => "https://api.stytch.com/v1/".to_string(),
        }
    }
}
