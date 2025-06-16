# luciene-sl

Transparent and Stateless Agent for OnChain Financial Models.

## Local testing

This is done in 3 steps: 

1. Configure and start a local validator.
2. Generate and fund a signer.
3. Validate and localy use the program. 

Configure Solana CLI to use `localhost`, which usually would be `http://localhost::8099`

```shell
solana config set --url localhost
```

Start a local validator for testing purposes.

```shell
solana-test-validator
```
build the anchor program

```shell
anchor build
```

```shell
anchor deploy
```

Generate a signer to be the default

```shell
solana-keygen new -o target/deploy/luciene-keypair.json
```

Airdrop solana tokens to have balance

```shell
solana airdrop 10
```

## Devnet deployment

### Config 

Configure to devnet

```shell
solana config set --url devnet
```

validate 

```shell
solana config get
```

### Wallet and Funds

optionally, create a new wallet. 

```shell
solana-keygen new --outfile ~/.config/solana/id.json
```

Get the solana address

```shell
solana address
```

Fund the wallet with SOL (in devnet)

```shell
solana airdrop 5
```

Verify it was sucessfully created and additioned the new 5 SOL balance

```shell
solana balance
```

### Build and deploy program

Build the program

```shell
anchor build
```

Generate the program ID

```shell
solana address -k target/deploy/luciene-keypair.json
```

Now, update the program ID in the respective codes. 

- `programs/luciene-sl/src/lib.rs` in the `declare_id!("<PROGRAM_ID>")` line.
- `Anchor.toml` in the `lucien = "\<PROGRAM_ID\>"

Now, rebuild the program.

```shell
anchor build
```

Validate manually the `PROGRAM_ID` is correctly updated. Then, deploy to the `devnet`.

```shell
anchor deploy
```

Check the status of the program, now should be deployed. 

```shell
solana program show PROGRAM_ID
```

or look at the logs 

```shell
solana logs PROGRAM_ID
```

### Initialize, update and test program's functionality

In order to conduct tests, as they are defined in `/test` run the following command 
with the `--skip-local-validator` flag in order to avoid the full cycle of build, deploy, 
test and shutdown. In favor for a more granular sequence. 

```shell
anchor test --skip-local-validator
```

