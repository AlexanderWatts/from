use cli::Cli;
use std::io;

fn main() -> Result<(), io::Error> {
    Cli.run()
}
