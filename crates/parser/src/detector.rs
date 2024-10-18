use anyhow::{Context, Result};
use byteorder::ReadBytesExt;
use std::fmt::Display;
use std::io::BufReader;

#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "napi", napi)]
pub enum TachoFileType {
    VehicleUnitGen1,
    VehicleUnitGen2,
    VehicleUnitGen2V2,
    DriverCardGen1,
    DriverCardGen2,
    DriverCardGen2V2,
}
impl Display for TachoFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TachoFileType::VehicleUnitGen1 => write!(f, "Vehicle Unit Gen1"),
            TachoFileType::VehicleUnitGen2 => write!(f, "Vehicle Unit Gen2"),
            TachoFileType::VehicleUnitGen2V2 => write!(f, "Vehicle Unit Gen2 V2"),
            TachoFileType::DriverCardGen1 => write!(f, "Driver Card Gen1"),
            TachoFileType::DriverCardGen2 => write!(f, "Driver Card Gen2"),
            TachoFileType::DriverCardGen2V2 => write!(f, "Driver Card Gen2 V2"),
        }
    }
}

fn detect(fb: u8, sb: u8, buffer: &[u8]) -> Result<TachoFileType> {
    match [fb, sb] {
        [0x76, second_byte] if (0x01..=0x05).contains(&second_byte) => {
            log::info!(
                "File detected as vehicle gen1, first byte: 0x{:02X}, second byte: 0x{:02X}",
                fb,
                sb
            );
            Ok(TachoFileType::VehicleUnitGen1)
        }
        [0x76, second_byte] if (0x21..=0x25).contains(&second_byte) => {
            log::info!(
                "File detected as vehicle gen2v1, first: 0x{:02X}, second: 0x{:02X}",
                fb,
                sb
            );
            Ok(TachoFileType::VehicleUnitGen2)
        }
        [0x76, second_byte] if (0x30..=0x35).contains(&second_byte) => {
            log::info!(
                "File detected as vehicle gen2v2, first byte: 0x{:02X}, second byte: 0x{:02X}",
                fb,
                sb
            );
            Ok(TachoFileType::VehicleUnitGen2V2)
        }
        [0x00, 0x02] => {
            // Check for DriverCardGen2V2 first
            if buffer.windows(3).any(|window| window == [0x05, 0x25, 0x02]) {
                log::info!("File detected as Driver Card Gen2 V2");
                Ok(TachoFileType::DriverCardGen2V2)
            // Check for DriverCardGen2
            } else if buffer.windows(3).any(|window| window == [0x05, 0x01, 0x02]) {
                log::info!("File detected as Driver Card Gen2");
                Ok(TachoFileType::DriverCardGen2)
            } else {
                log::info!(
                    "File detected as Driver Card Gen1, first byte: 0x00, second byte: 0x02"
                );
                Ok(TachoFileType::DriverCardGen1)
            }
        }
        _ => anyhow::bail!(
            "Unsupported tacho file type, first byte: 0x{:02X}, second byte: 0x{:02X}",
            fb,
            sb
        ),
    }
}

pub fn detect_from_bytes(bytes: &[u8]) -> Result<TachoFileType> {
    let mut cursor = BufReader::new(bytes);

    let fb = cursor.read_u8().context("Failed to read first byte")?;
    let sb = cursor.read_u8().context("Failed to read second byte")?;

    detect(fb, sb, bytes)
}

pub fn detect_from_file(file_path: &str) -> Result<TachoFileType> {
    let bytes = std::fs::read(file_path).context("Failed to read file")?;
    let mut cursor = BufReader::new(&bytes[..]);

    let fb = cursor.read_u8().context("Failed to read first byte")?;
    let sb = cursor.read_u8().context("Failed to read second byte")?;

    detect(fb, sb, &bytes)
}
