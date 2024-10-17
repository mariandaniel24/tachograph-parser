use crate::dt::{gen1, gen2, gen2v2};
use anyhow::{Context, Result};
use byteorder::ReadBytesExt;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct VuGen1Blocks {
    pub vu_overview: gen1::VuOverviewBlock,
    pub vu_activities: Vec<gen1::VuActivitiesBlock>,
    pub vu_events_and_faults: Vec<gen1::VuEventsAndFaultsBlock>,
    pub vu_detailed_speed: Vec<gen1::VuDetailedSpeedData>,
    pub vu_company_locks: Vec<gen1::VuCompanyLocksBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VuGen2Blocks {
    pub vu_overview: gen2::VuOverviewBlock,
    pub vu_activities: Vec<gen2::VuActivitiesBlock>,
    pub vu_events_and_faults: Vec<gen2::VuEventsAndFaultsBlock>,
    pub vu_detailed_speed: Vec<gen2::VuSpeedBlock>,
    pub vu_company_locks: Vec<gen2::VuCompanyLocksBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VuGen2V2Blocks {
    pub vu_overview: gen2v2::VuOverviewBlock,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VuData {
    #[serde(rename = "vu_gen1_blocks")]
    VuGen1Blocks(VuGen1Blocks),
    #[serde(rename = "vu_gen2_blocks")]
    VuGen2Blocks(VuGen2Blocks),
    #[serde(rename = "vu_gen2v2_blocks")]
    VuGen2V2Blocks(VuGen2V2Blocks),
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
        let mut reader = BufReader::new(&self.input[..]);

        // Read the first byte to determine the generation
        let trep = reader.fill_buf().context("Failed to fill buffer")?[0];
        let sid = reader.fill_buf().context("Failed to fill buffer")?[1];

        match (trep, sid) {
            // Gen1 (checks for first block being VuOverviewBlock)
            (0x76, 0x01) => self.parse_gen1(&mut reader),
            // Gen2 (checks for first block being VuOverviewBlock)
            (0x76, 0x21) => self.parse_gen2(&mut reader),
            // Gen2V2 (checks for first block being VuOverviewBlock)
            (0x76, 0x31) => self.parse_gen2v2(&mut reader),
            _ => Err(anyhow::anyhow!(
                "Unknown file format: trep {:02x} sid {:02x}",
                trep,
                sid
            )),
        }
    }
    fn parse_gen1(&self, reader: &mut BufReader<&[u8]>) -> Result<VuData> {
        let mut vu_overview: Option<gen1::VuOverviewBlock> = None;
        let mut vu_activities: Vec<gen1::VuActivitiesBlock> = Vec::new();
        let mut vu_events_and_faults: Vec<gen1::VuEventsAndFaultsBlock> = Vec::new();
        let mut vu_detailed_speed: Vec<gen1::VuDetailedSpeedData> = Vec::new();
        let mut vu_company_locks: Vec<gen1::VuCompanyLocksBlock> = Vec::new();
        while !reader.fill_buf()?.is_empty() {
            let sid = reader.read_u8().context("Failed to read sid")?;
            let trep = reader.read_u8().context("Failed to read trep")?;
            log::trace!("sid: {:02x}, trep: {:02x}", sid, trep);
            match (sid, trep) {
                (0x76, 0x01) => {
                    vu_overview = Some(
                        gen1::VuOverviewBlock::parse(reader)
                            .context("Failed to parse VuOverviewBlock")?,
                    );
                }
                (0x76, 0x02) => {
                    vu_activities.push(
                        gen1::VuActivitiesBlock::parse(reader)
                            .context("Failed to parse VuActivitiesBlock")?,
                    );
                }
                (0x76, 0x03) => {
                    vu_events_and_faults.push(
                        gen1::VuEventsAndFaultsBlock::parse(reader)
                            .context("Failed to parse VuEventsAndFaultsBlock")?,
                    );
                }
                (0x76, 0x04) => {
                    vu_detailed_speed.push(
                        gen1::VuDetailedSpeedData::parse(reader)
                            .context("Failed to parse VuDetailedSpeedData")?,
                    );
                }
                (0x76, 0x05) => {
                    vu_company_locks.push(
                        gen1::VuCompanyLocksBlock::parse(reader)
                            .context("Failed to parse VuCompanyLocksBlock")?,
                    );
                }
                _ => {
                    log::warn!("Unknown block type: sid: {:02x}, trep: {:02x}", sid, trep);
                    break;
                }
            }
        }

        // Implement Gen1 parsing logic here
        Ok(VuData::VuGen1Blocks(VuGen1Blocks {
            vu_overview: vu_overview
                .context("unable to find VuOverviewBlock after parsing file")?,
            vu_activities,
            vu_events_and_faults,
            vu_detailed_speed,
            vu_company_locks,
        }))
    }

    fn parse_gen2(&self, reader: &mut BufReader<&[u8]>) -> Result<VuData> {
        let mut vu_overview: Option<gen2::VuOverviewBlock> = None;
        let mut vu_activities: Vec<gen2::VuActivitiesBlock> = Vec::new();
        let mut vu_events_and_faults: Vec<gen2::VuEventsAndFaultsBlock> = Vec::new();
        let mut vu_speed: Vec<gen2::VuSpeedBlock> = Vec::new();
        let mut vu_company_locks: Vec<gen2::VuCompanyLocksBlock> = Vec::new();

        while !reader.fill_buf()?.is_empty() {
            let sid = reader.read_u8().context("Failed to read sid")?;
            let trep = reader.read_u8().context("Failed to read trep")?;
            match (sid, trep) {
                (0x76, 0x21) => {
                    vu_overview = Some(
                        gen2::VuOverviewBlock::parse(reader)
                            .context("Failed to parse VuOverviewGen2")?,
                    )
                }
                (0x76, 0x22) => vu_activities.push(
                    gen2::VuActivitiesBlock::parse(reader)
                        .context("Failed to parse VuActivitiesGen2")?,
                ),
                (0x76, 0x23) => vu_events_and_faults.push(
                    gen2::VuEventsAndFaultsBlock::parse(reader)
                        .context("Failed to parse VuEventsAndFaultsGen2")?,
                ),
                (0x76, 0x24) => vu_speed.push(
                    gen2::VuSpeedBlock::parse(reader).context("Failed to parse VuDetailedSpeed")?,
                ),
                (0x76, 0x25) => vu_company_locks.push(
                    gen2::VuCompanyLocksBlock::parse(reader)
                        .context("Failed to parse VuCompanyLocksGen2")?,
                ),
                _ => {
                    log::warn!("Unknown block type: sid: {:02x}, trep: {:02x}", sid, trep);
                }
            }
        }

        Ok(VuData::VuGen2Blocks(VuGen2Blocks {
            vu_overview: vu_overview
                .context("unable to find VuOverviewBlock after parsing file")?,
            vu_activities,
            vu_events_and_faults,
            vu_detailed_speed: vu_speed,
            vu_company_locks,
        }))
    }

    fn parse_gen2v2(&self, reader: &mut BufReader<&[u8]>) -> Result<VuData> {
        let mut vu_overview: Option<gen2v2::VuOverviewBlock> = None;

        while !reader.fill_buf()?.is_empty() {
            let sid = reader.read_u8().context("Failed to read sid")?;
            let trep = reader.read_u8().context("Failed to read trep")?;
            match (sid, trep) {
                (0x76, 0x31) => {
                    vu_overview = Some(
                        gen2v2::VuOverviewBlock::parse(reader)
                            .context("Failed to parse VuOverviewGen2V2")?,
                    )
                }
                _ => {
                    log::warn!("Unknown block type: sid: {:02x}, trep: {:02x}", sid, trep);
                    break;
                }
            }
        }

        Ok(VuData::VuGen2V2Blocks(VuGen2V2Blocks {
            vu_overview: vu_overview
                .context("unable to find VuOverviewBlock after parsing file")?,
            // vu_activities: Vec::new(),
            // vu_events_and_faults: Vec::new(),
            // vu_detailed_speed: Vec::new(),
            // vu_company_locks: Vec::new(),
        }))
    }

    pub fn parse_to_json(&self) -> Result<String> {
        let vu_data = self.parse().context("Failed to parse vehicle data")?;
        let json = serde_json::to_value(&vu_data)
            .context("Failed to convert vehicle data to serde value")?;
        let pretty_json = serde_json::to_string_pretty(&json)
            .context("Failed to convert serde value to pretty JSON string")?;
        Ok(pretty_json)
    }
}
