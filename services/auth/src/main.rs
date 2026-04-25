use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_actix_web::TracingLogger;

use dissona_auth::config::Settings;
use dissona_auth::handlers;
use dissona_auth::jwt::JwtConfig;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let settings = Settings::from_env()?;

    // Initialize logging
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("Starting Disona Auth Service");

    // Database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.url)
        .await?;

    info!("Connected to Auth database");

    // Run migrations
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    info!("Auth database migrations applied");

    // OAuth client
    let oauth_client = dissona_auth::oauth::create_google_client(&settings)?;

    // JWT config
    let jwt_config = JwtConfig::new(&settings.jwt.secret);

    // Start HTTP server
    let server_addr = format!("{}:{}", settings.server.host, settings.server.port);
    info!("Starting server on {}", server_addr);

    // Clone settings for use in closure
    let settings_data = web::Data::new(settings.clone());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(86400);

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(oauth_client.clone()))
            .app_data(web::Data::new(jwt_config.clone()))
            .app_data(settings_data.clone())
            .configure(handlers::configure)
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
