use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::{Keypair, Signer}, system_instruction, transaction::Transaction};
use std::io::{self, Write};
use std::str::FromStr;
use bs58;

fn main() {
    // Select Solana network (mainnet/testnet/devnet)
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    print!("Enter the wallet address to check balance: ");
    io::stdout().flush().unwrap();
    let mut wallet_address = String::new();
    io::stdin().read_line(&mut wallet_address).expect("Input error");
    let wallet_address = wallet_address.trim();

    let pubkey = match Pubkey::from_str(wallet_address) {
        Ok(pk) => pk,
        Err(_) => {
            println!("Invalid address.");
            return;
        }
    };

    match client.get_balance(&pubkey) {
        Ok(lamports) => {
            let sol = lamports as f64 / 1_000_000_000.0;
            println!("Balance: {} SOL", sol);
        }
        Err(e) => println!("Failed to fetch balance: {}", e),
    }

    // === Transfer feature step 1: recipient address ===
    print!("Enter recipient wallet address: ");
    io::stdout().flush().unwrap();
    let mut recipient_address = String::new();
    io::stdin().read_line(&mut recipient_address).expect("Input error");
    let recipient_address = recipient_address.trim();
    let recipient_pubkey = match Pubkey::from_str(recipient_address) {
        Ok(pk) => pk,
        Err(_) => {
            println!("Invalid recipient address.");
            return;
        }
    };

    // === Transfer feature step 2: amount ===
    print!("Enter amount to send (SOL): ");
    io::stdout().flush().unwrap();
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Input error");
    let amount: f64 = match amount.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid amount.");
            return;
        }
    };
    let lamports = (amount * 1_000_000_000.0) as u64;

    // === Transfer feature step 3: sender's private key ===
    print!("Enter your private key (base58 string from Phantom): ");
    io::stdout().flush().unwrap();
    let mut private_key = String::new();
    io::stdin().read_line(&mut private_key).expect("Input error");
    let private_key = private_key.trim();

    // Convert base58 private key to Keypair
    let keypair_bytes = match bs58::decode(private_key).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("Failed to decode private key. Make sure it's a valid base58 string.");
            return;
        }
    };
    let sender = match Keypair::from_bytes(&keypair_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            println!("Failed to create Keypair from private key bytes.");
            return;
        }
    };

    // === Create, sign, and send transaction ===
    let transfer_ix = system_instruction::transfer(&sender.pubkey(), &recipient_pubkey, lamports);
    let recent_blockhash = match client.get_latest_blockhash() {
        Ok(hash) => hash,
        Err(e) => {
            println!("Failed to get recent blockhash: {}", e);
            return;
        }
    };
    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => println!("Transaction successful! Signature: {}", sig),
        Err(e) => println!("Transaction failed: {}", e),
    }
} 