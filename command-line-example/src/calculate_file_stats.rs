use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// Represents the arguments passed to this process during initialization
#[derive(Debug)]
struct CliArgs {
    lines: bool,
    words: bool,
    paths: Vec<PathBuf>,
}

impl CliArgs {
    fn parse() -> Self {
        let mut lines = false;
        let mut words = false;
        let mut paths = Vec::new();

        let args = env::args_os().skip(1);
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.to_str().unwrap_or_default() {
                "-l" | "--lines" => lines = true,
                "-w" | "--words" => words = true,
                "-p" | "--paths" => {
                    paths.extend(args.by_ref().map(PathBuf::from));
                    break;
                }
                _ => {}
            }
        }
        CliArgs {
            lines,
            words,
            paths,
        }
    }
}

// Represents the statistics of a given filepath
struct Stats<'p> {
    filepath: &'p Path,
    lines: Option<usize>,
    words: Option<usize>,
}

fn stat_files(cli_args: &CliArgs) -> Vec<Stats<'_>> {
    let mut stat_collection: Vec<Stats> = Vec::new();
    // If no flag is set, do no work
    if !cli_args.words && !cli_args.lines {
        return stat_collection;
    }

    for filepath in &cli_args.paths {
        // Skip anything that ain't a file
        if !filepath.is_file() {
            continue;
        }

        let Ok(content) = fs::read_to_string(filepath) else {
            continue;
        };

        let lines = match cli_args.lines {
            true => Some(content.lines().count()),
            false => None,
        };

        let words = match cli_args.words {
            true => Some(
                content
                    .lines()
                    .map(|line| line.split_whitespace().count())
                    .sum(),
            ),
            false => None,
        };

        let stats = Stats {
            filepath,
            lines,
            words,
        };
        stat_collection.push(stats);
    }
    stat_collection
}

// cargo run --release -- -l -w -p src/main.rs cargo.toml
// cargo run --release -- -l -p src/main.rs
fn main() {
    let cli_args = CliArgs::parse();
    let stats = stat_files(&cli_args);
    // Print individual file statistics
    for stat in stats {
        println!(
            "Path: {:?} {} {}",
            stat.filepath,
            if let Some(words) = stat.words {
                format!("Words: {}", words)
            } else {
                "".to_string()
            },
            if let Some(lines) = stat.lines {
                format!("Lines: {}", lines)
            } else {
                "".to_string()
            },
        );
    }
}
