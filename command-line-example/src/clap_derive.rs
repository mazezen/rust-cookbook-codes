use clap::{Parser, ValueEnum};
use std::net::IpAddr;

#[derive(ValueEnum, Clone, Debug)]
enum LogLevel {
    Debug,
    Error,
    Warn,
    Trace,
    Info,
}

#[derive(Parser, Clone, Debug)]
struct CliArgs {
    /// Logging threshold
    ///
    ///
    #[arg(short = 'l', long = "log-level")]
    log_level: Option<LogLevel>,

    /// The Backend Server Address
    #[arg(short = 'a', long = "server-address")]
    server_address: IpAddr,

    /// The Backend Server Port
    #[arg(short = 'p', long = "server-port")]
    server_port: Option<u16>,

    /// Interval between database snapshots in seconds
    #[arg(short = 's', long = "snapshot-freq")]
    snapshot_frequency: Option<usize>,
}

fn main() {
    let args = CliArgs::parse();
    dbg!(args);
}
