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

    let rc = match args.command {
        Command::Validate { filepath } => validate(filepath),
    };

    match rc {
        true => std::process::exit(0),
        false => std::process::exit(1),
    }
}
