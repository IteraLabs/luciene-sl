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

    use std::{
        sync::Arc,
        str::FromStr
    };
    use crate::test_config::{AnchorConfig, get_config};
    use anchor_client::{
        solana_sdk::{
            pubkey::Pubkey, 
            signature::read_keypair_file,
            signer::Signer
        },
        Client
    };
  
    use luciene_sl::state::model_params::ModelParameters;

    #[test]
    fn test_fetch_model_params() {

        // Load helper struct
        let anchor_config: AnchorConfig = get_config();
        let payer = Arc::new(read_keypair_file(anchor_config.wallet).unwrap());
        let payer_pubkey = payer.pubkey();
        
        let client = Client::new(anchor_config.cluster, payer.clone());
        let pubkey = Pubkey::from_str(&anchor_config.program_id).unwrap();
        let program = client.program(pubkey).unwrap();

        println!(" testing account data fetching...");
        
        // derive pdas
        let (model_params_pda, _) = Pubkey::find_program_address(
            &[b"model_params", payer_pubkey.as_ref()],
            &program.id()
        );
        
        // fetch and verify model parameters
        match program.account::<ModelParameters>(model_params_pda) {

            Ok(model_params) => {

                println!(" successfully fetched model parameters");
                println!("  - weights: {:?}", model_params.weights);
                println!("  - bias: {}", model_params.bias);
                println!("  - authority: {}", model_params.authority);
                
                // verify data integrity
                assert_eq!(model_params.authority, payer_pubkey);
                assert!(model_params.weights.len() == 5);
            },

            Err(e) => {
                println!("failed to fetch model parameters: {}", e);
            }
        }
    }
}

