use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rcli", version = "1.0", about = "An example CLI app",long_about = None)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show Csv ,or convert Csv to others formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path",value_parser = verify_input_file)]
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

fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        // roData编译的时候字面量就会编译到程序里面，生命周期和程序一样长
        Err("Input file does not exist.")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}
impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => anyhow::bail!("Unsupported output format: {}", format),
        }
    }
}
