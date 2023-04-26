use std::collections::HashMap;
use std::future::Future;
use std::ops::Add;

use reqwest::{StatusCode, Url};
use reqwest::{Client, Response};
use serde::Deserialize;

use crate::core::config::OidcConfig;

#[derive(Debug, Deserialize)]
pub struct OidcResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Debug)]
pub enum LoginError {
    WrongCredentials,
    Generic,
}

#[derive(Clone)]
pub struct Oidc {
    url: Url,
    audience: String,
    client_id: String,
    client: Client,
}

impl Oidc {
    pub fn new(config: &OidcConfig) -> Self {
        let url = config.base_url.clone().add("/oauth/token");
        let client = Client::builder()
            .https_only(true)
            .build()
            .expect("Could not build client");

        Oidc {
            url: Url::parse(url.as_str()).expect("Configured base_url is not a valid URL"),
            audience: config.audience.clone(),
            client_id: config.client_id.clone(),
            client,
        }
    }

    pub async fn login(&self, username: String, password: String) -> Result<OidcResponse, LoginError> {
        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "password");
        form_data.insert("username", username.as_str());
        form_data.insert("password", password.as_str());
        form_data.insert("audience", self.audience.as_str());
        form_data.insert("client_id", self.client_id.as_str());
        form_data.insert("scope", "openid");

        let response = self.client
                           .post(self.url.clone())
                           .form(&form_data)
                           .send()
                           .await;

        if response.is_err() {
            return Err(LoginError::Generic);
        }

        let response = response.unwrap();

        match response.status() {
            StatusCode::OK => Oidc::handle_success(response).await,
            StatusCode::FORBIDDEN => Err(LoginError::WrongCredentials),
            StatusCode::UNAUTHORIZED => Err(LoginError::WrongCredentials),
            _ => Err(LoginError::Generic),
        }
    }

    async fn handle_success(response: Response) -> Result<OidcResponse, LoginError> {
        match response.json::<OidcResponse>().await {
            Ok(oidc_response) => Ok(oidc_response),
            Err(_) => Err(LoginError::Generic)
        }
    }
}

