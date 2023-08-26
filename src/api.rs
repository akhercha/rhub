use reqwest;
use reqwest::{header, Client, Error, Response};

pub struct ApiHandler {
    client: Client,
}

impl ApiHandler {
    /// Generate the headers for the API client.
    /// Specifically, it attaches the Authorization Bearer token to the headers.
    fn generate_headers(token: &str) -> Result<header::HeaderMap, header::InvalidHeaderValue> {
        let mut headers = header::HeaderMap::new();
        let auth_value: String = format!("Bearer {}", token);
        let mut auth_value = header::HeaderValue::from_str(&auth_value)?;
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        Ok(headers)
    }

    pub fn new(user_agent: &str, token: &str) -> Result<Self, header::InvalidHeaderValue> {
        let headers = Self::generate_headers(token)?;
        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()
            .expect("Failed to build the API client");
        Ok(ApiHandler { client })
    }

    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        let response = self.client.get(url).send().await?;
        Ok(response)
    }

    pub async fn post(&self, url: &str, body: &str) -> Result<Response, Error> {
        let response = self
            .client
            .post(url.to_string())
            .body(body.to_string())
            .send()
            .await?;
        Ok(response)
    }
}
