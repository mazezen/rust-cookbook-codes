use std::{io, println};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn process_data(data: &[u8]) {
    println!("Data length: {}", data.len());
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // write file
    write_file().await?;

    // read to a Vec<u8>
    let content = tokio::fs::read("data.txt").await?;
    println!("content: {:?}", content);

    // read to a string
    let str_content = tokio::fs::read_to_string("data.txt").await?;
    process_data(str_content.as_bytes());
    println!("str_content: {:?}", str_content);

    // remove
    remove().await?;

    // AsyncRead and AsyncWrite
    async_read_async_write().await?;

    Ok(())
}

async fn write_file() -> io::Result<()> {
    let content = b"Generic data from program!";
    tokio::fs::write("data.txt", content).await?;

    Ok(())
}

async fn remove() -> io::Result<()> {
    // remove a file
    tokio::fs::remove_file("data.txt").await?;

    // remove an empty directory
    tokio::fs::remove_dir("my_dir").await?;

    // remove a directory and all its contents
    tokio::fs::remove_dir_all("my_dir_all").await?;

    Ok(())
}

async fn async_read_async_write() -> io::Result<()> {
    // AsyncWrite: write bytes to a file
    let mut file = File::create("bb.txt").await?;
    file.write_all(b"Generic Data!").await?;

    // AsyncRead: read bytes from a file
    let mut file: File = File::open("bb.txt").await?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await?;
    println!("Data Length: {}", contents.len());

    // stream from reader to writer
    let mut reader = File::open("bb.txt").await?;
    let mut writer = File::create("bb_copy.txt").await?;
    tokio::io::copy(&mut reader, &mut writer).await?;

    Ok(())
}
