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

fn main() {
    println!("Hello, world!");
    //let args: Vec<String> = env::args().collect();


    //let _cluster = &args[1];
    //let _private_path = &args[2];
    //let _amount = &args[3];

    //println!("{}", _private_path);

    //let contents = fs::read_to_string(_private_path).expect("파읽을 읽을 수 없습니다.");
    //println!("읽은 파일 내용: \n {}", contents);

    let conn = RpcClient::new_with_timeout_and_commitment(
        "https://api.devnet.solana.com".to_string(),
        Duration::from_secs(30),
        CommitmentConfig::confirmed(),
    );
    let token_program = &id();
    let payer = Keypair::from_bytes(&[224,245,41,255,144,138,135,186,235,154,170,223,80,83,181,247,78,211,216,34,24,113,171,196,90,107,106,202,129,125,107,58,138,8,204,161,88,214,230,228,127,94,238,74,147,80,105,97,220,85,34,76,115,69,120,246,178,86,221,129,3,63,65,42]).unwrap();
    //let mint_account = Keypair::from_bytes(&[224,245,41,255,144,138,135,186,235,154,170,223,80,83,181,247,78,211,216,34,24,113,171,196,90,107,106,202,129,125,107,58,138,8,204,161,88,214,230,228,127,94,238,74,147,80,105,97,220,85,34,76,115,69,120,246,178,86,221,129,3,63,65,42]).unwrap();
    let owner = Keypair::from_bytes(&[224,245,41,255,144,138,135,186,235,154,170,223,80,83,181,247,78,211,216,34,24,113,171,196,90,107,106,202,129,125,107,58,138,8,204,161,88,214,230,228,127,94,238,74,147,80,105,97,220,85,34,76,115,69,120,246,178,86,221,129,3,63,65,42]).unwrap();

    let mint_account = Keypair::new();
    //let owner = Keypair::new();



    println!("{}", &owner.pubkey());

    let account = conn.get_account(&payer.pubkey()).unwrap();

    println!("account: {}", account.lamports);

    let mint_rent = conn.get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap();

    let recent_blockhash = conn.get_latest_blockhash().unwrap();

    println!("mint_rent: {}", mint_rent);

    let token_mint_a_account_ix = solana_program::system_instruction::create_account(
        &payer.pubkey(),
        &mint_account.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        token_program,
    );


    let token_mint_inst = instruction::initialize_mint(
        token_program,
        &mint_account.pubkey(),
        &owner.pubkey(),
        None,
        4,
    )
    .unwrap();



    // create mint transaction
    let token_mint_tx = Transaction::new_signed_with_payer(
        &[token_mint_a_account_ix, token_mint_inst],
        Some(&payer.pubkey()),
        &[&payer, &mint_account],
        recent_blockhash,
    );

    let signature = conn.send_and_confirm_transaction(&token_mint_tx).unwrap();
    println!("Signature: {}", signature);

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
    let associated_token_address = spl_associated_token_account::get_associated_token_address(&owner.pubkey(), &mint_account.pubkey());

    println!("associated_token_address. {}", &associated_token_address.to_string());

    /*
    // Mint tokens into newly created account
    let mint_amount: u64 = 10000000;
    let mint_to_ix = instruction::mint_to(
        &token_program,
        &mint_account.pubkey(),
        &token_account.pubkey(),
        &owner.pubkey(),
        &[],
        mint_amount.clone(),
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
    */




}
