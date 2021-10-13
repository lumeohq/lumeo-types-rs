#![warn(
    rust_2018_idioms,
    unused_qualifications,
    clippy::cloned_instead_of_copied,
    clippy::dbg_macro,
    clippy::str_to_string,
    clippy::todo,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix,
    clippy::wildcard_imports
)]

use std::{
    error::Error,
    io::{self, Read},
    process,
};

use clap::Clap;
use lumeo_pipeline::Pipeline;

#[derive(Clap)]
struct Options {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Checks whether a JSON pipeline definition can be deserialized correctly (reads from stdin)
    CheckPipeline,
}

fn main() {
    let opt = Options::parse();
    match opt.subcmd {
        SubCommand::CheckPipeline => {
            if let Err(e) = check_pipeline() {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }
}

fn check_pipeline() -> Result<(), Box<dyn Error + 'static>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    match serde_json::from_str::<Pipeline>(&buf) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprint!("Deserialization failed: ");
            Err(e.into())
        }
    }
}
