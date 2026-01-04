mod base64;
mod csv_opts;
mod gen_pass;
mod prelude;
use std::path::Path;

pub use base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64Opts};
use clap::Parser;
pub use csv_opts::{CsvOpts, OutputFormat};
pub use gen_pass::GenPassOpts;
#[derive(Parser, Debug)]
#[command(name = "rcli", version = "1.0", about = "An example CLI app",long_about = None)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show Csv ,or convert Csv to others formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassOpts),
    #[clap(subcommand)]
    Base64(Base64Opts),
}

pub fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        // roData编译的时候字面量就会编译到程序里面，生命周期和程序一样长
        Err("Input file does not exist.")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".to_string()));
        assert_eq!(verify_input_file(""), Err("Input file does not exist."));
        assert_eq!(
            verify_input_file(".gitignore"),
            Ok(".gitignore".to_string())
        );
    }
}
