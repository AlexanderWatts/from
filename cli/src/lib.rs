use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug, PartialEq)]
pub struct CliArgs {}

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
        let output = CliArgs::parse_from([""]);

        assert_eq!(CliArgs {}, output)
    }
}
