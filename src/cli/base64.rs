use std::str::FromStr;

use super::prelude::*;
#[derive(Parser, Debug)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Base64 encode")]
    EnCode(Base64EncodeOpts),
    #[command(name = "decode", about = "Base64 decode")]
    DeCode(Base64DecodeOpts),
}
impl CmdExc for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::EnCode(base64_encode_opts) => {
                let mut reader = crate::get_reader(&base64_encode_opts.input)?;
                let encoded = crate::process_base64_encode(&mut reader, base64_encode_opts.format)?;
                println!("{}", encoded);
                Ok(())
            }
            Base64SubCommand::DeCode(base64_decode_opts) => {
                let mut reader = crate::get_reader(&base64_decode_opts.input)?;
                let decoded = crate::process_base64_decode(&mut reader, base64_decode_opts.format)?;
                println!("{}", decoded);
                Ok(())
            }
        }
    }
}
#[derive(Parser, Debug)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-", help = "Input file path")]
    pub input: String,
    #[arg(short, long, help = "Base64 format",value_parser = parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-", help = "Input file path")]
    pub input: String,
    #[arg(short, long, help = "Base64 format",value_parser = parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(s: &str) -> Result<Base64Format, anyhow::Error> {
    s.parse()
}
impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => anyhow::bail!("Invalid base64 format"),
        }
    }
}
impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl std::fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
