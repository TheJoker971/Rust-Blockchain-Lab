use crate::models::Wallet;

#[test]
fn test_wallet_new() {
    let wallet = Wallet::new();
    assert!(wallet.address.starts_with("w"),"Wallet address should start with 'w'");
    assert_eq!(wallet.balance, 0,"Wallet balance should be 0");
    let wallet2 = Wallet::new();
    assert_ne!(wallet2.address, wallet.address,"Wallet address should not be the same");
    assert_eq!(wallet2.balance, wallet.balance,"Wallet balance should be the same");
}
