use std::io;
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    // create file
    File::create("data.txt").await?;

    // create a single directory
    tokio::fs::create_dir("my_dir").await?;

    // create directory and missing parents
    tokio::fs::create_dir_all("my_dir/sub_dir/inner").await?;

    Ok(())
}
