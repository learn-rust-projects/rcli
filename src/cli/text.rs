use std::path::PathBuf;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use strum::{Display, EnumString, IntoStaticStr};

use super::prelude::*;
use crate::{get_reader, process_text_key_generate, process_text_verify, read_input};
#[derive(Parser, Debug)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a message with a public/shared key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key pair")]
    Generate(TextKeyGenerateOpts),
}
impl CmdExc for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(text_sign_opts) => {
                let mut reader = get_reader(&text_sign_opts.input)?;
                let key = read_input(&text_sign_opts.key)?;
                let sig = crate::process_text_sign(&mut reader, &key, text_sign_opts.format)?;
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
                Ok(())
            }
            TextSubCommand::Verify(text_verify_opts) => {
                let mut reader = get_reader(&text_verify_opts.input)?;
                let key = read_input(&text_verify_opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&text_verify_opts.sig)?;
                let verify =
                    process_text_verify(&mut reader, &key, &decoded, text_verify_opts.format)?;
                if verify {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
                Ok(())
            }
            TextSubCommand::Generate(text_key_generate_opts) => {
                let key = process_text_key_generate(text_key_generate_opts.format)?;
                for (k, v) in key {
                    std::fs::write(text_key_generate_opts.output_path.join(k), v)?;
                }
                Ok(())
            }
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
