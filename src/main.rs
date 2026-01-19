use clap::Parser;
use template::{CmdExc, Opts};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let opts: Opts = Opts::parse();
    opts.sub.execute().await?;
    Ok(())
}
// 新增测试
#[cfg(test)]
mod tests {
    use template::{OutputFormat, process_csv};
    #[test]
    fn test_process_csv() {
        let input = "test_data/input.csv";
        let output = "test_data/output.json";
        let result = process_csv(input, output, OutputFormat::Json);
        assert!(result.is_err());
    }
}
