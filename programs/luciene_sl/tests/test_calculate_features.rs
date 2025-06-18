
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

    use luciene_sl::state::{model_features::ModelFeatures, data_prices::DataPrices};

    #[test]
    fn test_calculate_features() {
        
        // Load configuration from existing setup
        use crate::test_utils::AnchorConfig;
        let test_config = AnchorConfig::new(
            Cluster::Localnet,
            "PROGRAM".to_string(),
            "WALLET".to_string(),
        );

        println!("🧪 Testing Calculate Features ... ");
        
        // Load helper struct
        let anchor_config: AnchorConfig = test_config.get_config();
        let payer = Arc::new(read_keypair_file(anchor_config.wallet).unwrap());
        let payer_pubkey = payer.pubkey();
        let client = Client::new(anchor_config.cluster, payer.clone());
        let pubkey = Pubkey::from_str(&anchor_config.program).unwrap();
        let program = client.program(pubkey).unwrap();

        // Derive expected PDAs
        let (model_features_pda, _) = Pubkey::find_program_address(
            &[b"model_features", payer_pubkey.as_ref()],
            &program.id()
        );
        let (data_prices_pda, _) = Pubkey::find_program_address(
            &[b"data_prices", payer_pubkey.as_ref()],
            &program.id()
        );

        // --- 1. Check ModelFeatures account existence and contents ---
        let model_features_before = program
            .account::<ModelFeatures>(model_features_pda)
            .expect("ModelFeatures account should exist");

        println!("ModelFeatures before calculation:");
        println!("  - Last update: {}", model_features_before.last_update);
        println!("  - Computed features: {:?}", model_features_before.computed_features);

        assert_eq!(
            model_features_before.authority, payer_pubkey,
            "Authority should match test wallet"
        );

        // --- 2. Check DataPrices account existence and contents ---
        let data_prices = program
            .account::<DataPrices>(data_prices_pda)
            .expect("DataPrices account should exist");

        println!("DataPrices:");
        println!("  - Prices: {:?}", data_prices.prices);

        // Ensure at least one price is non-zero (i.e., data has been written)
        assert!(
            data_prices.prices.iter().any(|&p| p != 0.0),
            "DataPrices.prices should contain non-zero values"
        );

        // --- 3. Call calculate_features instruction ---
        // Build the instruction
        let ix = program
            .request()
            .accounts(luciene_sl::accounts::CalculateFeatures {
                model_features: model_features_pda,
                data_prices: data_prices_pda,
                authority: payer_pubkey,
                system_program: solana_sdk::system_program::ID,
            })
            .args(luciene_sl::instruction::CalculateFeatures {})
            .signer(payer.clone());

        // Send transaction
        let sig = ix.send().expect("calculate_features instruction failed");

        println!("calculate_features transaction signature: {}", sig);

        // --- 4. Fetch ModelFeatures again and validate update ---
        let model_features_after = program
            .account::<ModelFeatures>(model_features_pda)
            .expect("ModelFeatures account should exist after calculation");

        println!("ModelFeatures after calculation:");
        println!("  - Last update: {}", model_features_after.last_update);
        println!("  - Computed features: {:?}", model_features_after.computed_features);

        // Check that last_update changed and features are computed (non-zero)
        assert!(
            model_features_after.last_update > model_features_before.last_update,
            "last_update should be updated after calculation"
        );
        assert!(
            model_features_after.computed_features.iter().any(|&f| f != 0.0),
            "Computed features should contain non-zero values after calculation"
        );
    }
}
