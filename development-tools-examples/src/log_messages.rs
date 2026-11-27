// use env_logger::{Builder, Target};

// fn execute_query(query: &str) {
//     log::debug!("Executing query: {}", query);
// }

// fn execute_query2(_query: &str) -> Result<(), &'static str> {
//     Err("I'm afraid I can't do that")
// }

use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn main() -> Result<(), SetLoggerError> {
    // env_logger::init();
    // execute_query("DROP TABLE students");

    // let response = execute_query2("DROP TABLE students");
    // if let Err(err) = response {
    //     log::error!("Failed to execute query: {}", err);
    // }

    // Builder::new().target(Target::Stdout).init();
    // log::error!("This error has been printed to Stdout");

    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    log::info!("Hello log");
    log::warn!("warning");
    log::error!("oops");

    Ok(())
}
