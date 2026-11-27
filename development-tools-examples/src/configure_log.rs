// mod foo {
//     mod bar {
//         pub fn run() {
//             log::warn!("[bar] warn");
//             log::info!("[bar] info");
//             log::debug!("[bar] debug");
//         }
//     }

//     pub fn run() {
//         log::warn!("[foo] warn");
//         log::info!("[foo] info");
//         log::debug!("[foo] debug");
//         bar::run();
//     }
// }

// fn main() {
//     env_logger::init();
//     log::warn!("[root] warn");
//     log::info!("[root] info");
//     log::debug!("[root] debug");
//     foo::run();
// }

// use chrono::Local;
// use env_logger::Builder;
// use log::LevelFilter;
// use std::{io::Write, writeln};

// fn main() {
//     Builder::new()
//         .format(|buf, record| {
//             writeln!(
//                 buf,
//                 "{} [{}] - {}",
//                 Local::now().format("%Y-%m-%dT%H:%M:%S"),
//                 record.level(),
//                 record.args()
//             )
//         })
//         .filter(None, LevelFilter::Info)
//         .init();

//     log::warn!("warn");
//     log::info!("info");
//     log::debug!("debug");
// }

use anyhow::Result;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn main() -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    log::info!("Hello, world!");

    Ok(())
}
