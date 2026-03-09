use crate::models::Transaction;
use std::array;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: [Transaction; 20],
    pub transaction_count: usize,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String) -> Self {
        Self { 
            index, 
            timestamp: Utc::now().timestamp(), 
            transactions: array::from_fn(|_| Transaction::default()),
            transaction_count: 0,
            previous_hash, 
            hash: String::new() 
        }
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

}