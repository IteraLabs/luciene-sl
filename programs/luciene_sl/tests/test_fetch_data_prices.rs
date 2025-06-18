#[cfg(test)]

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod test_utils; 

mod tests {

    use std::{sync::Arc, str::FromStr};
    use anchor_client::{
        Client, Cluster,
        solana_sdk::{
            signature::Signer,
            pubkey::Pubkey,
            signature::read_keypair_file,}
        };

    use luciene_sl::state::data_prices::DataPrices;

    #[test]
    fn test_fetch_data_prices() {

        println!("🧪 Testing Fetch Data Prices ... ");
        
        use crate::test_utils::AnchorConfig;
        let test_config = AnchorConfig::new(
            Cluster::Localnet,
            "PROGRAM".to_string(),
            "WALLET".to_string(),
        );

        // Load helper struct
        let anchor_config: AnchorConfig = test_config.get_config();
        let payer = Arc::new(read_keypair_file(anchor_config.wallet).unwrap());
        let payer_pubkey = payer.pubkey();
        let client = Client::new(anchor_config.cluster, payer.clone());
        let pubkey = Pubkey::from_str(&anchor_config.program).unwrap();
        let program = client.program(pubkey).unwrap();

        println!(" testing account data prices fetching...");
        
        // derive pdas
        let (data_prices_pda, _) = Pubkey::find_program_address(
            &[b"data_prices", payer_pubkey.as_ref()],
            &program.id()
        );
        
        // fetch and verify model parameters
        match program.account::<DataPrices>(data_prices_pda) {

            Ok(data_prices) => {

                println!(" successfully fetched data prices");
                println!("  - last_updated: {:?}", data_prices.last_updated);
                println!("  - current_index: {:?}", data_prices.current_index);
                println!("  - prices: {:?}", data_prices.prices);
                println!("  - timestamps: {:?}", data_prices.timestamps);
                println!("  - is_full: {:?}", data_prices.is_full);
                println!("  - authority: {:?}", data_prices.authority);
                
                // verify data integrity
                assert_eq!(data_prices.authority, payer_pubkey);
                assert!(data_prices.prices.len() == 10);
            },

            Err(e) => {
                println!("failed to fetch model parameters: {}", e);
            }
        }
    }
}

