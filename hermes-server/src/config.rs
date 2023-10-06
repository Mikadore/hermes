use std::path::PathBuf;

use crate::eyre::{WrapErr, Result};
use clap::Parser;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigFile {
    pub web_root: PathBuf,
    pub port: u16,
    pub address: String,
    pub tls: Option<TlsConfig>,
}

#[derive(Deserialize, Debug)]
pub struct TlsConfig {
    pub key_path: PathBuf,
    pub cert_path: PathBuf,
}

#[derive(Debug)]
pub struct Environment {
    pub conf: ConfigFile,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long="config", short='c')]
    conf_file: Option<PathBuf>,
}

impl Environment {nq
    pub fn load() -> Result<Self> {
        let cli = Cli::parse();
        let config_file = if let Some(path) = cli.conf_file {
            path
        } else {
            "hermes.toml".into()
        };
        let content = std::fs::read_to_string(config_file).wrap_err("Failed to read config file")?;
        let conf = toml::from_str(&content).wrap_err("Failed to parse config file")?;
        Ok(Self { conf })
    }
}
