use solana_sdk::pubkey::Pubkey;
use anchor_client::{Client, Cluster};
use solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use std::str::FromStr;

pub struct UserAccount {
    pub first_name: String,
    pub last_name: String,
}

fn main() {
    // Load the payer keypair (typically the keypair for the wallet you want to use)
    let payer = Keypair::from_base58_string("uKFHJxJQY7uKeX76RgTtFPeAuQwTFSdf4NuanGCiEYmBSLHN88ad66NGGm9vG7X36ZUtRaqYL8X179fN7dHCng8");
    // println!("{}", &payer.pubkey());
    // Replace with your program ID
    let program_id = Pubkey::from_str("99MH9CydVjVr3sZY2PvQ6nHmRZagg3peScR3LKF2Fwkd").unwrap();

    // Connect to the Solana cluster (Devnet or Testnet)
    let client = Client::new_with_options(
        Cluster::Devnet, // Or `Cluster::Testnet` or `Cluster::Mainnet` as needed
        &payer,           // Use the Keypair directly
        CommitmentConfig::confirmed(),
    );


    // Connect to the Solana program
    let program = client.program(program_id);

    // Public key of the account where data is stored (replace with actual public key)
    let user_account_pubkey = Pubkey::from_str("B4DgYfUAqd2DkTdBqGSZo7rGB4dDdLum6QgctRS4fLNQ").unwrap();

    // Fetch the data from the on-chain account
    match program.account::<UserAccount>(user_account_pubkey) {
        Ok(user_account) => {
            println!("User's first name: {}", user_account.first_name);
            println!("User's last name: {}", user_account.last_name);
        }
        Err(err) => {
            eprintln!("Failed to fetch user account: {:?}", err);
        }
    }

}
