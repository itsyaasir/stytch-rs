use thiserror::Error;

#[derive(Debug, Error)]
pub enum StytchErrorTypes {
    #[error("Client Library Error")]
    ClientLibraryError,
    #[error("Stytch Error: {0}")]
    StytchError(Error),
    #[error("Reqwest Error: {0}")]
    ReqwestError(reqwest::Error),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Error {
    pub status_code: i32,
    pub request_id: String,
    pub error_type: String,
    pub error_message: String,
    pub error_url: String,
}

// Implement Display for Error
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let error = serde_json::to_string(self).unwrap();
        write!(f, "{}", error)
    }
}


impl Error {
    pub fn new(
        status_code: i32,
        request_id: String,
        error_type: String,
        error_message: String,
        error_url: String,
    ) -> Error {
        Error {
            status_code,
            request_id,
            error_type,
            error_message,
            error_url,
        }
    }

    pub async fn from_response(response: reqwest::Response) -> Self {
        let body = response.text().await.unwrap();
        let error: Error = serde_json::from_str(&body).unwrap();
        error
    }

    pub fn status_code(&self) -> i32 {
        self.status_code
    }

    pub fn request_id(&self) -> String {
        self.request_id.clone()
    }

    pub fn error_type(&self) -> String {
        self.error_type.clone()
    }

    pub fn error_message(&self) -> String {
        self.error_message.clone()
    }

    pub fn error_url(&self) -> String {
        self.error_url.clone()
    }
}
