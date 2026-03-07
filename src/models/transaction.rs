use crate::models::Wallet;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}
impl TransactionStatus {
    pub fn eq(&self, other: &TransactionStatus) -> bool {
        self == other
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub fee: u64,
    pub signature: String,
    pub status: TransactionStatus,
}

impl Transaction {
    pub fn new(sender: &Wallet, recipient: &Wallet, amount: u64, fee: u64) -> Self {
        Self { sender: sender.address.clone(), recipient: recipient.address.clone(), amount, fee, signature: String::new(), status: TransactionStatus::Pending }
    }
}