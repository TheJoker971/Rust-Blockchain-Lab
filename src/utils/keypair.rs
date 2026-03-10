use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;
use hex;

#[derive(Debug, Clone)]
pub struct Keypair {
    pub public_key: String,
    pub private_key: String,
}

impl Keypair {
    pub fn generate() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        Self {
            public_key: format!("w{}", hex::encode(public_key.serialize())),
            private_key: format!("p{}", hex::encode(secret_key.secret_bytes())),
        }
    }
    pub fn from_private_key(private_key: String) -> Self {
        let key_str = if private_key.starts_with("pk") {
            private_key.strip_prefix("p").unwrap().to_string()
        } else {
            private_key.clone()
        };
        
        let secp = Secp256k1::new();
        let private_key_bytes = hex::decode(&key_str)
            .expect("Invalid hex string for private key");
        let secret_key = SecretKey::from_slice(&private_key_bytes)
            .expect("Invalid private key bytes");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Self {
            public_key: format!("w{}", hex::encode(public_key.serialize())),
            private_key: if private_key.starts_with("p") {
                private_key
            } else {
                format!("p{}", private_key)
            },
        }
    }
}