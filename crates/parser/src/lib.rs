mod bytes;
pub mod card_parser;
pub mod detector;
pub mod dt;
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
