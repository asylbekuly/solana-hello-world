use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    // 👉 Твой задеплоенный Program ID
    let program_id = Pubkey::from_str("Bdp9Vus1bgucSnzjTfy3WAkpsh6ZesRoqzt7uaZMBkvR").unwrap();

    // Подключение к локальному RPC
    let rpc_url = String::from("http://127.0.0.1:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Создаём плательщика
    let payer = Keypair::new();

    // Запрашиваем airdrop
    let airdrop_amount = 1_000_000_000; // 1 SOL
    let signature = client
        .request_airdrop(&payer.pubkey(), airdrop_amount)
        .expect("Airdrop failed");

    // Ждём подтверждения
    loop {
        if client.confirm_transaction(&signature).unwrap() {
            break;
        }
    }

    // Создаём инструкцию
    let instruction = Instruction {
        program_id,
        accounts: vec![], // нет аккаунтов
        data: vec![],     // пустые данные
    };

    // Создаём транзакцию
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], client.get_latest_blockhash().unwrap());

    // Отправляем транзакцию
    match client.send_and_confirm_transaction(&transaction) {
        Ok(sig) => println!("✅ Transaction Signature: {}", sig),
        Err(err) => eprintln!("❌ Error: {}", err),
    }
}
