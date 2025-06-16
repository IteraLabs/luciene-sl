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
    use crate::test_config::{ AnchorConfig, get_config };
    use anchor_lang::system_program;
    use anchor_client::
        solana_sdk::{
            pubkey::Pubkey,
            signature::read_keypair_file,
        };

    #[test]
    fn test_initialize_data_accounts() -> Result<(), Box<dyn std::error::Error>> {

        println!("🧪 Testing Data Accounts Initialization... ");

        // Load helper struct
        let anchor_config: AnchorConfig = get_config();
        let payer = Arc::new(read_keypair_file(anchor_config.wallet).unwrap());
        let payer_pubkey = payer.pubkey();
        
        let client = Client::new(anchor_config.cluster, payer.clone());
        let pubkey = Pubkey::from_str(&anchor_config.program_id).unwrap();
        let program = client.program(pubkey).unwrap();

        println!(" testing data prices account...");
        
        // derive pdas
        let (data_prices_pda, _) = Pubkey::find_program_address(
            &[b"data_prices", payer_pubkey.as_ref()],
            &program.id()
        );
        
        // Test each initialization
        let accounts_to_test = vec![
            ("data_prices", data_prices_pda, "InitializeDataPrices"),
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

                "InitializeDataPrices" => {

                program
                    .request()
                    .accounts(luciene_sl::accounts::InitializeDataPrices {
                        data_prices: data_prices_pda,
                        authority: payer_pubkey,
                        system_program: system_program::ID,
                    })
                    .args(luciene_sl::instruction::InitializeDataPrices {})
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
