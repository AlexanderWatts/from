use std::{
    error::Error,
    fs,
    io::{self, Write, stderr, stdout},
    path::PathBuf,
};

use clap::Parser as ClapParser;
use code_generation::CodeGenerator;
use parser::Parser;
use transpiler::Transpiler;

#[derive(ClapParser, Debug, PartialEq)]
pub struct CliArgs {
    #[arg(short, long, value_parser = file_extension_validation)]
    input_path: PathBuf,

    #[arg(short, long, default_value = "./from.js")]
    output_path: PathBuf,
}

pub fn file_extension_validation(input_path: &str) -> Result<PathBuf, String> {
    let path_buf = PathBuf::from(input_path);

    match path_buf
        .extension()
        .and_then(|input_path| input_path.to_str())
    {
        Some("from") => Ok(path_buf),
        _ => Err(format!("Could not find or process {}", path_buf.display())),
    }
}

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
        let transpiler = Transpiler;
        let estree = transpiler.transpile(&proto);
        let output = CodeGenerator::new().generate(&estree);

        let mut output_file = fs::File::create(output_path)?;
        output_file.write_all(output.as_bytes())?;

        Ok(output)
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
