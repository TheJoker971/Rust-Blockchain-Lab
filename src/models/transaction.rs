use crate::models::Wallet;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub fee: u64,
    pub signature: String,
    pub status: TransactionStatus,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            sender: String::new(),
            recipient: String::new(),
            amount: 0,
            fee: 0,
            signature: String::new(),
            status: TransactionStatus::Pending,
        }
    }
}

impl Transaction {
    pub fn new(sender: &Wallet, recipient: &Wallet, amount: u64, fee: u64) -> Self {
        Self { sender: sender.address.clone(), recipient: recipient.address.clone(), amount, fee, signature: String::new(), status: TransactionStatus::Pending }
    }
    
    pub fn is_empty(&self) -> bool {
        self.sender.is_empty() && self.recipient.is_empty() && self.amount == 0
    }
}