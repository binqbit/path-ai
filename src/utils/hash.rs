use sha2::{Sha256, Digest};



pub fn hashing(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    base64::encode(hasher.finalize())
}