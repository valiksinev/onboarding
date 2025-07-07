use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        message::Message,
        pubkey::Pubkey, instruction::{AccountMeta, Instruction},
        commitment_config::{CommitmentConfig, CommitmentLevel},
        signature::{read_keypair_file, Signer},
        transaction::Transaction,
    },
    std::{
        str::FromStr, path::Path
    },
};

// path to keypair
const KEYPAIR_PATH: &str = "/home/user/.config/solana/id.json";
// solana is running on local machine
const SOLANA_URL: &str = "http://localhost:8899";
// program_id of the "memo" program
// https://github.com/solana-program/memo/blob/37568de8dae6a4e69572a85e8c166910da232b90/program/src/lib.rs#L26
const PROGRAM_ID: &str = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr";

// create instuction of the memo-program
pub fn build_memo(memo: &[u8], signer_pubkeys: &[&Pubkey]) -> Instruction {
    Instruction {
        program_id: Pubkey::from_str(PROGRAM_ID).unwrap(),
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data: memo.to_vec(),
    }
}

fn main() {
    // read keypair, will be used to sign transaction
    let payer =  read_keypair_file(Path::new(KEYPAIR_PATH)).unwrap();

    // rpc-client, it will be used to send transaction to solana-validator
    let client = RpcClient::new_with_commitment(
        SOLANA_URL,
        CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        },
    );

    // create instruction
    let ix = build_memo("hello world".as_bytes(), &[&payer.pubkey()]);

    // take a look at purpose of the blockhash:
    // https://solana.com/docs/core/transactions#recent-blockhash
    let blockhash = client.get_latest_blockhash().unwrap();

    let message = Message::new(&[ix], Some(&payer.pubkey()));

    // solana tx
    let tx = Transaction::new(&[payer], message, blockhash);

    // let's send it!
    let  sig= client.send_and_confirm_transaction(&tx).unwrap();

    println!("we have done it, solana signature: {}", sig);
}
