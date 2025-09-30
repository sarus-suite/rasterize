use raster::validate;
use clap::{Parser, Subcommand};

/// CLI tool for sarus-suite
#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate EDF file 
    Validate {
        filepath: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Validate { filepath } => validate(filepath),
    }
}
