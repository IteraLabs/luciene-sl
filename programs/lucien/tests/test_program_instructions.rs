
#[cfg(test)]

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod test_config {

    use anchor_client::Cluster;

    pub struct AnchorConfig {
        pub cluster: Cluster,
        pub program_id: String,
        pub wallet: String,
    }

    impl AnchorConfig {
        pub fn new(
            cluster: Cluster,
            program_id: String,
            wallet: String,
        ) -> Self {
            AnchorConfig { cluster, program_id, wallet }
        }
    }

    pub fn get_config() -> AnchorConfig {

        let program_id = std::env::var("PROGRAM_ID").unwrap();
        let wallet = std::env::var("ANCHOR_WALLET").unwrap();
        let cluster = Cluster::Devnet;
        
        AnchorConfig::new(cluster, program_id, wallet)
        
        }
}

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod tests {

    use anchor_client::Client;
    use std::rc::Rc;
    use std::str::FromStr;
    use crate::test_config::{AnchorConfig, get_config};
    use anchor_client::
        solana_sdk::{
            pubkey::Pubkey, 
            signature::read_keypair_file,
        };

    #[test]
    fn test_program_instructions() {

        let anchor_config: AnchorConfig = get_config();
        let payer = Rc::new(read_keypair_file(anchor_config.wallet).unwrap());
        let client = Client::new(anchor_config.cluster, payer.clone());
        let pubkey = Pubkey::from_str(&anchor_config.program_id).unwrap();
        let _program = client.program(pubkey).unwrap();

        // Test calculate_features instruction (WIP)
        // Test run_inference instruction (WIP)

    }

}

