use clap::Parser;
use template::{Opts, SubCommand, process_csv};

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
