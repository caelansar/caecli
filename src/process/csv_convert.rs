use crate::cmd::OutputFormat;
use anyhow::Result;
use csv::Reader;
use std::fs;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let json_value = headers
            .iter()
            .zip(result?.iter())
            .collect::<serde_json::Value>();

        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
