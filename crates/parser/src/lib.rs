mod bytes;
pub mod card_parser;
pub mod detector;
mod dt;
pub mod vu_parser;
use anyhow::{Context, Result};
use card_parser::CardParser;
use detector::TachoFileType;
#[cfg(feature = "ts")]
use ts_rs::TS;
use vu_parser::VuParser;

// Vehicle Unit
pub fn parse_vu_from_file(file_path: &str) -> Result<vu_parser::VuData> {
    let output = VuParser::new_from_file(file_path)
        .context("Failed to create VuParser")?
        .parse();
    output
}
pub fn parse_vu_from_file_to_json(file_path: &str) -> Result<String> {
    let vu_data_json = VuParser::new_from_file(file_path)
        .context("Failed to create VuParser")?
        .parse_to_json()?;
    Ok(vu_data_json)
}

pub fn parse_vu_from_file_to_json_pretty(file_path: &str) -> Result<String> {
    let vu_data_json = VuParser::new_from_file(file_path)
        .context("Failed to create VuParser")?
        .parse_to_json_pretty()?;
    Ok(vu_data_json)
}

pub fn parse_vu_from_bytes(bytes: &[u8]) -> Result<vu_parser::VuData> {
    let output = VuParser::new_from_bytes(bytes)
        .context("Failed to create VuParser")?
        .parse();
    output
}
pub fn parse_vu_from_bytes_to_json(bytes: &[u8]) -> Result<String> {
    let vu_data_json = VuParser::new_from_bytes(bytes)
        .context("Failed to create VuParser")?
        .parse_to_json()?;
    Ok(vu_data_json)
}

pub fn parse_vu_from_bytes_to_json_pretty(bytes: &[u8]) -> Result<String> {
    let vu_data_json = VuParser::new_from_bytes(bytes)
        .context("Failed to create VuParser")?
        .parse_to_json_pretty()?;
    Ok(vu_data_json)
}

// Card
pub fn parse_card_from_file(file_path: &str) -> Result<card_parser::CardData> {
    let output = CardParser::new_from_file(file_path)
        .context("Failed to create CardParser")?
        .parse();
    output
}
pub fn parse_card_from_file_to_json(file_path: &str) -> Result<String> {
    let card_data_json = CardParser::new_from_file(file_path)
        .context("Failed to create CardParser")?
        .parse_to_json()?;
    Ok(card_data_json)
}
pub fn parse_card_from_file_to_json_pretty(file_path: &str) -> Result<String> {
    let card_data_json = CardParser::new_from_file(file_path)
        .context("Failed to create CardParser")?
        .parse_to_json_pretty()?;
    Ok(card_data_json)
}

pub fn parse_card_from_bytes(bytes: &[u8]) -> Result<card_parser::CardData> {
    let output = CardParser::new_from_bytes(bytes)
        .context("Failed to create CardParser")?
        .parse();
    output
}
pub fn parse_card_from_bytes_to_json(bytes: &[u8]) -> Result<String> {
    let card_data_json = CardParser::new_from_bytes(bytes)
        .context("Failed to create CardParser")?
        .parse_to_json()?;
    Ok(card_data_json)
}
pub fn parse_card_from_bytes_to_json_pretty(bytes: &[u8]) -> Result<String> {
    let card_data_json = CardParser::new_from_bytes(bytes)
        .context("Failed to create CardParser")?
        .parse_to_json_pretty()?;
    Ok(card_data_json)
}

#[derive(Debug)]
#[cfg_attr(feature = "ts", derive(TS))]
pub enum TachoData {
    Card { card_data: card_parser::CardData },
    Vu { vu_data: vu_parser::VuData },
}

pub fn parse_from_bytes(bytes: &[u8]) -> Result<TachoData> {
    let detected_file_type =
        detector::detect_from_bytes(bytes).expect("Failed to detect file type");
    let output = match detected_file_type {
        TachoFileType::DriverCardGen1
        | TachoFileType::DriverCardGen2
        | TachoFileType::DriverCardGen2V2 => {
            let card_data = CardParser::new_from_bytes(bytes)
                .context("Failed to create CardParser")?
                .parse()?;
            TachoData::Card { card_data }
        }
        TachoFileType::VehicleUnitGen1
        | TachoFileType::VehicleUnitGen2
        | TachoFileType::VehicleUnitGen2V2 => {
            let vu_data = VuParser::new_from_bytes(bytes)
                .context("Failed to create VuParser")?
                .parse()?;
            TachoData::Vu { vu_data }
        }
    };
    Ok(output)
}

#[cfg(test)]
mod tests {
    use detector::detect_from_file;
    use serde_json;

    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_process_card_file() {
        let data_dir = Path::new("../../data/ddd");
        let output_dir = Path::new("../../data/json");
        assert!(data_dir.exists(), "Data directory does not exist");
        fs::create_dir_all(output_dir).expect("Failed to create output directory");

        let mut files_processed = 0;

        if let Ok(entries) = fs::read_dir(data_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name.starts_with("C_")
                        && (file_name.ends_with(".ddd") || file_name.ends_with(".DDD"))
                    {
                        match parse_card_from_file(path.to_str().unwrap()) {
                            Ok(card_data) => {
                                println!("Successfully parsed file: {}", path.display());

                                // Create output JSON file path
                                let json_file_name =
                                    file_name.replace(".ddd", ".json").replace(".DDD", ".json");
                                let json_path = output_dir.join(json_file_name);

                                // Serialize and write JSON to file
                                let json = serde_json::to_string_pretty(&card_data)
                                    .expect("Failed to serialize to JSON");
                                fs::write(&json_path, json).expect("Failed to write JSON file");

                                println!("JSON output written to: {}", json_path.display());
                                files_processed += 1;
                            }
                            Err(e) => {
                                println!("Failed to parse file: {}", path.display());
                                eprintln!("Error: {:#}", e);

                                let file_type = detect_from_file(path.to_str().unwrap());
                                println!("detected file: {:#}", file_type.unwrap());

                                panic!("Error occurred while parsing");
                            }
                        }
                    }
                }
            }
        }

        println!("Files processed: {}", files_processed);

        assert!(files_processed > 0, "No files were successfully processed");
    }

    #[test]
    fn test_process_vu_file() {
        let data_dir = Path::new("../../data/ddd");
        let output_dir = Path::new("../../data/json");
        assert!(data_dir.exists(), "Data directory does not exist");
        fs::create_dir_all(output_dir).expect("Failed to create output directory");

        let mut files_processed = 0;

        if let Ok(entries) = fs::read_dir(data_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name.starts_with("M_")
                        && (file_name.ends_with(".ddd") || file_name.ends_with(".DDD"))
                    {
                        match parse_vu_from_file(path.to_str().unwrap()) {
                            Ok(vu_data) => {
                                println!("Successfully parsed file: {}", path.display());

                                // Create output JSON file path
                                let json_file_name =
                                    file_name.replace(".ddd", ".json").replace(".DDD", ".json");
                                let json_path = output_dir.join(json_file_name);

                                // Serialize and write JSON to file
                                let json = serde_json::to_string_pretty(&vu_data)
                                    .expect("Failed to serialize to JSON");
                                fs::write(&json_path, json).expect("Failed to write JSON file");

                                println!("JSON output written to: {}", json_path.display());
                                files_processed += 1;
                            }
                            Err(e) => {
                                println!("Failed to parse file: {}", path.display());
                                eprintln!("Error: {:#}", e);

                                let file_type = detect_from_file(path.to_str().unwrap());
                                println!("detected file: {:#}", file_type.unwrap());

                                panic!("Error occurred while parsing");
                            }
                        }
                    }
                }
            }
        }

        println!("Files processed: {}", files_processed);

        assert!(files_processed > 0, "No files were successfully processed");
    }
}
