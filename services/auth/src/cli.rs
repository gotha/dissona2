//! CLI tool for generating JWT tokens
//!
//! Usage:
//!   disona-auth-cli generate --user-id <UUID> --email <EMAIL> --name <NAME>
//!   disona-auth-cli generate -u <UUID> -e <EMAIL> -n <NAME> --expires-in 365d
//!   disona-auth-cli verify <TOKEN>

use chrono::{Duration, Utc};
use clap::{Parser, Subcommand};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "disona-auth-cli")]
#[command(about = "Disona Auth CLI - Generate and verify JWT tokens")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new JWT token
    Generate {
        /// User ID (UUID)
        #[arg(short, long)]
        user_id: Uuid,

        /// User email
        #[arg(short, long)]
        email: String,

        /// User name
        #[arg(short, long)]
        name: String,

        /// Token expiration (e.g., "1h", "7d", "365d")
        #[arg(short = 'x', long, default_value = "30d")]
        expires_in: String,
    },

    /// Verify a JWT token
    Verify {
        /// The JWT token to verify
        token: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    name: String,
    iat: i64,
    exp: i64,
    jti: String,
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("Empty duration".to_string());
    }

    let (num_str, unit) = s.split_at(s.len() - 1);
    let num: i64 = num_str
        .parse()
        .map_err(|_| format!("Invalid number: {}", num_str))?;

    match unit {
        "s" => Ok(Duration::seconds(num)),
        "m" => Ok(Duration::minutes(num)),
        "h" => Ok(Duration::hours(num)),
        "d" => Ok(Duration::days(num)),
        _ => Err(format!("Unknown unit: {}. Use s, m, h, or d", unit)),
    }
}

fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| {
        eprintln!("Warning: JWT_SECRET not set, using default dev secret");
        "dev-secret-change-this-in-production-must-be-at-least-32-chars".to_string()
    })
}

fn main() {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            user_id,
            email,
            name,
            expires_in,
        } => {
            let secret = get_jwt_secret();
            let duration = parse_duration(&expires_in).expect("Invalid duration format");
            let now = Utc::now();
            let exp = now + duration;

            let claims = Claims {
                sub: user_id.to_string(),
                email: email.clone(),
                name: name.clone(),
                iat: now.timestamp(),
                exp: exp.timestamp(),
                jti: Uuid::new_v4().to_string(),
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            )
            .expect("Failed to generate token");

            eprintln!("Generated token for:");
            eprintln!("  User ID: {}", user_id);
            eprintln!("  Email:   {}", email);
            eprintln!("  Name:    {}", name);
            eprintln!("  Expires: {}", exp.format("%Y-%m-%d %H:%M:%S UTC"));
            eprintln!();
            println!("{}", token);
        }

        Commands::Verify { token } => {
            let secret = get_jwt_secret();

            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            ) {
                Ok(data) => {
                    println!("✓ Token is valid");
                    println!();
                    println!("Claims:");
                    println!("  User ID: {}", data.claims.sub);
                    println!("  Email:   {}", data.claims.email);
                    println!("  Name:    {}", data.claims.name);
                    println!(
                        "  Issued:  {}",
                        chrono::DateTime::from_timestamp(data.claims.iat, 0)
                            .map(|t| t.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                            .unwrap_or_else(|| "Invalid".to_string())
                    );
                    println!(
                        "  Expires: {}",
                        chrono::DateTime::from_timestamp(data.claims.exp, 0)
                            .map(|t| t.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                            .unwrap_or_else(|| "Invalid".to_string())
                    );
                }
                Err(e) => {
                    eprintln!("✗ Token is invalid: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
