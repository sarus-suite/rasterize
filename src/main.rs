use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

/// CLI tool for sarus-suite
#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(ValueEnum, Clone)]
enum FormatOutput {
    Text,
    Json,
}

#[derive(Subcommand)]
enum Command {
    /// Validate EDF file 
    Validate {
        filepath: String,
        #[arg(long, short, value_enum,default_value_t = FormatOutput::Text)]
        output: FormatOutput,
    },
    /// Render EDF file 
    Render {
        filepath: String,
        #[arg(long, short, value_enum,default_value_t = FormatOutput::Text)]
        output: FormatOutput,
    },
}

#[derive(Serialize, Deserialize, Clone)]
struct Out {
    stdout: String,
    stderr: String,
    return_code: i32,
}

fn printout(fout: FormatOutput, out: Out) {

    match fout {
        FormatOutput::Text => {
            let o = out.stdout.as_str();
            let e = out.stderr.as_str();

            if o != "" {
                println!("{o}");
            }
            if e != "" {
                eprintln!("{e}");
            }
        }
        FormatOutput::Json => {
            println!("{}", (
                    serde_json::to_string_pretty(&out)).
                        unwrap_or(String::from("{}"))
            );

        }
    }
}

fn validate(filepath: String, fout: FormatOutput) -> i32 {
    let mut out = Out {
        stdout: format!(""),
        stderr: format!(""),
        return_code: 0,
    };

    let ret = raster::validate(filepath.clone());

    match ret {
        Ok(_) => {
            out.stdout = format!("{filepath} is a valid EDF file");
        },
        Err(e) => {
            out.stdout = format!("{filepath} is an INVALID EDF file");
            out.stderr = format!("{e}");
            out.return_code = 1;
        }
    }
    printout(fout, out.clone());
    return out.return_code;
}

fn render(filepath: String, fout: FormatOutput) -> i32 {
    let mut out = Out {
        stdout: format!(""),
        stderr: format!(""),
        return_code: 0,
    };

    let ret = raster::render(filepath.clone());

    match ret {
        Ok(o) => {
            out.stdout = format!("{}", serde_json::to_string_pretty(&o).unwrap_or(String::from("ERROR")));
        },
        Err(e) => {
            out.stdout = format!("");
            out.stderr = format!("{e}");
            out.return_code = 1;
        }
    }
    printout(fout, out.clone());
    return out.return_code;
}

fn main() {
    let args = Args::parse();

    let rc = match args.command {
        Command::Validate { filepath, output } => validate(filepath, output),
        Command::Render { filepath, output } => render(filepath, output),
    };

    std::process::exit(rc);
}
