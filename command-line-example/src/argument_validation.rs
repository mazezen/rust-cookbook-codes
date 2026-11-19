use clap::{Parser, value_parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct CliArgs {
    #[arg(short = 'p', long = "path")]
    path: PathBuf,

    #[arg(short = 't', long = "threshold", value_parser = value_parser!(u8).range(0..=10))]
    threshold: u8,

    #[arg(short = 'n', long = "notify", value_parser = parse_email)]
    notify: String,
}

fn parse_email(s: &str) -> Result<String, String> {
    match s.split_once('@') {
        Some((user, domain)) if !user.is_empty() && domain.contains('.') => Ok(s.to_string()),
        _ => Err("Invalid email format".to_string()),
    }
}

fn parse_config(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.is_file() {
        Ok(path)
    } else {
        Err("Provided config path is not a file".to_string())
    }
}

fn main() {
    let args = CliArgs::parse();
    dbg!(args);
}
