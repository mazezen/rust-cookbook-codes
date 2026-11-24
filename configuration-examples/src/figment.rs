use figment::providers::{Env, Format, Toml};
use figment::Figment;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    port: u16,
    workers: usize,
}

fn figment() -> Figment {
    Figment::new()
        .merge(Toml::file("APP.toml"))
        .merge(Env::prefixed("APP_"))
}

fn main() -> Result<(), figment::Error> {
    let config: Config = figment().extract()?;
    println!(
        "{} listening on port {} with {} workers",
        config.name, config.port, config.workers
    );
    Ok(())
}
