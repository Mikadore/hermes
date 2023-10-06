pub use color_eyre::eyre as eyre;
pub use eyre::{WrapErr, Result};

use tracing::debug;
use poem_openapi::OpenApiService;
use poem::{Route, listener::{RustlsCertificate, RustlsConfig, TcpListener}, endpoint::StaticFilesEndpoint};

use std::path::PathBuf;

use config::TlsConfig;

mod config;
mod api;

async fn load_tls_config(key_path: PathBuf, cert_path: PathBuf) -> Result<RustlsConfig> {
    let key = tokio::fs::read(key_path).await?;
    let cert = tokio::fs::read(cert_path).await?;
    let cert = RustlsCertificate::new().key(key).cert(cert);
    Ok(RustlsConfig::new().fallback(cert))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    debug!("Initialized logger");

    let env = config::Environment::load().wrap_err("Failed to load config")?;
    debug!("Running in environment: {:#?}", env); 

    let api_service = OpenApiService::new(api::Api, "Hermes API", env!("CARGO_PKG_VERSION"));

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", StaticFilesEndpoint::new(env.conf.web_root).index_file("index.html").fallback_to_index());

    let listener = TcpListener::bind((env.conf.address, env.conf.port));

    match env.conf.tls {
        Some(TlsConfig { key_path, cert_path }) => {
            let rustls_config  = load_tls_config(key_path, cert_path).await?; 
            
        },
        None => {},
    }
    poem::Server::new(listener).run(app).await?;

    Ok(())
}