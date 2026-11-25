use argon2::Argon2;
use argon2::password_hash::phc::PasswordHash;
use argon2::password_hash::{PasswordHasher, PasswordVerifier};

fn main() -> Result<(), argon2::password_hash::Error> {
    let password = "Guess Me If You Can!";

    let hash = Argon2::default()
        .hash_password(password.as_bytes())?
        .to_string();
    println!("Argon2 hash: {hash}");

    let parsed = PasswordHash::new(&hash)?;
    assert!(
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    );
    assert!(
        Argon2::default()
            .verify_password(b"Definitely not the correct password", &parsed)
            .is_err()
    );

    Ok(())
}
