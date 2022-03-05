use crate::model::client::Stytch;
use std::{collections::HashMap, str::FromStr};

// Const version number
const VERSION: &str = "0.1.0";
#[derive(Debug, Clone)]
pub struct Base {
    pub headers: reqwest::header::HeaderMap,
    pub client: Stytch,
    pub auth: HashMap<String, String>,
}

impl Base {
    pub fn new(client: &Stytch) -> Self {
        Self {
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers.insert(
                    "User-Agent".to_string(),
                    format!("Stytch Rust v{}", VERSION),
                );
                let mut header_map = reqwest::header::HeaderMap::new();
                for (key, value) in headers {
                    header_map.insert(
                        reqwest::header::HeaderName::from_str(&key).unwrap(),
                        reqwest::header::HeaderValue::from_str(&value).unwrap(),
                    );
                }
                header_map
            },
            client: client.clone(),
            auth: {
                let mut auth = HashMap::new();
                auth.insert(
                    "project_id".to_string(),
                    client.project_id.to_string().clone(),
                );
                auth.insert("secret".to_string(), client.secret.to_string().clone());
                auth
            },
        }
    }

    pub fn get_url(&self, arg: &str) -> String {
        format!("{}{}", self.client.environment.base_url(), arg)
    }

    pub fn validate_fields(&self, accepted_fields: &[&str], fields: &[&str]) -> bool {
        if accepted_fields.iter().cloned().collect::<Vec<&str>>().len()
            + fields.iter().cloned().collect::<Vec<&str>>().len()
            > accepted_fields.len()
        {
            panic!("Unknown arguments applied");
        }
        true
    }

    pub async fn post(&self, url: String, data: String) -> reqwest::Response {
        println!("URL : {}", url);
        let response = reqwest::Client::new()
            .post(url)
            .basic_auth(
                self.auth.get("project_id").unwrap(),
                Some(self.auth.get("secret").unwrap()),
            )
            .headers(self.headers.clone())
            .body(data)
            .send()
            .await
            .unwrap();
        response
    }
}
