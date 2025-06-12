use std::str::FromStr;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, 
        pubkey::Pubkey, 
        signature::{read_keypair_file, Signer},
        system_program
    },
    Client, Cluster
};
use anyhow::Result;

const PROGRAM_ID: &str = "Dig4zzQSRAg1bHzHwH4uGCLJAYNxVHqt66QWH1WyHSDH";

pub fn main() -> Result<()> {

    // 1. Set up client and payer
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json")).unwrap();
    let client = Client::new_with_options(
        Cluster::Devnet,
        &payer,
        CommitmentConfig::confirmed()
    );
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    let program = client.program(program_id)?;

    // 2. Derive PDAs for all accounts
    let authority = payer.pubkey();
    
    let (model_params, _) = Pubkey::find_program_address(
        &[b"model_params", authority.as_ref()],
        &program_id
    );

    let (model_results, _) = Pubkey::find_program_address(
        &[b"model_results", model_params.as_ref()],
        &program_id
    );

    let (model_features, _) = Pubkey::find_program_address(
        &[b"model_features", authority.as_ref()],
        &program_id
    );

    let (price_history, _) = Pubkey::find_program_address(
        &[b"price_history", authority.as_ref()],
        &program_id
    );

    // 3. Initialize Model Parameters
    let weights = [0.1, 0.2, 0.3, 0.2, 0.2];
    let bias = 0.05;
    
    let tx = program.request()
        .accounts(lucien::accounts::InitializeModel {
            model_params,
            authority,
            system_program: system_program::ID
        })
        .args(lucien::instruction::InitializeModel { weights, bias })
        .signer(&payer)
        .send()?;
    println!("ModelParams initialized: {}", tx);

    // 4. Initialize Model Results
    let tx = program.request()
        .accounts(lucien::accounts::InitializeResults {
            model_results,
            model_params,
            authority,
            system_program: system_program::ID
        })
        .args(lucien::instruction::InitializeResults {})
        .signer(&payer)
        .send()?;
    println!("ModelResults initialized: {}", tx);

    // 5. Initialize Model Features
    let tx = program.request()
        .accounts(lucien::accounts::InitializeFeatures {
            model_features,
            authority,
            system_program: system_program::ID
        })
        .args(lucien::instruction::InitializeFeatures {})
        .signer(&payer)
        .send()?;
    println!("ModelFeatures initialized: {}", tx);

    // 6. Initialize Price History
    let tx = program.request()
        .accounts(lucien::accounts::InitializePriceHistory {
            price_history,
            authority,
            system_program: system_program::ID
        })
        .args(lucien::instruction::InitializePriceHistory {})
        .signer(&payer)
        .send()?;
    println!("PriceHistory initialized: {}", tx);

    Ok(())
}

