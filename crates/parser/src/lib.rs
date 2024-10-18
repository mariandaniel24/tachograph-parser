mod bytes;
pub mod card_parser;
pub mod detector;
mod dt;
pub mod vu_parser;
use anyhow::{Context, Result};
use card_parser::CardParser;
use vu_parser::VuParser;

// Vehicle Unit
pub fn process_vu_file(file_path: &str) -> Result<vu_parser::VuData> {
    let output = VuParser::new_from_file(file_path)
        .context("Failed to create VuParser")?
        .parse();
    output
}
pub fn process_vu_file_json(file_path: &str) -> Result<String> {
    let vu_data_json = VuParser::new_from_file(file_path)
        .context("Failed to create VuParser")?
        .parse_to_json()?;
    Ok(vu_data_json)
}

pub fn process_vu_bytes(bytes: &[u8]) -> Result<vu_parser::VuData> {
    let output = VuParser::new_from_bytes(bytes)
        .context("Failed to create VuParser")?
        .parse();
    output
}
pub fn process_vu_bytes_json(bytes: &[u8]) -> Result<String> {
    let vu_data_json = VuParser::new_from_bytes(bytes)
        .context("Failed to create VuParser")?
        .parse_to_json()?;
    Ok(vu_data_json)
}

// Card
pub fn process_card_file(file_path: &str) -> Result<card_parser::CardData> {
    let output = CardParser::new_from_file(file_path)
        .context("Failed to create CardParser")?
        .parse();
    output
}
pub fn process_card_file_json(file_path: &str) -> Result<String> {
    let card_data_json = CardParser::new_from_file(file_path)
        .context("Failed to create CardParser")?
        .parse_to_json()?;
    Ok(card_data_json)
}

pub fn process_card_bytes(bytes: &[u8]) -> Result<card_parser::CardData> {
    let output = CardParser::new_from_bytes(bytes)
        .context("Failed to create CardParser")?
        .parse();
    output
}
pub fn process_card_bytes_json(bytes: &[u8]) -> Result<String> {
    let card_data_json = CardParser::new_from_bytes(bytes)
        .context("Failed to create CardParser")?
        .parse_to_json()?;
    Ok(card_data_json)
}