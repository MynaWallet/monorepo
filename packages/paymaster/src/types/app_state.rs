use crate::config::Config;
use crate::utils::{load_provider, load_signer};
use ethers::providers::{Http, Provider};

pub struct AppState {
    pub signer: ethers::signers::LocalWallet,
    pub provider: Provider<Http>,
}

impl AppState {
    pub fn load_from_config(config: &Config) -> Self {
        Self {
            signer: load_signer(config),
            provider: load_provider(config, config.NETWORK.as_str()),
        }
    }
}
