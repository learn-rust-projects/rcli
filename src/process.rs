use anyhow::Context;
use csv::Reader;
use serde::{Deserialize, Serialize};
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

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input).context("Failed to open input file")?;
    let mut vec = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Record = result.context("Failed to deserialize record")?;
        vec.push(record);
    }
    let json_data =
        serde_json::to_string_pretty(&vec).context("Failed to serialize records to JSON")?;
    std::fs::write(output, json_data).context("Failed to write output file")?;

    Ok(())
}
