use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use serde::Deserialize;

use crate::config::Settings;

pub fn create_google_client(settings: &Settings) -> anyhow::Result<BasicClient> {
    let client = BasicClient::new(
        ClientId::new(settings.google.client_id.clone()),
        Some(ClientSecret::new(settings.google.client_secret.clone())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?,
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?),
    )
    .set_redirect_uri(RedirectUrl::new(settings.google.redirect_uri.clone())?);

    Ok(client)
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

pub async fn get_google_user_info(access_token: &str) -> Result<GoogleUserInfo, reqwest::Error> {
    let client = reqwest::Client::new();
    
    client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<GoogleUserInfo>()
        .await
}
