use std::path::PathBuf;

use eyre::{WrapErr, Result};

#[derive(Debug)]
pub struct Config {
    pub key_path: PathBuf,
    pub cert_path: PathBuf,
    pub port: u16,
    pub root: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        fn get_var(name: &str) -> Result<String> {
            std::env::var(name).wrap_err_with(|| format!("Failed to get required environment variable '{name}'"))
        }
        let key_path = get_var("HERMES_KEY_PATH")?.into();
        let cert_path = get_var("HERMES_CERT_PATH")?.into();
        let root = get_var("HERMES_ROOT_PATH")?.into();
        let port = get_var("PORT")?.parse()?;

        Ok(Self { key_path, cert_path, root, port })
    }
}