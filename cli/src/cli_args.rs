use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, PartialEq)]
pub struct CliArgs {
    #[arg(short, long, value_parser = file_extension_validation)]
    pub input_path: PathBuf,

    #[arg(short, long, default_value = "./from.js")]
    pub output_path: PathBuf,
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
