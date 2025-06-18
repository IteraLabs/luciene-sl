
#[cfg(test)]

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod test_utils; 

mod tests {

    use std::{sync::Arc, str::FromStr};
    use anchor_client::{
        Client, Cluster,
        solana_sdk::{
            pubkey::Pubkey, 
            signature::read_keypair_file,}
        };
    
    #[test]
    fn test_program_exists() -> Result<(), Box<dyn std::error::Error>> {
        
        use crate::test_utils::AnchorConfig;
        let test_config = AnchorConfig::new(
            Cluster::Localnet,
            "PROGRAM".to_string(),
            "WALLET".to_string(),
        );
        
        println!(" Testing if program exists on {:?} ...", test_config.cluster);
        
        // Load helper struct
        let anchor_config: AnchorConfig = test_config.get_config();
        let payer = Arc::new(read_keypair_file(anchor_config.wallet).unwrap());
        
        let client = Client::new(anchor_config.cluster, payer.clone());
        let pubkey = Pubkey::from_str(&anchor_config.program).unwrap();
        let program = client.program(pubkey).unwrap();

        // Try to get program account info
        let account_info = program.rpc().get_account(&program.id());
        
        match account_info {
            Ok(account) => {
                println!(" Program account found on {:?}", test_config.cluster);
                println!("  - Owner: {}", account.owner);
                println!("  - Executable: {}", account.executable);
                println!("  - Lamports: {}", account.lamports);
                assert!(account.executable, "Program account should be executable");
            },
            Err(e) => {
                return Err(format!("Error fetching program account: {}", e).into());
            }
        }
        Ok(())
    }
}

