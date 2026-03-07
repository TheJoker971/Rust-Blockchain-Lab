use crate::models::Wallet;
use crate::models::{Transaction, TransactionStatus};

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
    let mut transaction = Transaction::new(&wallet, &wallet2, 100, 1);
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