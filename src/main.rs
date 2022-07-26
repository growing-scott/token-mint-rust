use {
    solana_sdk::{
        hash::Hash,
        program_pack::Pack,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
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
use std::{env, fs};
use std::num::ParseIntError;

fn main() {
    println!("mint new token");
    let args: Vec<String> = env::args().collect();

    let _cluster = &args[1];
    let _decimals = u8::from_str(&args[3]).unwrap();
    let _amount: Result<u64, ParseIntError> = u64::from_str(&args[4]);

    let _private_path = &args[2];
    let _contents = fs::read_to_string(_private_path).expect("File not found.");

    println!("_cluster: {}", _cluster);
    println!("_decimals: {}", _decimals);
    println!("_contents: {}", _contents);
    println!("_amount: {}", _amount.unwrap());


    let test = Keypair::from_bytes(&_contents.as_bytes()).unwrap();
    println!("test: {}", test.pubkey().to_string());

    //let _cluster = &args[1];
    //let _private_path = &args[2];
    //let _amount = &args[3];

    //println!("{}", _private_path);

    //
    //println!("읽은 파일 내용: \n {}", contents);

    let token_program = &id();

    let mut url = String::from("https://api.devnet.solana.com");
    if _cluster == "mainnet" {
        url = String::from("https://api.devnet.solana.com");
    }

    // Connection
    let conn = RpcClient::new_with_timeout_and_commitment(
        url,
        Duration::from_secs(30),
        CommitmentConfig::confirmed(),
    );
    let recent_blockhash = conn.get_latest_blockhash().unwrap();

    // Token Account
    let payer = Keypair::from_bytes(&[224,245,41,255,144,138,135,186,235,154,170,223,80,83,181,247,78,211,216,34,24,113,171,196,90,107,106,202,129,125,107,58,138,8,204,161,88,214,230,228,127,94,238,74,147,80,105,97,220,85,34,76,115,69,120,246,178,86,221,129,3,63,65,42]).unwrap();
    //let mint_account = Keypair::from_bytes(&[224,245,41,255,144,138,135,186,235,154,170,223,80,83,181,247,78,211,216,34,24,113,171,196,90,107,106,202,129,125,107,58,138,8,204,161,88,214,230,228,127,94,238,74,147,80,105,97,220,85,34,76,115,69,120,246,178,86,221,129,3,63,65,42]).unwrap();
    let owner = Keypair::from_bytes(&[224,245,41,255,144,138,135,186,235,154,170,223,80,83,181,247,78,211,216,34,24,113,171,196,90,107,106,202,129,125,107,58,138,8,204,161,88,214,230,228,127,94,238,74,147,80,105,97,220,85,34,76,115,69,120,246,178,86,221,129,3,63,65,42]).unwrap();

    // Account Balance Check(Payer)
    let account = conn.get_account(&payer.pubkey()).unwrap();
    println!("account: {}", account.lamports);
    if account.lamports < 1000000000 {
        panic!("Not enough balance");
    }

    // Mint Token
    let mint_rent = conn.get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap();

    let mint_account = Keypair::new();

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
    )
        .unwrap();

    let mint_to_tx = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&payer.pubkey()),
        &[&payer, &owner],
        recent_blockhash,
    );
    let mint_to_signature = conn.send_and_confirm_transaction(&mint_to_tx).unwrap();
    println!("Mint to. Signature: {}", mint_to_signature);







    // Create account that can hold the newly minted tokens

    /*
    let account_rent = conn.get_minimum_balance_for_rent_exemption(Account::LEN).unwrap();
    let token_account = Keypair::new();
    let new_token_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &token_account.pubkey(),
        account_rent,
        Account::LEN as u64,
        token_program,
    );

    let my_account = Keypair::new();

    let initialize_account_ix = instruction::initialize_account(
        &spl_associated_token_account::id(),
        &token_account.pubkey(),
        &mint_account.pubkey(),
        &my_account.pubkey(),
    )
        .unwrap();

    let create_new_token_account_tx = Transaction::new_signed_with_payer(
        &[new_token_account_ix, initialize_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &token_account],
        recent_blockhash,
    );
    let signature = conn.send_and_confirm_transaction(&create_new_token_account_tx).unwrap();
    println!("create_new_token_account. Signature: {}", signature);
     */
    //let mint_account = Pubkey::new("2MpTrG9Wes5Xh3cpf4JyCoGo1gnHAYbcQfnRFLfTxDTN".as_bytes());

    //let mint_account = Pubkey::from_str("2MpTrG9Wes5Xh3cpf4JyCoGo1gnHAYbcQfnRFLfTxDTN").unwrap();
    //let associated_program_id = Pubkey::from_str("2MpTrG9Wes5Xh3cpf4JyCoGo1gnHAYbcQfnRFLfTxDTN").unwrap();


    /*
    let associated_token_account_ix= spl_associated_token_account::instruction::create_associated_token_account(&payer.pubkey(), &owner.pubkey(), &mint_account);

    let create_new_token_account_tx = Transaction::new_signed_with_payer(
        &[associated_token_account_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let signature = conn.send_and_confirm_transaction(&create_new_token_account_tx).unwrap();
    println!("create_new_token_account_tx signature. {}", signature);
    */







}
