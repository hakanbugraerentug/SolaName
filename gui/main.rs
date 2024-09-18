use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::io::{self, Write};
use std::str::FromStr;
use anchor_client::solana_sdk::signer::keypair::Keypair;
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{Client, Cluster};
use dotenv::dotenv;

struct UserAccount {
    first_name: String,
    last_name: String,
}

fn main() {
    // Create an RpcClient connected to the Solana testnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);

    let mut user_account = UserAccount {
        first_name: String::new(),
        last_name: String::new(),
    };

    loop {
        // Display the menu
        println!(" _____       _       _   _       ");               
        println!(" / ____|     | |     | \\ | |       ");              
        println!("| (___   ___ | | __ _|  \\| | __ _ _ __ ___   ___ ");
        println!(" \\___ \\ / _ \\| |/ _` | . ` |/ _` | '_ ` _ \\ / _ \\");
        println!(" ____) | (_) | | (_| | |\\  | (_| | | | | | |  __/");
        println!("|_____/ \\___/|_|\\__,_|_| \\_|\\__,_|_| |_| |_|\\___|");
        println!("====================== v0.0.1 by hakanbugraerentug");

        println!("1. Set Name");
        println!("2. Get Name");
        println!("3. Request Airdrop");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        // Read user input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {

                // The first and last names to be passed into the create_user function
                let mut first_name = String::new();
                let mut last_name = String::new();

                // Set the user's name
                print!("Enter first name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut first_name).unwrap();

                print!("Enter last name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut last_name).unwrap();

                // Remove newline characters
                user_account.first_name = user_account.first_name.trim().to_string();
                user_account.last_name = user_account.last_name.trim().to_string();

                // Load the payer keypair (typically the keypair for the wallet you want to use)
                let priv_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY is not set");
                let payer = Keypair::from_base58_string(priv_key);

                // Replace with your program ID
                let program_id = Pubkey::from_str("8ZUmf4y3KiNbi8sb68BWxJH4nQXTJvSyfevwsVKm8iV2").unwrap();

                // Connect to Solana devnet or testnet (adjust as necessary)
                let client = Client::new_with_options(Cluster::Devnet, payer.clone(), CommitmentConfig::confirmed());

                // Connect to the program
                let program = client.program(program_id);

                // Create a keypair for the new account (user)
                let new_account = Keypair::new();

                // Send the transaction to call `create_user`
                let tx = program
                    .request()
                    .accounts(instruction::AccountMeta {
                        pubkey: new_account.pubkey(),
                        is_signer: true,
                        is_writable: true,
                    })
                    .args((
                        first_name, // First argument: first name
                        last_name,  // Second argument: last name
                    ))
                    .signer(&payer)
                    .send();

                match tx {
                    Ok(signature) => {
                        println!("Transaction sent! Tx signature: {}", signature);
                    }
                    Err(err) => {
                        println!("Transaction failed: {:?}", err);
                    }
                }                

                println!("Name set: {} {}", user_account.first_name, user_account.last_name);
            }
            "2" => {

                // Load the payer keypair (typically the keypair for the wallet you want to use)
                let priv_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY is not set");
                let payer = Keypair::from_base58_string(priv_key);

                // Replace with your program ID
                let program_id = Pubkey::from_str("8ZUmf4y3KiNbi8sb68BWxJH4nQXTJvSyfevwsVKm8iV2").unwrap();

                // Connect to Solana devnet or testnet
                let client = Client::new_with_options(Cluster::Devnet, payer.clone(), CommitmentConfig::confirmed());

                // Connect to the program
                let program = client.program(program_id);

                // The public key of the user account that stores the name (replace with actual pubkey)
                let mut user_pubkey = String::new();
                print!("Enter User account Public Key: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut user_pubkey).unwrap();

                let user_account_pubkey = Pubkey::from_str(&user_pubkey).unwrap();

                // Fetch the account data from the on-chain account
                match program.account::<UserAccount>(user_account_pubkey) {
                    Ok(user_account) => {
                        println!("User's first name: {}", user_account.first_name);
                        println!("User's last name: {}", user_account.last_name);
                    }
                    Err(err) => {
                        println!("Failed to fetch user account: {:?}", err);
                    }
                }


            }
            "3" => {
                // Request an airdrop
                print!("Enter your public key: ");
                io::stdout().flush().unwrap();

                let mut pubkey_str = String::new();
                io::stdin().read_line(&mut pubkey_str).unwrap();
                pubkey_str = pubkey_str.trim().to_string();

                match Pubkey::from_str(&pubkey_str) {
                    Ok(pubkey) => {
                        let lamports = 1_000_000_000; // 1 SOL in lamports
                        match client.request_airdrop(&pubkey, lamports) {
                            Ok(signature) => {
                                println!("Airdrop requested successfully! Tx: {}", signature);
                            }
                            Err(err) => {
                                println!("Failed to request airdrop: {:?}", err);
                            }
                        }
                    }
                    Err(_) => {
                        println!("Invalid public key.");
                    }
                }
            }
            "4" => {
                // Exit the program
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please choose an option from 1 to 4.");
            }
        }
    }
}
