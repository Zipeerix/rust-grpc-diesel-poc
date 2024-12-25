use crate::config::load_configuration;
use crate::grpc_server::start_grpc_server;
use crate::metrics_server::start_metrics_server;
use clap::Parser;
use dotenvy::dotenv;
use log::{error, info};
use std::error::Error;
use std::sync::Arc;

mod account_service;
mod config;
mod database;
mod grpc_server;
mod metrics_server;
mod order_service;
mod schema;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Proof of concept - Rust gRPC application"
)]
struct CommandLineArguments {
    /// Path to configuration file
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = CommandLineArguments::parse();
    let config = Arc::new(load_configuration(&args.config));
    let metrics_config = Arc::clone(&config);

    let grpc_handle = tokio::spawn(async move {
        if let Err(e) = start_grpc_server(&config).await {
            error!("gRPC server encountered an error: {:?}", e);
        }
    });

    let metrics_handle = tokio::spawn(async move {
        if let Err(e) = start_metrics_server(&metrics_config).await {
            error!("Metrics server encountered an error: {:?}", e);
        }
    });

    tokio::select! {
        _ = grpc_handle => error!("gRPC server terminated"),
        _ = metrics_handle => error!("Metrics server terminated"),
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down...");
    }}

    Ok(())
}
