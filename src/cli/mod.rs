mod base64;
mod csv_opts;
mod gen_pass;
mod prelude;
mod text;
use std::path::{Path, PathBuf};

pub use base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand};
use clap::Parser;
pub use csv_opts::{CsvOpts, OutputFormat};
pub use gen_pass::GenPassOpts;
pub use text::{TextSignFormat, TextSubCommand};
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
    Base64(Base64SubCommand),
    #[clap(subcommand)]
    Text(TextSubCommand),
}

pub fn verify_file(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        // roData编译的时候字面量就会编译到程序里面，生命周期和程序一样长
        Err("Input file does not exist.")
    }
}
pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    // if input is "-" or file exists
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("Input file does not exist."));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("Input file does not exist."));
    }
}
