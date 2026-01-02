use clap::Parser;
use template::{Opts, SubCommand, process_csv};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.sub {
        SubCommand::Csv(csv_opts) => {
            process_csv(&csv_opts.input, &csv_opts.output)?;
            Ok(())
        }
    }
}
// 新增测试
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_csv() {
        let input = "test_data/input.csv";
        let output = "test_data/output.json";
        let result = process_csv(input, output);
        assert!(result.is_err());
    }
}
