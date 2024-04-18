use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::message::Message;
use solana_sdk::signature::Signer;
use solana_sdk::system_instruction::transfer;
use solana_sdk::transaction::Transaction;

fn main() {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let wallet1_key: [u8; 64] = [
        162, 252, 220, 218, 122, 18, 214, 234, 110, 70, 175, 94, 239, 23, 99, 176, 177, 63, 53, 250, 105, 249, 179, 208, 16, 33, 23, 199, 237, 42, 113, 22, 10, 36, 16, 34, 44, 180, 26, 51, 137, 166, 47, 218, 239, 55, 175, 71, 152, 157, 190, 216, 78, 28, 71, 127, 103, 172, 55, 188, 150, 62, 241, 228
    ];

    let wallet2_key: [u8; 64] = [
        141, 235, 25, 230, 203, 102, 247, 159, 194, 83, 164, 162, 100, 178, 231, 6, 145, 18, 90, 33, 107, 89, 33, 44, 144, 210, 100, 114, 195, 226, 60, 238, 26, 139, 168, 14, 180, 226, 98, 39, 186, 196, 30, 44, 194, 241, 199, 178, 31, 20, 118, 11, 116, 119, 72, 161, 128, 176, 5, 107, 166, 238, 149, 179
    ];

    let wallet1 = Keypair::from_bytes(&wallet1_key)
        .expect("Failed to create keypair from bytes");

    let wallet2 = Keypair::from_bytes(&wallet2_key)
        .expect("Failed to create keypair from bytes");

    println!("{}", wallet1.pubkey());
    println!("{}", wallet2.pubkey());

    let wallet1_bal = rpc_client
        .get_balance(&wallet1.pubkey())
        .expect("Failed to retrieve balance");

    let wallet2_bal = rpc_client
        .get_balance(&wallet2.pubkey())
        .expect("Failed to retrieve balance");

    println!("Wallet 1 balance: {}\nWallet 2 balance: {}", wallet1_bal, wallet2_bal);

    let transfer_instruction = transfer(&wallet1.pubkey(), &wallet2.pubkey(), 1_000_000);
    let message = Message::new(&[transfer_instruction], Some(&wallet1.pubkey()));
    let transaction = Transaction::new_unsigned(message);

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_config(
            &transaction,
            CommitmentConfig::processed(),
            solana_client::rpc_config::RpcSendTransactionConfig {
                skip_preflight: false,
                preflight_commitment: None,
                encoding: None,
                max_retries: None,
                min_context_slot: None,
            },
        )
        .unwrap();

    println!("Transaction signature: {:?}", signature);

    println!("Wallet 1 balance: {}\nWallet 2 balance: {}", wallet1_bal, wallet2_bal);

}