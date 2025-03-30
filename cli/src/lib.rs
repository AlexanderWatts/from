use std::{error::Error, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug, PartialEq)]
pub struct CliArgs {
    #[arg(short, long)]
    input_path: PathBuf,

    #[arg(short, long, default_value = "./")]
    output_path: PathBuf,
}

#[derive(Parser, Debug)]
pub struct Cli {}

impl Cli {
    pub fn run() -> Result<(), Box<dyn Error>> {
        let cli_args = CliArgs::parse();

        Ok(())
    }
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn cli_args() {
        let output = CliArgs::parse_from(["", "-i", "./main.from"]);

        assert_eq!(
            CliArgs {
                input_path: PathBuf::from("./main.from"),
                output_path: PathBuf::from("./"),
            },
            output
        )
    }
}
