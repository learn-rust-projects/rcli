use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    prelude::*,
};

use crate::cli::Base64Format;
pub fn process_base64_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let buf = read_input(input)?;
    let encoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
        Base64Format::Standard => STANDARD.encode(&buf),
    };
    println!("{}", encoded);
    Ok(())
}
pub fn process_base64_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut buf = read_input(input)?;
    while matches!(buf.last(), Some(b'\n' | b'\r')) {
        buf.pop();
    }
    let decoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf)?,
        Base64Format::Standard => STANDARD.decode(&buf)?,
    };

    // TODO: assume itdecode data is string
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}
fn read_input(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader: Box<dyn std::io::Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_base64_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        let result = process_base64_encode(input, format);
        assert!(result.is_ok());
    }
    #[test]
    fn test_process_base64_decode() {
        let input = "fixtures/encode.txt";
        let format = Base64Format::Standard;
        let result = process_base64_decode(input, format);
        assert!(result.is_ok());
    }
}
