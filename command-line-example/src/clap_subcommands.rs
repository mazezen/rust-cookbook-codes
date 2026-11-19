use std::println;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct NotesArgs {
    /// Enable syntax highlighting
    #[arg(global = true, short = 'c', long = "color")]
    color: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new note
    New {
        #[arg(short = 's', long = "title", help = "Title of the new note")]
        title: String,
        #[arg(short = 'b', long = "body", help = "Description of the note")]
        body: Option<String>,
    },

    /// list all available notes
    List,

    // Delete all available notes
    Delete {
        /// Index of the note
        index: u64,
    },

    /// Update an existing note
    Update {
        /// Index of the note
        index: u64,

        #[arg(short = 't', long = "title", help = "Updated title of the note")]
        title: Option<String>,

        #[arg(short = 'b', long = "body", help = "Updated description of the note")]
        body: Option<String>,
    },
}

fn main() {
    let args = NotesArgs::parse();
    if args.color {
        print!("Enable syntax highlighting");
    }

    match args.command {
        Commands::New { title, body } => {
            println!(
                "Creating new note!\nTITLE: {}\n{}",
                title,
                if let Some(body) = body {
                    format!("BODY: {}", body)
                } else {
                    "".into()
                }
            );
        }
        Commands::List => println!("Listing available tasks"),
        Commands::Delete { index } => println!("Deleting note {index}"),
        Commands::Update { index, title, body } => {
            println!("Updating note {index}");
            if let Some(title) = title {
                println!("NEW TITLE: {title}");
            }
            if let Some(body) = body {
                println!("NEW BODY: {body}");
            }
        }
    }
}
