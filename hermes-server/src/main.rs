use eyre::{WrapErr, Result};
use tracing::debug;
use poem_openapi::OpenApiService;
use poem::{Route, listener::{Listener, RustlsCertificate, RustlsConfig, TcpListener}, endpoint::StaticFilesEndpoint};

use std::path::PathBuf;

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
    debug!("Initialized tracing_subscriber");
    let conf = config::Config::load().wrap_err("Failed to load config")?;
    debug!("Running with config: {:?}", conf);

    let api_service = OpenApiService::new(api::Api, "Add", "0.1");

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", StaticFilesEndpoint::new(conf.root).index_file("index.html").fallback_to_index());

    let listener = TcpListener::bind(("localhost", conf.port))
        .rustls(load_tls_config(conf.key_path, conf.cert_path).await?);

    poem::Server::new(listener).run(app).await?;

    Ok(())
}