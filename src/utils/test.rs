use crate::utils::Keypair;


#[test]
fn test_keypair_generate() {
    let keypair = Keypair::generate();
    assert!(keypair.public_key.starts_with("w"), "Public key should start with 'w'");
    assert!(keypair.private_key.starts_with("pk"), "Private key should start with 'pk'");
}

#[test]
fn test_keypair_from_private_key() {
    let keypair = Keypair::generate();
    let keypair2 = Keypair::from_private_key(keypair.private_key.clone());
    assert_eq!(keypair2.public_key, keypair.public_key, "Public keys should be equal");
    assert_eq!(keypair2.private_key, keypair.private_key, "Private keys should be equal");
    assert!(!keypair2.private_key.starts_with("pkpk"), "Private key should not start with 'pkpk'");
}