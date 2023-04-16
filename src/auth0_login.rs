use std::collections::HashMap;
use std::error::Error;

use reqwest::{StatusCode, Url};
use reqwest::blocking::{Client, Request, RequestBuilder, Response};
use serde::Deserialize;

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

// TODO: move to config file
static URL: &'static str = "https://dark-triad.eu.auth0.com/oauth/token";
static AUDIENCE: &'static str = "https://dark-triad.com";
static CLIENT_ID: &'static str = "tbUwI28fzIYTwS8KQLeZ9FqNXHLRZnZk";

pub fn login(username: String, password: String) -> Result<OidcResponse, LoginError> {
    let parsed_url = Url::parse(URL).unwrap();
    let mut form_data = HashMap::new();
    form_data.insert("grant_type", "password");
    form_data.insert("username", username.as_str());
    form_data.insert("password", password.as_str());
    form_data.insert("audience", AUDIENCE);
    form_data.insert("client_id", CLIENT_ID);
    form_data.insert("scope", "openid");

    let client = Client::builder()
        .https_only(true)
        .build()
        .unwrap();

    let result = client
        .post(parsed_url)
        .form(&form_data)
        .send();

    if result.is_err() {
        return Err(LoginError::Generic);
    }

    let response = result.unwrap();

    match response.status() {
        StatusCode::OK => handle_success(response),
        StatusCode::FORBIDDEN => Err(LoginError::WrongCredentials),
        StatusCode::UNAUTHORIZED => Err(LoginError::WrongCredentials),
        _ => Err(LoginError::Generic),
    }
}

fn handle_success(response: Response) -> Result<OidcResponse, LoginError> {
    match response.json::<OidcResponse>() {
        Ok(oidc_response) => Ok(oidc_response),
        Err(_) => Err(LoginError::Generic)
    }
}
