mod utils;
// use crate::blockchain::Blockchain;
use crate::utils::Keypair;
fn main() {
    let keypair = Keypair::generate();
    println!("Keypair: {:#?}", keypair.clone());
    let keypair2 = Keypair::from_private_key(keypair.private_key.clone());
    println!("Keypair2: {:#?}", keypair2.clone());
}