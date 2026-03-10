use crate::models::{Wallet,Block,Transaction, TransactionStatus, Blockchain};
use chrono::Utc;

#[test]
fn test_wallet_new() {
    let wallet = Wallet::new();
    assert!(wallet.address.starts_with("w"),"Wallet address should start with 'w'");
    assert_eq!(wallet.balance, 0,"Wallet balance should be 0");
    let wallet2 = Wallet::new();
    assert_ne!(wallet2.address, wallet.address,"Wallet address should not be the same");
    assert_eq!(wallet2.balance, wallet.balance,"Wallet balance should be the same");
}

#[test]
fn test_transaction_new() {
    let wallet = Wallet::new();
    let wallet2 = Wallet::new();
    let transaction = Transaction::new(&wallet, &wallet2, 100, 1);
    assert_eq!(transaction.sender, wallet.address,"Transaction sender should be the wallet address");
    assert_eq!(transaction.recipient, wallet2.address,"Transaction recipient should be the wallet2 address");
    assert_eq!(transaction.amount, 100,"Transaction amount should be 100");
    assert_eq!(transaction.fee, 1,"Transaction fee should be 1");
    assert!(transaction.signature.is_empty(),"Transaction signature should be empty");
}

#[test]
fn test_switch_transaction_status() {
    let wallet = Wallet::new();
    let wallet2 = Wallet::new();
    let mut transaction = Transaction::new(&wallet, &wallet2, 100, 1);
    assert_eq!(transaction.status, TransactionStatus::Pending,"Transaction status should be Pending");
    switch_transaction_status(&mut transaction, TransactionStatus::Confirmed);
    assert_eq!(transaction.status, TransactionStatus::Confirmed,"Transaction status should be Confirmed");
    switch_transaction_status(&mut transaction, TransactionStatus::Failed);
    assert_eq!(transaction.status, TransactionStatus::Failed,"Transaction status should be Failed");
}

fn switch_transaction_status(transaction: &mut Transaction, status: TransactionStatus)  {
    transaction.status = status;
}

#[test]
fn test_block_new() {
    let block = Block::new(0,  "0".to_string());
    assert_eq!(block.index, 0,"Block index should be 0");
    assert!(block.timestamp <= Utc::now().timestamp(),"Block timestamp should be 0");
    assert_eq!(block.transactions.len(), 20,"Block transactions should not be empty");
    assert_eq!(block.previous_hash, "0","Block previous hash should be '0x000000...000000'");
    assert!(block.hash.is_empty(),"Block hash should be empty");
    assert_eq!(block.transaction_count, 0,"Block transaction count should be 0");
    assert!(!block.is_full(),"Block should not be full");
}

#[test]
fn test_block_genesis_block() {
    let block = Block::genesis_block();
    assert_eq!(block.index, 0,"Block index should be 0");
    assert!(block.timestamp <= Utc::now().timestamp(),"Block timestamp should be 0");
    assert_eq!(block.transactions.len(), 20,"Block transactions should not be empty");
    assert_eq!(block.previous_hash, format!("0x{}", "0".repeat(64)),"Block previous hash should be '0x000000...000000'");
    assert!(block.hash.is_empty(),"Block hash should be empty");
    assert_eq!(block.transaction_count, 0,"Block transaction count should be 0");
    assert!(!block.is_full(),"Block should not be full");
}

#[test]
fn test_block_add_transaction() {
    let mut block = Block::new(0,  format!("0x{}", "0".repeat(64))).unwrap();
    let transaction = Transaction::new(&Wallet::new(), &Wallet::new(), 100, 1);
    block.add_transaction(&transaction).unwrap();
    assert_eq!(block.transaction_count, 1,"Block transaction count should be 1");
    assert_eq!(block.transactions[0], transaction,"Block transaction should be the same");
}
#[test]
fn test_blockchain_new() {
    let blockchain = Blockchain::new();
    assert!(blockchain.blocks.is_empty(),"Blockchain blocks should be empty");
    assert!(blockchain.wallets.is_empty(),"Blockchain wallets should be empty");
}
