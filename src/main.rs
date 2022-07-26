use {
    solana_sdk::{
        hash::Hash,
        program_pack::Pack,
        pubkey::Pubkey,
        signature::{Keypair, Signer, read_keypair_file},
        system_instruction,
        transaction::Transaction,
        transport::TransportError,
    },
    spl_token::{
        id, instruction,
        state::{Account, Mint},
    },
    spl_associated_token_account,
    solana_client::{nonblocking::rpc_client::RpcClient as AsyncRpcClient, rpc_client::RpcClient}
};
use std::time::Duration;
use solana_sdk::commitment_config::CommitmentConfig;
use std::str::FromStr;
use std::{env};
use std::num::ParseIntError;

fn main() {
    println!("mint new token");
    let args: Vec<String> = env::args().collect();

    let _cluster = &args[1];
    let _private_path = &args[2];
    let _decimals = u8::from_str(&args[3]).unwrap();
    let _amount: Result<u64, ParseIntError> = u64::from_str(&args[4]);

    println!("_cluster: {}", _cluster);
    println!("_decimals: {}", _decimals);
    println!("_amount: {}", _amount.as_ref().unwrap());

    let token_program = &id();

    let mut url = String::from("https://api.devnet.solana.com");
    if _cluster == "mainnet" {
        url = String::from("https://api.mainnet-beta.solana.com");
    }

    // Connection
    let conn = RpcClient::new_with_timeout_and_commitment(
        url,
        Duration::from_secs(30),
        CommitmentConfig::confirmed(),
    );
    let recent_blockhash = conn.get_latest_blockhash().unwrap();

    // Token Owner & Payer
    let owner = read_keypair_file(_private_path).unwrap();
    let payer = read_keypair_file(_private_path).unwrap();
    println!("owner: {}", owner.pubkey().to_string());
    println!("payer: {}", payer.pubkey().to_string());

    // Account Balance Check(Payer)
    let account = conn.get_account(&payer.pubkey()).unwrap();
    println!("account balance(sol): {}", account.lamports);
    if account.lamports < 2000000000 {
        panic!("Not enough balance");
    }

    // Mint Token
    let mint_rent = conn.get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap();

    let mint_account = Keypair::new();
    println!("token address: {}", mint_account.pubkey().to_string());

    let token_mint_account_ix = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &mint_account.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        token_program,
    );

    let token_mint_ix = instruction::initialize_mint(
        token_program,
        &mint_account.pubkey(),
        &owner.pubkey(),
        Some(&owner.pubkey()),
        _decimals,
    )
    .unwrap();

    // create mint transaction
    let token_mint_tx = Transaction::new_signed_with_payer(
        &[token_mint_account_ix, token_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint_account],
        recent_blockhash,
    );

    let mint_signature = conn.send_and_confirm_transaction(&token_mint_tx).unwrap();
    println!("Mint Signature: {}", mint_signature);

    // Mint Account
    let create_associated_token_account_ix= spl_associated_token_account::instruction::create_associated_token_account(&payer.pubkey(), &owner.pubkey(), &mint_account.pubkey());

    let create_associated_token_account_tx = Transaction::new_signed_with_payer(
        &[create_associated_token_account_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let associated_token_account_signature = conn.send_and_confirm_transaction(&create_associated_token_account_tx).unwrap();
    println!("associated_token_account_signature. {}", associated_token_account_signature);

    let associated_token_account = spl_associated_token_account::get_associated_token_address(&owner.pubkey(), &mint_account.pubkey());
    println!("associated_token_account. {}", associated_token_account);

    // Mint to
    let mint_to_ix = instruction::mint_to(
        token_program,
        &mint_account.pubkey(),
        &associated_token_account,
        &owner.pubkey(),
        &[],
        _amount.unwrap().clone(),
    ).unwrap();

    let mint_to_tx = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&payer.pubkey()),
        &[&payer, &owner],
        recent_blockhash,
    );
    let mint_to_signature = conn.send_and_confirm_transaction(&mint_to_tx).unwrap();
    println!("Mint to. Signature: {}", mint_to_signature);
}
