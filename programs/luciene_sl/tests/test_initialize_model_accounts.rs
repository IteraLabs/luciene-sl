
#[cfg(test)]

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod test_utils; 

mod tests {

    use std::{sync::Arc, str::FromStr};
    use anchor_lang::system_program;
    use anchor_client::{
        Client, Cluster,
        solana_sdk::{
            signature::Signer,
            signature::read_keypair_file,
            pubkey::Pubkey,}
        };

    #[test]
    fn test_initialize_model_accounts() -> Result<(), Box<dyn std::error::Error>> {

        println!("🧪 Testing Model Account Initialization... ");

        use crate::test_utils::AnchorConfig;
        let test_config = AnchorConfig::new(
            Cluster::Localnet,
            "PROGRAM".to_string(),
            "WALLET".to_string(),
        );
        
        // Load helper struct
        let anchor_config: AnchorConfig = test_config.get_config();
        let keypair_file = read_keypair_file(&anchor_config.wallet).unwrap();

        let payer = Arc::new(keypair_file);
        let payer_pubkey = payer.pubkey();

        println!("payer_pubkey: {:?}", payer_pubkey);

        let client = Client::new(anchor_config.cluster, payer.clone());
        let pubkey = Pubkey::from_str(&anchor_config.program).unwrap();
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
                        .accounts(luciene_sl::accounts::InitializeResults {
                            model_results: model_results_pda,
                            model_params: model_params_pda,
                            authority: payer_pubkey,
                            system_program: system_program::ID,
                        })
                        .args(luciene_sl::instruction::InitializeResults {})
                        .signer(&payer)
                        .send()
                },

                "InitializeFeatures" => {
                    program
                        .request()
                        .accounts(luciene_sl::accounts::InitializeFeatures {
                            model_features: model_features_pda,
                            authority: payer_pubkey,
                            system_program: system_program::ID,
                        })
                        .args(luciene_sl::instruction::InitializeFeatures {})
                        .signer(&payer)
                        .send()
                },

                "InitializeParams" => {
                    
                    // Initialize model with sample weights and bias
                    let weights: [f32; 5] = [0.1, 0.2, 0.3, 0.4, 0.5];
                    let bias: f32 = 1.0;

                    program
                        .request()
                        .accounts(luciene_sl::accounts::InitializeParams {
                            model_params: model_params_pda,
                            authority: payer_pubkey,
                            system_program: system_program::ID,
                        })
                        .args(luciene_sl::instruction::InitializeParams { weights, bias })
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
