use clap::Parser;
use template::{
    Base64Opts, Opts, SubCommand, gen_pass, process_base64_decode, process_base64_encode,
    process_csv,
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
            let _ = gen_pass(
                gen_pass_opts.length,
                gen_pass_opts.upper_case,
                gen_pass_opts.lower_case,
                gen_pass_opts.number,
                gen_pass_opts.symbol,
            );
            Ok(())
        }
        SubCommand::Base64(base64_opts) => match base64_opts {
            Base64Opts::EnCode(base64_encode_opts) => {
                process_base64_encode(&base64_encode_opts.input, base64_encode_opts.format)?;
                Ok(())
            }
            Base64Opts::DeCode(base64_decode_opts) => {
                process_base64_decode(&base64_decode_opts.input, base64_decode_opts.format)?;
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
