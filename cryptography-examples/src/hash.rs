use std::println;

use data_encoding::HEXLOWER;
use sha2::digest::DynDigest;
use sha2::{Sha256, Sha384, Sha512};

fn hasher_by_name(name: &str) -> Option<Box<dyn DynDigest>> {
    match name {
        "sha256" => Some(Box::new(Sha256::default())),
        "sha384" => Some(Box::new(Sha384::default())),
        "sha512" => Some(Box::new(Sha512::default())),
        _ => None,
    }
}

fn main() {
    for name in ["sha256", "sha384", "sha512", "md5"] {
        match hasher_by_name(name) {
            Some(mut hasher) => {
                hasher.update(b"hello ");
                hasher.update(b"world");
                println!("{name}: {}", HEXLOWER.encode(&hasher.finalize()));
            }
            None => println!("{name}: unsupported algorithm"),
        }
    }
}
