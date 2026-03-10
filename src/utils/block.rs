pub type BlockHash = String;

pub fn validate_hash_block(hash:String) -> Result<BlockHash, String> {
    if !hash.starts_with("0x") {
        return Err("Hash does not start with '0x'".to_string());
    }
    if hash.strip_prefix("0x").unwrap().len() != 64 {
        return Err("Hash length is not 64".to_string());
    }
    Ok(hash)
}