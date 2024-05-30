use std::fs;
use anyhow::{Error, Ok};
use csv::Reader;
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
struct Record{
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="Position")]
    position: String,
    #[serde(rename="DOB")]
    dob: String,
    #[serde(rename="Nationality")]
    nationality: String,
    #[serde(rename="Kit Number")]
    kit: u8,
}


pub fn process_csv(input: &str, output: &str) -> Result<(), Error> {
    let mut reader = Reader::from_path(input)?;
            let record = reader
                .deserialize()
                .map(|record| record.unwrap())
                .collect::<Vec<Record>>(); 
            let json = serde_json::to_string_pretty(&record)?;
            fs::write(output, json)?;
    Ok(())
}