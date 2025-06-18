#[cfg(test)]

// -- ---------------------------------------------------------------- DEVNET TESTS -- //
// -- ---------------------------------------------------------------- ------------ -- //

mod tests {

    #[test]
    fn verify_devnet_connectivity() {
        
        use solana_client::rpc_client::RpcClient;
        println!(" Verifying devnet connectivity...");
        
        let client = RpcClient::new(std::env::var("NETWORK").unwrap());
        
        // Test basic connectivity
        let version = client.get_version().unwrap();
        println!("Connected to devnet");
        println!("  - Solana version: {}", version.solana_core);
        
        // Test recent blockhash
        let recent_blockhash = client.get_latest_blockhash().unwrap();
        println!("  - Latest blockhash: {}", recent_blockhash);
        
    }
}

