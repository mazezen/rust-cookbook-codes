use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;
use std::{eprintln, println};

use anyhow::Result;
use config::{Config, File};
use notify::RecursiveMode;
use notify_debouncer_full::new_debouncer;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Settings {
    host: String,
    port: u16,
}

fn load(path: &Path) -> Result<Settings, config::ConfigError> {
    Config::builder()
        .add_source(File::from(path))
        .build()?
        .try_deserialize()
}

fn main() -> Result<()> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.toml");

    let mut settings = load(&path)?;
    println!("serving {}:{}", settings.host, settings.port);

    let (tx, rx) = mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx)?;
    debouncer.watch(&path, RecursiveMode::NonRecursive)?;

    println!(
        "watching {} for changes, Press Ctrl-C to stop.",
        path.display()
    );

    for result in rx {
        match result {
            Ok(_events) => match load(&path) {
                Ok(updated) => {
                    settings = updated;
                    println!("reloaded {}:{}", settings.host, settings.port);
                }
                Err(error) => eprintln!("ignoring invalid config: {error}"),
            },
            Err(errors) => {
                for error in errors {
                    eprintln!("watch error: {error:?}");
                }
            }
        }
    }

    Ok(())
}
