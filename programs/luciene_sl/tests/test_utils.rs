
// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

use anchor_client::Cluster;
use std::{ env, path::{Path, PathBuf } };

pub enum NetworkConfig {

    Localnet,
    Devnet,
    Mainnet,

}

#[derive(Debug, Clone)]
pub struct AnchorConfig {

    pub cluster: Cluster,
    pub program: String,
    pub wallet: String,

}

impl AnchorConfig {

    pub fn new(
        cluster: Cluster,
        program: String,
        wallet: String,
    ) -> Self {
        AnchorConfig { cluster, program, wallet }
    }

}

impl AnchorConfig {

    pub fn expand_tilde<P: AsRef<str>>(&self, path: P) -> PathBuf {
        
        let path_str = path.as_ref();

        if let Some(stripped) = path_str.strip_prefix("~/") {
            if let Ok(home) = env::var("HOME") {
                return PathBuf::from(home).join(stripped);
            }
        }

        PathBuf::from(path_str)

    }

    pub fn get_config(&self) -> Self {
        
        let wallet_dir = env!("WALLET");
        let wallet_root = Path::new(wallet_dir)
            .parent()
            .expect("Failed to get wallet local dir");

        let wallet_file = self.expand_tilde(
                wallet_root
                .join("solana")
                .join("id.json")
                .to_str()
                .unwrap()
            );
     
        AnchorConfig {
            cluster: Cluster::Localnet,
            program: std::env::var(self.program.to_string()).unwrap(),
            wallet: wallet_file.to_str().unwrap().to_string(),
        }

    }

}


