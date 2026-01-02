use anyhow::Context;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::OutputFormat;
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    name: String,

    position: String,
    #[serde(rename = "DOB")]
    dob: String,

    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input).context("Failed to open input file")?;
    let mut vec = Vec::with_capacity(128);
    let headers = reader
        .headers()
        .context("Failed to read CSV headers")?
        .clone();

    for result in reader.records() {
        let record = result.context("Failed to deserialize record")?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        println!("{:?}", json_value);
        vec.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&vec)?,
        OutputFormat::Yaml => serde_yaml::to_string(&vec)?,
    };
    std::fs::write(output, content).context("Failed to write output file")?;

    Ok(())
}
