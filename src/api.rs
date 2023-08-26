use reqwest;
use reqwest::{header, Client, Error, Response};

use super::cli::CliArgs;

pub struct ApiHandler {
    client: Client,
}

impl ApiHandler {
    /// Generate the headers for the API client.
    /// Specifically, it attaches the Authorization Bearer token to the headers.
    fn generate_headers(token: &str) -> Result<header::HeaderMap, header::InvalidHeaderValue> {
        let mut headers = header::HeaderMap::new();
        let auth_value: String = format!("Bearer {}", token);
        let mut header_value = header::HeaderValue::from_str(&auth_value)?;
        header_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, header_value);
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

/// Check if the repository exists on GitHub.
pub async fn repo_exists_on_github(
    api_handler: &ApiHandler,
    username: &str,
    repository_name: &str,
) -> Result<bool, reqwest::Error> {
    let url = format!("https://api.github.com/repos/{username}/{repository_name}");
    let res = api_handler.get(&url).await?;

    Ok(res.status() != 404)
}

pub async fn create_new_repository_on_github(
    api_handler: &ApiHandler,
    cli_args: &CliArgs,
) -> Result<(), reqwest::Error> {
    let url = format!("https://api.github.com/user/repos");
    let body = format!(
        r#"
        {{
            "name": "{repository_name}",
            "description": "{description}",
            "private": {private}
        }}
        "#,
        repository_name = cli_args.name,
        description = cli_args.description,
        private = cli_args.private
    );
    let res = api_handler.post(&url, &body).await?;
    if res.status() == 201 {
        println!("Repository created successfully on GitHub");
    } else {
        eprintln!("Failed to create repository on GitHub");
        std::process::exit(1);
    }

    Ok(())
}
