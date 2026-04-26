#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]

pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod nats;
pub mod s3;
pub mod telemetry;
