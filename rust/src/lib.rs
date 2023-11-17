use solana_client::rpc_client::RpcClient;
use solana_program::system_program;
use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, read_keypair_file},
    system_instruction::transfer,
    transaction::Transaction,
};
use std::str::FromStr;

mod programs;
use programs::wba_prereq::{WbaPrereqProgram, CompleteArgs};


const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)]
mod tests {
    use solana_program::message;
    use solana_sdk::signer::Signer;
    use bs58;
    use std::io::{self, BufRead};

    use super::*;

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
      println!("Input your private key as base58:");
      let stdin = io::stdin();
      let base58 = stdin.lock().lines().next().unwrap().unwrap();
      let wallet = bs58::decode(base58).into_vec().unwrap();
      println!("Your wallet file is: {:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',').map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn claim_airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(signature) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", signature);
            },
            Err(err) => println!("Oops, something went wrong: {}", err),
        }
    }

    #[test]
    fn transfer_to_wba() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("SeseaYWQAV5m257VVmCPpNTenjLE9q4oHoSN8wYSwtB").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        // Get balance of dev wallet
        let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");

        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
        &[transfer(
        &keypair.pubkey(),
        &to_pubkey,
        balance,
        )],
        Some(&keypair.pubkey()),
        &recent_blockhash
        );

        // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
        let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            balance - fee,
            )],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );

        let signature = rpc_client.send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
       
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }

    #[test]
    fn submit_completion() {
        let signer: Keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        println!("Captured Signer: {}", signer.pubkey());

        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        println!("Captured Blockhash: {}", recent_blockhash);
    
        let args = CompleteArgs {
            github: b"MinhNguyen".to_vec(),
        };

        let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq",signer.pubkey().to_bytes().as_ref()]);
    
        let new_prereq = WbaPrereqProgram::derive_program_address(&[b"prereq", b"random_bytes".as_ref()]);

        // Define our instruction data
        let args = CompleteArgs {
        github: b"MinhNguyen".to_vec()
        };

        // Get recent blockhash
        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let existing_account = match rpc_client.get_account(&signer.pubkey()) {
            Ok(account) => account,
            Err(err) => {
                eprintln!("Failed to fetch account: {:?}", err);
                return;
            }
        };
    
        // Now we can invoke the "complete" function
        let transaction = WbaPrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash
        );
    
        println!("Catured TXN: {}", transaction.is_signed());
    
        let signature = rpc_client.send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
    
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }

}