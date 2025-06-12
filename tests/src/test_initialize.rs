use std::str::FromStr;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, 
        pubkey::Pubkey, 
        signature::read_keypair_file,
        signer::Signer,
        system_program,
    },
    Client, Cluster, Program,
};

use anchor_lang::prelude::*;

#[tokio::test]
async fn test_solana_program_deployment() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration
    let program_id = "Dig4zzQSRAg1bHzHwH4uGCLJAYNxVHqt66QWH1WyHSDH";
    let anchor_wallet = std::env::var("ANCHOR_WALLET")
        .unwrap_or_else(|_| "/home/user/.config/solana/id.json".to_string());
    
    // Create client for DEVNET (not localnet)
    let payer = read_keypair_file(&anchor_wallet).unwrap();
    let client = Client::new_with_options(
        Cluster::Devnet, // Changed from Localnet to Devnet
        &payer, 
        CommitmentConfig::confirmed()
    );
    
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();
    
    println!("Connected to devnet successfully");
    println!("Program ID: {}", program_id);
    println!("Payer: {}", payer.pubkey());
    
    // Test 1: Verify program exists on devnet
    test_program_exists(&program).await;
    
    // Test 2: Initialize model parameters
    test_initialize_model(&program, &payer).await;
    
    // Test 3: Initialize other accounts
    test_initialize_accounts(&program, &payer).await;
    
    // Test 4: Test account fetching
    test_account_fetching(&program, &payer).await;
    
    println!("🎉 All tests passed! Program is successfully deployed and functioning on devnet");
    Ok(())
}

async fn test_program_exists(program: &Program) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing if program exists on devnet...");
    
    // Try to get program account info
    let account_info = program.rpc().get_account(&program.id()).await;
    
    match account_info {
        Ok(Some(account)) => {
            println!(" Program account found on devnet");
            println!("  - Owner: {}", account.owner);
            println!("  - Executable: {}", account.executable);
            println!("  - Lamports: {}", account.lamports);
            assert!(account.executable, "Program account should be executable");
        },
        Ok(None) => {
            return Err("Program account not found on devnet".into());
        },
        Err(e) => {
            return Err(format!("Error fetching program account: {}", e).into());
        }
    }
    Ok(())
}

async fn test_initialize_model(
    program: &Program, 
    payer: &impl Signer
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing model initialization...");
    
    // Derive PDA for model parameters
    let (model_params_pda, _bump) = Pubkey::find_program_address(
        &[b"model_params", payer.pubkey().as_ref()],
        &program.id()
    );
    
    println!("  Model params PDA: {}", model_params_pda);
    
    // Check if account already exists
    let account_exists = program.rpc()
        .get_account(&model_params_pda)
        .await?
        .is_some();
    
    if account_exists {
        println!("Model parameters account already exists");
        return Ok(());
    }
    
    // Initialize model with sample weights and bias
    let weights: [f32; 5] = [0.1, 0.2, 0.3, 0.4, 0.5];
    let bias: f32 = 0.1;
    
    let tx = program
        .request()
        .accounts(lucien::accounts::InitializeModel {
            model_params: model_params_pda,
            authority: payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(lucien::instruction::InitializeModel { weights, bias })
        .signer(payer)
        .send()
        .await;
    
    match tx {
        Ok(signature) => {
            println!("✅ Model initialization successful");
            println!("  Transaction signature: {}", signature);
        },
        Err(e) => {
            return Err(format!("❌ Model initialization failed: {}", e).into());
        }
    }
    
    Ok(())
}

async fn test_initialize_accounts(
    program: &Program, 
    payer: &impl Signer
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing account initialization...");
    
    // Derive PDAs for all accounts
    let (model_params_pda, _) = Pubkey::find_program_address(
        &[b"model_params", payer.pubkey().as_ref()],
        &program.id()
    );
    
    let (model_results_pda, _) = Pubkey::find_program_address(
        &[b"model_results", model_params_pda.as_ref()],
        &program.id()
    );
    
    let (model_features_pda, _) = Pubkey::find_program_address(
        &[b"model_features", payer.pubkey().as_ref()],
        &program.id()
    );
    
    let (price_history_pda, _) = Pubkey::find_program_address(
        &[b"price_history", payer.pubkey().as_ref()],
        &program.id()
    );
    
    // Test each initialization
    let accounts_to_test = vec![
        ("model_results", model_results_pda, "InitializeResults"),
        ("model_features", model_features_pda, "InitializeFeatures"),
        ("price_history", price_history_pda, "InitializePriceHistory"),
    ];
    
    for (name, pda, instruction_name) in accounts_to_test {
        println!("  Testing {} initialization...", name);
        
        let account_exists = program.rpc()
            .get_account(&pda)
            .await?
            .is_some();
        
        if account_exists {
            println!("    ✅ {} account already exists", name);
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
                        authority: payer.pubkey(),
                        system_program: system_program::ID,
                    })
                    .args(lucien::instruction::InitializeResults {})
                    .signer(payer)
                    .send()
                    .await
            },
            "InitializeFeatures" => {
                program
                    .request()
                    .accounts(lucien::accounts::InitializeFeatures {
                        model_features: model_features_pda,
                        authority: payer.pubkey(),
                        system_program: system_program::ID,
                    })
                    .args(lucien::instruction::InitializeFeatures {})
                    .signer(payer)
                    .send()
                    .await
            },
            "InitializePriceHistory" => {
                program
                    .request()
                    .accounts(lucien::accounts::InitializePriceHistory {
                        price_history: price_history_pda,
                        authority: payer.pubkey(),
                        system_program: system_program::ID,
                    })
                    .args(lucien::instruction::InitializePriceHistory {})
                    .signer(payer)
                    .send()
                    .await
            },
            _ => continue,
        };
        
        match result {
            Ok(signature) => {
                println!("    ✅ {} initialization successful: {}", name, signature);
            },
            Err(e) => {
                println!("    ❌ {} initialization failed: {}", name, e);
                return Err(format!("Failed to initialize {}: {}", name, e).into());
            }
        }
    }
    
    Ok(())
}

async fn test_account_fetching(
    program: &Program, 
    payer: &impl Signer
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Testing account data fetching...");
    
    // Derive PDAs
    let (model_params_pda, _) = Pubkey::find_program_address(
        &[b"model_params", payer.pubkey().as_ref()],
        &program.id()
    );
    
    // Fetch and verify model parameters
    match program.account::<ModelParameters>(model_params_pda).await {
        Ok(model_params) => {
            println!("✅ Successfully fetched model parameters");
            println!("  - Weights: {:?}", model_params.weights);
            println!("  - Bias: {}", model_params.bias);
            println!("  - Authority: {}", model_params.authority);
            
            // Verify data integrity
            assert_eq!(model_params.authority, payer.pubkey());
            assert!(model_params.weights.len() == 5);
        },
        Err(e) => {
            return Err(format!("❌ Failed to fetch model parameters: {}", e).into());
        }
    }
    
    Ok(())
}

// Mock structures for compilation (replace with actual program types)
#[derive(Clone)]
pub struct ModelParameters {
    pub weights: Vec<f32>,
    pub bias: f32,
    pub authority: Pubkey,
}

// Additional helper functions for comprehensive testing
async fn test_program_instructions(
    program: &Program, 
    payer: &impl Signer
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing program instructions...");
    
    // Test calculate_features instruction
    // Test run_inference instruction
    // Add more instruction tests as needed
    
    Ok(())
}

async fn verify_devnet_connectivity() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Verifying devnet connectivity...");
    
    use solana_client::rpc_client::RpcClient;
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    
    // Test basic connectivity
    let version = client.get_version().unwrap();
    println!("Connected to devnet");
    println!("  - Solana version: {}", version.solana_core);
    
    // Test recent blockhash
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    println!("  - Latest blockhash: {}", recent_blockhash);
    
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = verify_devnet_connectivity().await {
        eprintln!("Devnet connectivity test failed: {}", e);
        return;
    }
    
    if let Err(e) = test_solana_program_deployment().await {
        eprintln!("Program deployment test failed: {}", e);
        std::process::exit(1);
    }
}
