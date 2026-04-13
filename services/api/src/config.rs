use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub nats: NatsSettings,
    pub s3: S3Settings,
    pub jwt: JwtSettings,
}

#[derive(Debug, Deserialize)]
pub struct JwtSettings {
    pub secret: String,
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
pub struct NatsSettings {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct S3Settings {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    #[serde(default = "default_bucket_uploads")]
    pub bucket_uploads: String,
    #[serde(default = "default_bucket_audio")]
    pub bucket_audio: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_max_connections() -> u32 {
    10
}

fn default_bucket_uploads() -> String {
    "dissona-uploads".to_string()
}

fn default_bucket_audio() -> String {
    "dissona-audio".to_string()
}

impl Settings {
    pub fn from_env() -> anyhow::Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .set_default("server.host", default_host())?
            .set_default("server.port", default_port())?
            .set_default("database.max_connections", default_max_connections())?
            .set_default("s3.bucket_uploads", default_bucket_uploads())?
            .set_default("s3.bucket_audio", default_bucket_audio())?
            .build()?;

        let settings: Settings = settings.try_deserialize()?;
        Ok(settings)
    }
}
