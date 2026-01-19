use std::path::PathBuf;

use strum::{Display, EnumString, IntoStaticStr};

use super::prelude::*;
#[derive(Parser, Debug)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServerOpts),
}

#[derive(Parser, Debug)]
pub struct HttpServerOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".", help = "Directory to serve")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080", help = "Port to listen on")]
    pub port: u16,
}
impl CmdExc for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => crate::process_http_serve(opts.dir, opts.port).await,
        }
    }
}

#[derive(Parser, Debug)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-", help = "Input file path")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short,long, default_value = "blake3",value_parser = value_parser)]
    pub format: TextSignFormat,
}

#[derive(Parser, Debug)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-", help = "Input file path")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = value_parser)]
    pub format: TextSignFormat,
}

#[derive(Parser, Debug)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3",value_parser = value_parser)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, Copy, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn value_parser(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse()
        .map_err(|e| anyhow::anyhow!("Invalid text sign format: {}", e))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_text_sign_format() {
        assert_eq!(TextSignFormat::Blake3.to_string(), "blake3");
        assert_eq!(TextSignFormat::Ed25519.to_string(), "ed25519");
    }
}
