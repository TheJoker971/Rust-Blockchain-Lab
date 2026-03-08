use crate::utils::Keypair;

#[derive(Debug, Clone)]
pub struct Wallet {
    pub address: String,
    pub balance: u64,
}

impl Wallet {
    pub fn new() -> Self {
        let keypair = Keypair::generate();
        println!("This is your new wallet address: {}", keypair.public_key);
        println!("This is your new wallet private key: {}", keypair.private_key);
        Self { address: keypair.public_key.clone(), balance: 0 }
    }
}