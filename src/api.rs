use super::cli::CliArgs;
use reqwest::{header, Client, Error, Response, StatusCode};
use serde_json::Value;

const USER_AGENT: &str = "Rhub-CLI/0.1.0";

struct ApiHandler {
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

    pub fn new(token: &str) -> Result<Self, header::InvalidHeaderValue> {
        let headers = Self::generate_headers(token)?;
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
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

pub struct GithubApi {
    handler: ApiHandler,
}

impl GithubApi {
    pub fn new(token: &str) -> Result<Self, header::InvalidHeaderValue> {
        let handler = ApiHandler::new(token)?;
        Ok(GithubApi { handler })
    }

    pub async fn repo_exists(&self, username: &str, repository_name: &str) -> Result<bool, Error> {
        let url = format!("https://api.github.com/repos/{username}/{repository_name}");
        let res = self.handler.get(&url).await?;
        match res.status() {
            StatusCode::OK => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            StatusCode::UNAUTHORIZED => {
                panic!("UNAUTHORIZED: The provided github_pat_token is probably invalid.")
            }
            _ => panic!("Error: {}", res.status()),
        }
    }

    pub async fn create_new_repository(&self, cli_args: &CliArgs) -> Result<(), Error> {
        let url = "https://api.github.com/user/repos".to_string();
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
        let res = self.handler.post(&url, &body).await?;
        if res.status() == 201 {
            println!("Repository created successfully on GitHub");
        } else {
            eprintln!("Failed to create repository on GitHub");
            std::process::exit(1);
        }
        Ok(())
    }

    pub async fn get_username(&self) -> Result<String, Error> {
        let url = "https://api.github.com/user".to_string();
        let res = self.handler.get(&url).await?;
        match res.status() {
            StatusCode::OK => {
                let body = res.text().await?;
                let json_body: Value =
                    serde_json::from_str(&body).expect("Failed to parse JSON body");
                assert!(json_body["login"].is_string());
                Ok(json_body["login"].as_str().unwrap().to_string())
            }
            StatusCode::UNAUTHORIZED => {
                panic!("UNAUTHORIZED: The provided github_pat_token is probably invalid.")
            }
            _ => panic!("Error: {}", res.status()),
        }
    }
}
