use crate::api::{magiclink::magic_links::MagicLinks, otp::otp::Otp};

//  Stytch API Python client.

#[derive(Debug, Clone)]
pub struct Stytch {
    pub project_id: String,
    pub secret: String,
    pub environment: Environment,
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
        Self {
            project_id,
            secret,
            environment,
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
    /// `OTPs` methods are called from here
    pub fn otp(&self) -> Otp {
        Otp::new(self)
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
