use crate::dt::{gen1, gen2, gen2v2};
use anyhow::{Context, Result};
use byteorder::ReadBytesExt;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Cursor};
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct VuGen1Blocks {
    pub vu_overview: gen1::VuOverviewBlock,
    pub vu_activities: Vec<gen1::VuActivitiesBlock>,
    pub vu_events_and_faults: Vec<gen1::VuEventsAndFaultsBlock>,
    pub vu_detailed_speed: Vec<gen1::VuDetailedSpeedBlock>,
    pub vu_company_locks: Vec<gen1::VuCompanyLocksBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct VuGen2Blocks {
    pub vu_overview: gen2::VuOverviewBlockGen2,
    pub vu_activities: Vec<gen2::VuActivitiesBlockGen2>,
    pub vu_events_and_faults: Vec<gen2::VuEventsAndFaultsBlockGen2>,
    pub vu_detailed_speed: Vec<gen2::VuDetailedSpeedBlockGen2>,
    pub vu_company_locks: Vec<gen2::VuCompanyLocksGen2>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct VuGen2V2Blocks {
    pub vu_overview: gen2v2::VuOverviewBlockGen2V2,
    pub vu_activities: Vec<gen2v2::VuActivitiesBlockGen2V2>,
    pub vu_events_and_faults: Vec<gen2::VuEventsAndFaultsBlockGen2>,
    pub vu_company_locks: Vec<gen2v2::VuCompanyLocksGen2V2>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "generation")]
#[cfg_attr(feature = "ts", derive(TS))]
pub enum VuData {
    Gen1(VuGen1Blocks),
    Gen2(VuGen2Blocks),
    Gen2V2(VuGen2V2Blocks),
}

pub struct VuParser {
    input: Vec<u8>,
}
impl VuParser {
    pub fn new_from_file(file_path: &str) -> Result<Self> {
        let input = std::fs::read(file_path).context("Failed to read file")?;
        Ok(VuParser { input })
    }
    pub fn new_from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(VuParser {
            input: bytes.to_vec(),
        })
    }

    pub fn parse(&self) -> Result<VuData> {
        let mut cursor = Cursor::new(&self.input[..]);

        // Read the first byte to determine the generation
        let trep = cursor.fill_buf().context("Failed to fill buffer")?[0];
        let sid = cursor.fill_buf().context("Failed to fill buffer")?[1];

        match (trep, sid) {
            // Gen1 (checks for first block being VuOverviewBlock)
            (0x76, 0x01..=0x05) => self.parse_gen1(&mut cursor),
            // Gen2 (checks for first block being VuOverviewBlock)
            (0x76, 0x21..=0x25) => self.parse_gen2(&mut cursor),
            // Gen2V2 (checks for first block being VuOverviewBlock)
            (0x76, 0x31..=0x35) => self.parse_gen2v2(&mut cursor),
            _ => Err(anyhow::anyhow!(
                "Unknown file format: trep {:02x} sid {:02x}",
                trep,
                sid
            )),
        }
    }
    fn parse_gen1(&self, cursor: &mut Cursor<&[u8]>) -> Result<VuData> {
        let mut vu_overview: Option<gen1::VuOverviewBlock> = None;
        let mut vu_activities: Vec<gen1::VuActivitiesBlock> = Vec::new();
        let mut vu_events_and_faults: Vec<gen1::VuEventsAndFaultsBlock> = Vec::new();
        let mut vu_detailed_speed: Vec<gen1::VuDetailedSpeedBlock> = Vec::new();
        let mut vu_company_locks: Vec<gen1::VuCompanyLocksBlock> = Vec::new();
        while !cursor.fill_buf()?.is_empty() {
            let sid = cursor.read_u8().context("Failed to read sid")?;
            let trep = cursor.read_u8().context("Failed to read trep")?;
            log::debug!(
                "Parsing vu data with sid: {:#04x}, trep: {:#04x}",
                sid,
                trep
            );
            match (sid, trep) {
                (0x76, 0x01) => {
                    vu_overview = Some(
                        gen1::VuOverviewBlock::parse(cursor)
                            .context("Failed to parse VuOverviewBlock")?,
                    );
                }
                (0x76, 0x02) => {
                    vu_activities.push(
                        gen1::VuActivitiesBlock::parse(cursor)
                            .context("Failed to parse VuActivitiesBlock")?,
                    );
                }
                (0x76, 0x03) => {
                    vu_events_and_faults.push(
                        gen1::VuEventsAndFaultsBlock::parse(cursor)
                            .context("Failed to parse VuEventsAndFaultsBlock")?,
                    );
                }
                (0x76, 0x04) => {
                    vu_detailed_speed.push(
                        gen1::VuDetailedSpeedBlock::parse(cursor)
                            .context("Failed to parse VuDetailedSpeedData")?,
                    );
                }
                (0x76, 0x05) => {
                    vu_company_locks.push(
                        gen1::VuCompanyLocksBlock::parse(cursor)
                            .context("Failed to parse VuCompanyLocksBlock")?,
                    );
                }
                _ => {
                    log::warn!("Unknown block type: sid: {:#04x}, trep: {:#04x}", sid, trep);
                    break;
                }
            }
        }

        // Implement Gen1 parsing logic here
        Ok(VuData::Gen1(VuGen1Blocks {
            vu_overview: vu_overview
                .context("unable to find VuOverviewBlock after parsing file")?,
            vu_activities,
            vu_events_and_faults,
            vu_detailed_speed,
            vu_company_locks,
        }))
    }

    fn parse_gen2(&self, cursor: &mut Cursor<&[u8]>) -> Result<VuData> {
        let mut vu_overview: Option<gen2::VuOverviewBlockGen2> = None;
        let mut vu_activities: Vec<gen2::VuActivitiesBlockGen2> = Vec::new();
        let mut vu_events_and_faults: Vec<gen2::VuEventsAndFaultsBlockGen2> = Vec::new();
        let mut vu_detailed_speed: Vec<gen2::VuDetailedSpeedBlockGen2> = Vec::new();
        let mut vu_company_locks: Vec<gen2::VuCompanyLocksGen2> = Vec::new();

        while !cursor.fill_buf()?.is_empty() {
            let sid = cursor.read_u8().context("Failed to read sid")?;
            let trep = cursor.read_u8().context("Failed to read trep")?;
            log::debug!(
                "Parsing vu data with sid: {:#04x}, trep: {:#04x}",
                sid,
                trep
            );
            match (sid, trep) {
                (0x76, 0x21) => {
                    vu_overview = Some(
                        gen2::VuOverviewBlockGen2::parse(cursor)
                            .context("Failed to parse VuOverviewGen2")?,
                    )
                }
                (0x76, 0x22) => vu_activities.push(
                    gen2::VuActivitiesBlockGen2::parse(cursor)
                        .context("Failed to parse VuActivitiesGen2")?,
                ),
                (0x76, 0x23) => vu_events_and_faults.push(
                    gen2::VuEventsAndFaultsBlockGen2::parse(cursor)
                        .context("Failed to parse VuEventsAndFaultsGen2")?,
                ),
                (0x76, 0x24) => vu_detailed_speed.push(
                    gen2::VuDetailedSpeedBlockGen2::parse(cursor)
                        .context("Failed to parse VuDetailedSpeed")?,
                ),
                (0x76, 0x25) => vu_company_locks.push(
                    gen2::VuCompanyLocksGen2::parse(cursor)
                        .context("Failed to parse VuCompanyLocksGen2")?,
                ),
                _ => {
                    log::warn!("Unknown block type: sid: {:#04x}, trep: {:#04x}", sid, trep);
                    break;
                }
            }
        }

        Ok(VuData::Gen2(VuGen2Blocks {
            vu_overview: vu_overview
                .context("unable to find VuOverviewBlock after parsing file")?,
            vu_activities,
            vu_events_and_faults,
            vu_detailed_speed,
            vu_company_locks,
        }))
    }

    fn parse_gen2v2(&self, cursor: &mut Cursor<&[u8]>) -> Result<VuData> {
        let mut vu_overview: Option<gen2v2::VuOverviewBlockGen2V2> = None;
        let mut vu_activities: Vec<gen2v2::VuActivitiesBlockGen2V2> = Vec::new();
        let mut vu_events_and_faults: Vec<gen2::VuEventsAndFaultsBlockGen2> = Vec::new();
        let mut vu_company_locks: Vec<gen2v2::VuCompanyLocksGen2V2> = Vec::new();

        while !cursor.fill_buf()?.is_empty() {
            let sid = cursor.read_u8().context("Failed to read sid")?;
            let trep = cursor.read_u8().context("Failed to read trep")?;
            log::debug!(
                "Parsing vu data with sid: {:#04x}, trep: {:#04x}",
                sid,
                trep
            );
            match (sid, trep) {
                (0x76, 0x31) => {
                    vu_overview = Some(
                        gen2v2::VuOverviewBlockGen2V2::parse(cursor)
                            .context("Failed to parse VuOverviewGen2V2")?,
                    )
                }
                (0x76, 0x32) => vu_activities.push(
                    gen2v2::VuActivitiesBlockGen2V2::parse(cursor)
                        .context("Failed to parse VuActivitiesGen2V2")?,
                ),
                (0x76, 0x33) => vu_events_and_faults.push(
                    gen2::VuEventsAndFaultsBlockGen2::parse(cursor)
                        .context("Failed to parse VuEventsAndFaultsGen2")?,
                ),
                (0x76, 0x35) => vu_company_locks.push(
                    gen2v2::VuCompanyLocksGen2V2::parse(cursor)
                        .context("Failed to parse VuCompanyLocksGen2V2")?,
                ),
                _ => {
                    log::warn!("Unknown block type: sid: {:02x}, trep: {:02x}", sid, trep);
                    break;
                }
            }
        }
        Ok(VuData::Gen2V2(VuGen2V2Blocks {
            vu_overview: vu_overview
                .context("unable to find VuOverviewBlock after parsing file")?,
            vu_activities,
            vu_events_and_faults,
            vu_company_locks,
        }))
    }

    pub fn parse_to_json(&self) -> Result<String> {
        let vu_data = self.parse().context("Failed to parse vehicle data")?;
        let pretty_json = serde_json::to_string_pretty(&vu_data)
            .context("Failed to convert serde value to pretty JSON string")?;
        Ok(pretty_json)
    }
    pub fn parse_to_json_pretty(&self) -> Result<String> {
        let vu_data = self.parse().context("Failed to parse vehicle data")?;
        let pretty_json = serde_json::to_string_pretty(&vu_data)
            .context("Failed to convert serde value to pretty JSON string")?;
        Ok(pretty_json)
    }
}

#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use serde_json;

    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_process_vu_file() {
        let data_dir = Path::new("../../data/ddd");
        let output_dir = Path::new("../../data/json");
        assert!(data_dir.exists(), "Data directory does not exist");
        fs::create_dir_all(output_dir).expect("Failed to create output directory");

        let results: Vec<(String, Result<(), anyhow::Error>)> = fs::read_dir(data_dir)
            .expect("Failed to read directory")
            .par_bridge()
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .path()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|name| {
                        name.starts_with("M_") && (name.ends_with(".ddd") || name.ends_with(".DDD"))
                    })
                    .unwrap_or(false)
            })
            .map(|entry| {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

                let result = VuParser::new_from_file(path.to_str().unwrap()).and_then(|vu_data| {
                    let vu_data = vu_data.parse()?;

                    let json_file_name =
                        file_name.replace(".ddd", ".json").replace(".DDD", ".json");
                    let json_path = output_dir.join(json_file_name);

                    let json = serde_json::to_string_pretty(&vu_data)
                        .expect("Failed to serialize to JSON");
                    fs::write(&json_path, json).expect("Failed to write JSON file");

                    Ok(())
                });

                (file_name, result)
            })
            .collect();

        // Process all errors after collection
        let errors: Vec<_> = results
            .iter()
            .filter_map(|(file_name, result)| result.as_ref().err().map(|e| (file_name, e)))
            .collect();

        if !errors.is_empty() {
            println!("\n=== Processing Errors ===");
            println!("Total errors: {}", errors.len());
            println!("Detailed error list:");
            for (file, error) in errors {
                println!("\nFile: {}", file);
                println!("Error: {:#}", error);
            }
            panic!("Some files failed to process");
        }
    }
}
