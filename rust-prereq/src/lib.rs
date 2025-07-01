use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    system_program,
    hash::Hash,
};
use std::str::FromStr;

#[test]
fn submit_rs() {
    // 1. RPC Connection
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    // 2. Load signer keypair
    let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
    let signer_pubkey = signer.pubkey();

    // 3. Define fixed program IDs
    let turbin3_prereq_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
    let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
    let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    let system_program = system_program::id();

    // 4. Derive existing PDA created earlier via TS `.initialize()`
    let prereq_seeds = &[b"prereqs", signer_pubkey.as_ref()];
    let (prereq_pda, _) = Pubkey::find_program_address(prereq_seeds, &turbin3_prereq_program);

    // 5. Derive authority PDA (used internally by program)
    let authority_seeds = &[b"collection", collection.as_ref()];
    let (authority, _) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);

    // 6. Generate a new mint keypair (ephemeral NFT mint)
    let mint = Keypair::new();

    // 7. submit_rs discriminator from IDL
    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

    // 8. Accounts required by submit_rs
    let accounts = vec![
        AccountMeta::new(signer_pubkey, true),
        AccountMeta::new(prereq_pda, false),
        AccountMeta::new(mint.pubkey(), true),
        AccountMeta::new(collection, false),
        AccountMeta::new_readonly(authority, false),
        AccountMeta::new_readonly(mpl_core_program, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    // 9. Instruction
    let instruction = Instruction {
        program_id: turbin3_prereq_program,
        accounts,
        data,
    };

    // 10. Transaction + signers
    let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer_pubkey),
        &[&signer, &mint],
        blockhash,
    );

    // 11. Submit transaction
    let sig = rpc_client
        .send_and_confirm_transaction(&tx)
        .expect("Transaction failed");

    println!(
        "Success! Check your TX: https://explorer.solana.com/tx/{}?cluster=devnet",
        sig
    );
}
