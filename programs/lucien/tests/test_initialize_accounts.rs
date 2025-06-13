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
    use anchor_client::solana_sdk::signature::Signer;
    use std::sync::Arc;
    use std::str::FromStr;
    use crate::test_config::{AnchorConfig, get_config};
    use anchor_lang::system_program;
    use anchor_client::
        solana_sdk::{
            commitment_config::CommitmentConfig,
            pubkey::Pubkey,
            signature::read_keypair_file,
        };

    #[test]
    fn test_initialize_accounts() -> Result<(), Box<dyn std::error::Error>> {

        println!("🧪 Testing account initialization... ");

        let anchor_config: AnchorConfig = get_config();
        let payer = Arc::new(read_keypair_file(anchor_config.wallet).unwrap());
        let payer_pubkey = payer.pubkey();
        
        let client = Client::new_with_options(
            anchor_config.cluster,
            &payer,
            CommitmentConfig::confirmed(),
        );
        
        let pubkey = Pubkey::from_str(&anchor_config.program_id).unwrap();
        let program = client.program(pubkey).unwrap();
        
        // Derive PDAs for all accounts
        let (model_params_pda, _) = Pubkey::find_program_address(
            &[b"model_params", payer_pubkey.as_ref()],
            &program.id()
        );
        
        let (model_results_pda, _) = Pubkey::find_program_address(
            &[b"model_results", payer_pubkey.as_ref()],
            &program.id()
        );
        
        let (model_features_pda, _) = Pubkey::find_program_address(
            &[b"model_features", payer_pubkey.as_ref()],
            &program.id()
        );
        
        // Test each initialization
        let accounts_to_test = vec![
            ("model_params", model_params_pda, "InitializeParams"),
            ("model_results", model_results_pda, "InitializeResults"),
            ("model_features", model_features_pda, "InitializeFeatures"),
        ];
        
        for (name, pda, instruction_name) in accounts_to_test {
            println!(" Testing {} initialization...", name);
            
            let account_exists = program.rpc()
                .get_account(&pda)
                .is_ok();
            
            if account_exists {
                println!(" {} account already exists", name);
                continue;
            }
            
            // Call appropriate initialization instruction
            let result = match instruction_name {

                "InitializeResults" => {
                    program
                        .request()
                        .accounts(lucien::accounts::InitializeResults {
                            model_results: model_results_pda,
                            model_params: model_params_pda,
                            authority: payer_pubkey,
                            system_program: system_program::ID,
                        })
                        .args(lucien::instruction::InitializeResults {})
                        .signer(&payer)
                        .send()
                },

                "InitializeFeatures" => {
                    program
                        .request()
                        .accounts(lucien::accounts::InitializeFeatures {
                            model_features: model_features_pda,
                            authority: payer_pubkey,
                            system_program: system_program::ID,
                        })
                        .args(lucien::instruction::InitializeFeatures {})
                        .signer(&payer)
                        .send()
                },

                "InitializeParams" => {
                    
                    // Initialize model with sample weights and bias
                    let weights: [f32; 5] = [0.1, 0.2, 0.3, 0.4, 0.5];
                    let bias: f32 = 1.0;

                    program
                        .request()
                        .accounts(lucien::accounts::InitializeParams {
                            model_params: model_params_pda,
                            authority: payer_pubkey,
                            system_program: system_program::ID,
                        })
                        .args(lucien::instruction::InitializeParams { weights, bias })
                        .signer(&payer)
                        .send()
                },

                _ => continue,

            };
            
            match result {
                Ok(signature) => {
                    println!(" {} initialization successful: {}", name, signature);
                },
                Err(e) => {
                    println!(" {} initialization failed: {}", name, e);
                    return Err(format!("Failed to initialize {}: {}", name, e).into());
                }
            }
        }
        
        Ok(())
    }
}
