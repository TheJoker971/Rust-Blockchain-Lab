use crate::models::Transaction;
use std::array;
use chrono::Utc;

pub type BlockHash = String;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: [Transaction; 20],
    pub transaction_count: usize,
    pub previous_hash: BlockHash,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String) -> Result<Self, String> {
        let valide_hash = validate_hash_block(previous_hash);
        if valide_hash.is_err() {
            return Err(valide_hash.unwrap_err());
        }
        Ok(
            Self { 
                index, 
                timestamp: Utc::now().timestamp(), 
                transactions: array::from_fn(|_| Transaction::default()),
                transaction_count: 0,
                previous_hash: valide_hash.unwrap(), 
                hash: String::new() 
            }
        )
    }

    pub fn genesis_block() -> Self {
        Self {
            index: 0,
            timestamp: Utc::now().timestamp(),
            transactions: array::from_fn(|_| Transaction::default()),
            transaction_count: 0,
            previous_hash: format!("0x{}", "0".repeat(64)),
            hash: String::new()
        }
    }
    
    pub fn is_full(&self) -> bool {
        self.transaction_count >= 20
    }

    pub fn add_transaction(&mut self, transaction: &Transaction) -> Result<(), String> {
        if self.is_full() {
            return Err("Block is full".to_string());
        }
        self.transactions[self.transaction_count] = transaction.clone();
        self.transaction_count += 1;
        Ok(())
    }
}

pub fn validate_hash_block(hash:String) -> Result<BlockHash, String> {
    if !hash.starts_with("0x") {
        return Err("Hash does not start with '0x'".to_string());
    }
    if hash.strip_prefix("0x").unwrap().len() != 64 {
        return Err("Hash length is not 64".to_string());
    }
    Ok(hash)
}