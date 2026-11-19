use std::env;

// fn main() {
//     // If not set, we use a sensible default value
//     let mode = env::var("MODE").unwrap_or_else(|_| "INFO".to_string());
//     println!("{mode}")
// }

// fn main() {
//     // SAFETY: No other thread is currently manipulating the enironment
//     unsafe {
//         env::remove_var("MODE");
//     }
// }

#[derive(Debug)]
enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Trace,
}

// Handles conversion of string env variable to LogLevel enum.
impl TryFrom<String> for LogLevel {
    type Error = String;

    fn try_from(value: String) -> Result<Self, String> {
        let level = match value.to_lowercase().as_str() {
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "Warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            "trace" => LogLevel::Trace,
            other => return Err(format!("Unknown log level: {}", other)),
        };
        Ok(level)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Config {
    server_url: String,
    log_level: LogLevel,
    server_port: u16,
}

const DEFAULT_SERVER_URL: &str = "http://127.0.0.1";
const DEFAULT_SERVER_PORT: u16 = 8080_u16;
const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;

impl Config {
    fn load() -> Self {
        let server_url =
            env::var("BACKEND_SERVER_URL").unwrap_or_else(|_| DEFAULT_SERVER_URL.into());
        let server_port = env::var("BACKEND_SERVER_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(DEFAULT_SERVER_PORT);
        let log_level = env::var("BACKEND_LOG_LEVEL")
            .ok()
            .and_then(|v| v.try_into().ok())
            .unwrap_or(DEFAULT_LOG_LEVEL);

        Config {
            server_url,
            log_level,
            server_port,
        }
    }
}

fn main() {
    let cfg = Config::load();
    dbg!(cfg);
}
