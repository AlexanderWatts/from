use clap::Parser as ClapParser;
use cli_args::CliArgs;
use code_generation::CodeGenerator;
use parser::Parser;
use std::{
    error::Error,
    fs,
    io::{self, Write, stderr, stdout},
};
use transformer::Transformer;
mod cli_args;

#[derive(ClapParser, Debug)]
pub struct Cli;

impl Cli {
    pub fn run(&self) -> Result<(), io::Error> {
        match self.process() {
            Ok(output) => writeln!(stdout(), "{}", output),
            Err(error) => writeln!(stderr(), "{}", error),
        }
    }

    fn process(&self) -> Result<String, Box<dyn Error>> {
        let CliArgs {
            input_path,
            output_path,
        } = CliArgs::parse();

        let input = fs::read_to_string(input_path)?;

        let proto = Parser::new(&input).parse()?;
        let transformer = Transformer;
        let estree = transformer.transform(&proto);
        let output = CodeGenerator::new().generate(&estree);

        let mut output_file = fs::File::create(output_path)?;
        output_file.write_all(output.as_bytes())?;

        Ok(output)
    }
}

#[cfg(test)]
mod cli_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn cli_args() {
        let output = CliArgs::parse_from(["", "-i", "./main.from"]);

        assert_eq!(
            CliArgs {
                input_path: PathBuf::from("./main.from"),
                output_path: PathBuf::from("./from.js"),
            },
            output
        )
    }
}
