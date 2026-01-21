use anyhow::Context;
use convert_case::{Case, Casing};
use csv::{Reader, StringRecord};
use erased_serde::Serialize as ErasedSerialize;
use serde::{Deserialize, Serialize};

use crate::cli::OutputFormat;
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// unuse struct Record
pub struct Record {
    name: String,

    position: String,
    #[serde(rename(serialize = "Dob", deserialize = "DOB"))]
    dob: String,

    nationality: String,
    #[serde(rename(serialize = "Kit", deserialize = "Kit Number"))]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input).context("Failed to open input file")?;
    let headers = reader.headers()?.clone();

    let record = StringRecord::from(vec!["Name", "Position", "DOB", "Nationality", "Kit Number"]);
    let match_record = record.eq(&headers);
    let ret: Box<dyn ErasedSerialize> = match match_record {
        true => {
            let mut vec = Vec::with_capacity(128);
            // deserialize records
            for result in reader.deserialize() {
                let record: Record = result.context("Failed to deserialize record")?;
                // let json_value = headers.iter().zip(record.iter()).collect::<Value>();
                vec.push(record);
            }
            Box::new(vec)
        }
        false => {
            let mut ret = Vec::with_capacity(128);
            for result in reader.records() {
                let record = result?;
                let json_value = headers
                    .iter()
                    .zip(record.iter())
                    .collect::<serde_json::Value>();

                ret.push(json_value);
            }
            Box::new(ret)
        }
    };

    let content: Box<dyn AsRef<[u8]>> = match format {
        OutputFormat::Json => Box::new(serde_json::to_string_pretty(&ret)?),
        OutputFormat::Yaml => Box::new(serde_yaml::to_string(&ret)?),
        OutputFormat::Csv => {
            let buffer = vec![];
            let mut wtr = csv::WriterBuilder::new()
                .has_headers(match_record)
                .from_writer(buffer);
            reader = Reader::from_path(input).context("Failed to open input file")?;
            match match_record {
                true => {
                    for result in reader.deserialize() {
                        let record: Record = result.context("Failed to deserialize record")?;
                        // let json_value = headers.iter().zip(record.iter()).collect::<Value>();
                        wtr.serialize(record)?;
                    }
                }
                false => {
                    let upper_headers: Vec<String> =
                        headers.iter().map(|h| h.to_case(Case::Pascal)).collect();
                    wtr.write_record(&upper_headers)?;
                    for result in reader.records() {
                        let record = result?; // StringRecord
                        wtr.write_record(&record)?;
                    }
                }
            }
            wtr.flush()?;
            Box::new(wtr.into_inner()?)
        }
    };

    std::fs::write(output, content.as_ref()).context("Failed to write output file")?;

    Ok(())
}
