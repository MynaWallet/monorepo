use crate::error::{Error, Result};
use std::env;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
    static mut CONFIG: OnceLock<Config> = OnceLock::new();

    unsafe {
        CONFIG.get_or_init(|| {
            Config::load_from_env().unwrap_or_else(|e| {
                panic!("Failed to load config: {:?}", e);
            })
        })
    }
}

#[allow(non_snake_case)]
pub struct Config {
    pub PORT: u16,
    pub SIGNER_SECRET_KEY: String,
    pub MUMBAI_ENDPOINT: String,
    pub GOERLI_ENDPOINT: String,
    pub SEPOLIA_ENDPOINT: String,
    pub PAYMATER_CONTRACT_ADDRESS: String,
    pub NETWORK: String
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            PORT: get_env_u16("PORT")?,
            SIGNER_SECRET_KEY: get_env("SIGNER_SECRET_KEY")?,
            MUMBAI_ENDPOINT: get_env("MUMBAI_ENDPOINT")?,
            GOERLI_ENDPOINT: get_env("GOERLI_ENDPOINT")?,
            SEPOLIA_ENDPOINT: get_env("SEPOLIA_ENDPOINT")?,
            PAYMATER_CONTRACT_ADDRESS: get_env("PAYMATER_CONTRACT_ADDRESS")?,
            NETWORK: get_env("NETWORK")?
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|e| {
        println!("Failed to load env {}: {}", name, e);
        Error::ConfigMissingEnv(name)
    })
}

fn get_env_u16(name: &'static str) -> Result<u16> {
    get_env(name).and_then(|s| {
        s.parse().map_err(|e| {
            println!("Failed to parse {} as u16: {}", name, e);
            Error::ConfigParseError(name)
        })
    })
}
