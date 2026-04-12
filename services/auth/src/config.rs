use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub jwt: JwtSettings,
    pub google: GoogleSettings,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct JwtSettings {
    pub secret: String,
    #[serde(default = "default_access_token_ttl")]
    pub access_token_ttl_seconds: u64,
    #[serde(default = "default_refresh_token_ttl")]
    pub refresh_token_ttl_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct GoogleSettings {
    pub client_id: String,
    pub client_secret: String,
    #[serde(default = "default_redirect_uri")]
    pub redirect_uri: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8081
}

fn default_max_connections() -> u32 {
    5
}

fn default_access_token_ttl() -> u64 {
    3600 // 1 hour
}

fn default_refresh_token_ttl() -> u64 {
    30 * 24 * 3600 // 30 days
}

fn default_redirect_uri() -> String {
    "http://localhost/auth/callback/google".to_string()
}

impl Settings {
    pub fn from_env() -> anyhow::Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .set_default("server.host", default_host())?
            .set_default("server.port", default_port())?
            .set_default("database.max_connections", default_max_connections())?
            .set_default("jwt.access_token_ttl_seconds", default_access_token_ttl())?
            .set_default("jwt.refresh_token_ttl_seconds", default_refresh_token_ttl())?
            .set_default("google.redirect_uri", default_redirect_uri())?
            .build()?;

        let settings: Settings = settings.try_deserialize()?;
        Ok(settings)
    }
}
