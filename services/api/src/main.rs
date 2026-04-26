use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_actix_web::TracingLogger;

use dissona_api::auth::JwtValidator;
use dissona_api::config::Settings;
use dissona_api::handlers;
use dissona_api::nats as app_nats;
use dissona_api::s3::StorageClient;
use dissona_api::telemetry;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let settings = Settings::from_env()?;

    // Initialize telemetry (logging, metrics, tracing)
    telemetry::init(&settings)?;

    info!("Starting Disona API Service");

    // Database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.url)
        .await?;

    info!("Connected to database");

    // Run migrations
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    info!("API database migrations applied");

    // NATS connection
    let nats_client = async_nats::connect(&settings.nats.url).await?;
    let nats_jetstream = async_nats::jetstream::new(nats_client);
    info!("Connected to NATS");

    // Initialize NATS JetStream streams
    app_nats::init_streams(&nats_jetstream).await.map_err(|e| anyhow::anyhow!("NATS stream init: {}", e))?;
    info!("NATS streams initialized");

    // S3 storage client
    let storage_client = StorageClient::new(&settings.s3).await;
    storage_client.ensure_buckets().await.map_err(|e| anyhow::anyhow!("S3 bucket init: {}", e))?;
    info!("S3 storage initialized");

    // JWT validator
    let jwt_validator = JwtValidator::new(&settings.jwt.secret);
    info!("JWT validator initialized");

    // Start HTTP server
    let server_addr = format!("{}:{}", settings.server.host, settings.server.port);
    info!("Starting server on {}", server_addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // TODO: Restrict in production
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(86400);

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(nats_jetstream.clone()))
            .app_data(web::Data::new(jwt_validator.clone()))
            .app_data(web::Data::new(storage_client.clone()))
            .configure(handlers::configure)
    })
    .bind(&server_addr)?
    .run()
    .await?;

    Ok(())
}
