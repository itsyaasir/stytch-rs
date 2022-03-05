use reqwest::Client;

//  Stytch API Python client.
use crate::api::magic_links::MagicLinks;

#[derive(Debug, Clone)]
pub struct Stytch {
    pub project_id: String,
    pub secret: String,
    pub environment: Environment,
    pub http_client: Client,
}

// Make the client available to the rest of the program.

// Implement the `Client` struct. and add MagicLinks
impl Stytch {
    /// Constructs a new  `Stytch` instance.
    ///
    /// # Example
    /// ```ignore
    /// let client: Stytch = Stytch::new(
    ///     env::var("PROJECT_ID").unwrap(),
    ///     env::var("SECRET").unwrap(),
    ///     Environment::Test,
    /// );
    /// ```
    pub fn new(project_id: String, secret: String, environment: Environment) -> Self {
        let http_client = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(1000))
            .build()
            .expect("Failed to build HTTP client");
        Self {
            project_id,
            secret,
            environment,
            http_client,
        }
    }

    /// Gets the current `Environment`.
    pub fn environment(&self) -> &Environment {
        &self.environment
    }
    /// `MagicLinks` methods are called here.
    // #[cfg(feature = "magic-links")]
    pub fn magic_links(&self) -> MagicLinks {
        MagicLinks::new(self)
    }
}

// Two types of environment
#[derive(Debug, Clone)]
/// The `Environment` enum.
/// Provides the `Environment` for the `Stytch` instance.
/// The `Environment` determines the API endpoint.
/// The `Environment` is set in the `Stytch::new` method.
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
