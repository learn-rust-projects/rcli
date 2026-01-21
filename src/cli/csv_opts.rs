use strum::{Display, EnumString, IntoStaticStr};

use super::prelude::*;
#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path",value_parser = verify_file,default_value = "./assets/juventus.csv")]
    pub input: String,

    #[arg(short, long, help = "Output file path")]
    pub output: Option<String>,

    #[arg(short, long, help = "Output format", default_value = "json",value_parser = parse_format)]
    pub format: OutputFormat,

    #[arg(short, long, help = "Convert to JSON format", default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, help = "CSV has header", default_value_t = true)]
    pub header: bool,
}

impl CmdExc for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = self
            .output
            .unwrap_or_else(|| format!("output.{}", self.format));
        let _ = crate::process_csv(&self.input, &output, self.format);
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, IntoStaticStr, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Csv,
    Yaml,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format
        .parse::<OutputFormat>()
        .map_err(|e| anyhow::anyhow!("Unsupported output format: {}", e))
}
