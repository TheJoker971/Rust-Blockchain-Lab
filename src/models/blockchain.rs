use crate::models::{Block, Wallet};
use std::collections::HashMap;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub wallets: HashMap<String, Wallet>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: Vec::new(), wallets: HashMap::new() }
    }
}