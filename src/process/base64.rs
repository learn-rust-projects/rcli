use std::io::Read;

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    prelude::*,
};

use crate::{cli::Base64Format, trim_whitespace};
pub fn process_base64_encode(
    reader: &mut dyn Read,
    format: Base64Format,
) -> anyhow::Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    // 找到最后一个非空白字符的位置
    let buf = trim_whitespace(&buf);
    let encoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
        Base64Format::Standard => STANDARD.encode(buf),
    };
    Ok(encoded)
}
pub fn process_base64_decode(
    reader: &mut dyn Read,
    format: Base64Format,
) -> anyhow::Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
        Base64Format::Standard => STANDARD.decode(buf)?,
    };
    Ok(String::from_utf8(decoded)?)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::get_reader;
    #[test]
    fn test_process_base64_encode() -> anyhow::Result<()> {
        let input = "./fixtures/blake3.txt";
        let format = Base64Format::Standard;
        let mut read = get_reader(input)?;
        println!("{}", fs::read_to_string(input)?);
        let result = process_base64_encode(&mut read, format).unwrap();
        let encode = fs::read_to_string("./fixtures/encode.txt")?
            .trim_end_matches("\n")
            .to_string();
        assert_eq!(result, encode);
        Ok(())
    }
    #[test]
    fn test_process_base64_decode() -> anyhow::Result<()> {
        let input = "./fixtures/encode.txt";
        let format = Base64Format::Standard;
        let mut read = get_reader(input)?;
        let result = process_base64_decode(&mut read, format);
        // TODO: assume it decode data is string
        let decoded = result?;
        let expect: String = fs::read_to_string("./fixtures/blake3.txt")?
            .trim_end_matches("\n")
            .to_string();
        assert_eq!(decoded, expect);
        Ok(())
    }
}
