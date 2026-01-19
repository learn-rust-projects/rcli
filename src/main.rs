use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use clap::Parser;
use template::{
    Base64SubCommand, Opts, SubCommand, TextSubCommand, gen_pass, get_reader,
    process_base64_decode, process_base64_encode, process_csv, process_text_key_generate,
    process_text_sign, process_text_verify, read_input,
};
fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.sub {
        SubCommand::Csv(csv_opts) => {
            let output = csv_opts
                .output
                .unwrap_or_else(|| format!("output.{}", csv_opts.format));
            process_csv(&csv_opts.input, &output, csv_opts.format)?;
            Ok(())
        }
        SubCommand::GenPass(gen_pass_opts) => {
            let result = gen_pass(
                gen_pass_opts.length,
                gen_pass_opts.upper_case,
                gen_pass_opts.lower_case,
                gen_pass_opts.number,
                gen_pass_opts.symbol,
            );
            let strength = zxcvbn::zxcvbn(&result?, &[]);
            eprintln!("Password score: {:?}", strength.score());
            Ok(())
        }
        SubCommand::Base64(base64_opts) => match base64_opts {
            Base64SubCommand::EnCode(base64_encode_opts) => {
                let mut reader = get_reader(&base64_encode_opts.input)?;
                let encoded = process_base64_encode(&mut reader, base64_encode_opts.format)?;
                println!("{}", encoded);
                Ok(())
            }
            Base64SubCommand::DeCode(base64_decode_opts) => {
                let mut reader = get_reader(&base64_decode_opts.input)?;
                let decoded = process_base64_decode(&mut reader, base64_decode_opts.format)?;
                println!("{}", decoded);
                Ok(())
            }
        },
        SubCommand::Text(text_opts) => match text_opts {
            TextSubCommand::Sign(text_sign_opts) => {
                let mut reader = get_reader(&text_sign_opts.input)?;
                let key = read_input(&text_sign_opts.key)?;
                let sig = process_text_sign(&mut reader, &key, text_sign_opts.format)?;
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
        },
    }
}
// 新增测试
#[cfg(test)]
mod tests {
    use template::OutputFormat;

    use super::*;
    #[test]
    fn test_process_csv() {
        let input = "test_data/input.csv";
        let output = "test_data/output.json";
        let result = process_csv(input, output, OutputFormat::Json);
        assert!(result.is_err());
    }
}
