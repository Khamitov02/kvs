use std::process::exit;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None )]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Commands::Get { key } => {
            eprintln!("unimplemented!");
            exit(1);
        }
        Commands::Set { key, value } => {
            eprintln!("unimplemented!");
            exit(1);
        }
        Commands::Rm { key } => {
            eprintln!("unimplemented!");
            exit(1);
        }
    }
}
