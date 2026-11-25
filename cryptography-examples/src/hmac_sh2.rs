use anyhow::Result;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

// Changing the hash function is a one-line change, for example to `Sha512`.
type HmacSha256 = Hmac<Sha256>;

fn main() -> Result<()> {
    let mut key_value = [0u8; 48];
    getrandom::fill(&mut key_value)?;

    let message = "Legitimate and important message.";
    let mut signer = HmacSha256::new_from_slice(&key_value)?;
    signer.update(message.as_bytes());
    let tag = signer.finalize().into_bytes();

    // `verify_slice` compares the tags in constant time.
    let mut verifier = HmacSha256::new_from_slice(&key_value)?;
    verifier.update(message.as_bytes());
    verifier.verify_slice(&tag)?;

    let mut tampered = HmacSha256::new_from_slice(&key_value)?;
    tampered.update(b"Tampered message.");
    assert!(tampered.verify_slice(&tag).is_err());

    Ok(())
}
