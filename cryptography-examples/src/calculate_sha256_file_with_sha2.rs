use anyhow::Result;
use data_encoding::HEXLOWER;
use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::println;

fn digest_reader<D: Digest, R: Read>(mut reader: R) -> Result<Vec<u8>> {
    let mut hasher = D::new();
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(hasher.finalize().to_vec())
}

fn main() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("file.txt");

    let mut output = File::create(&path)?;
    write!(output, "We will generate a digest of this text")?;

    let sha256 = digest_reader::<Sha256, _>(BufReader::new(File::open(&path)?))?;
    println!("SHA-256 digest is {}", HEXLOWER.encode(&sha256));

    let sha512 = digest_reader::<Sha512, _>(BufReader::new(File::open(&path)?))?;
    println!("SHA-512 digest is {}", HEXLOWER.encode(&sha512));

    Ok(())
}
