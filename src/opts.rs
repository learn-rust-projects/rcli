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

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path",value_parser = verify_input_file)]
    pub input: String,
    //
    #[arg(short, long, help = "Output file path", default_value = "output.json")]
    pub output: String,

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
