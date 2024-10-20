use anyhow::{Context, Result};
use byteorder::ReadBytesExt;
#[cfg(feature = "napi")]
use napi_derive::napi;
use std::fmt::Display;
use std::io::{BufReader, Read};

#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "napi", napi(string_enum))]
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
            TachoFileType::VehicleUnitGen2V2 => write!(f, "Vehicle Unit Gen2V2"),
            TachoFileType::DriverCardGen1 => write!(f, "Driver Card Gen1"),
            TachoFileType::DriverCardGen2 => write!(f, "Driver Card Gen2"),
            TachoFileType::DriverCardGen2V2 => write!(f, "Driver Card Gen2V2"),
        }
    }
}

fn detect(fb: u8, sb: u8, buffer: &[u8]) -> Result<TachoFileType> {
    match [fb, sb] {
        // Vehicle Unit
        // Vehicle unit files always start with TREP 0x76, second byte usually refers to the block SID
        [0x76, _] => {
            // The order of these checks is NOT important for VU, as they are mutually exclusive

            // Check for Gen2V2 blocks
            if (0x31..=0x35).contains(&sb) {
                log::info!("File detected as Vehicle Unit Gen2V2");
                return Ok(TachoFileType::VehicleUnitGen2V2);
            }

            // Check for Gen2 blocks
            if (0x21..=0x25).contains(&sb) {
                log::info!("File detected as Vehicle Unit Gen2");
                return Ok(TachoFileType::VehicleUnitGen2);
            }

            // Check for Gen1 blocks
            if (0x01..=0x05).contains(&sb) {
                log::info!("File detected as Vehicle Unit Gen1");
                return Ok(TachoFileType::VehicleUnitGen1);
            }

            return Err(anyhow::anyhow!("Unsupported Vehicle Unit tacho file type"));
        }
        // Driver Card
        // These bytes should always be the same and should refer to the CardIccIdentification Gen1 (which driver files start with)
        [0x00, 0x02] => {
            // The order of these checks is important for Driver Card

            let mut reader = std::io::Cursor::new(buffer);
            reader.set_position(2); // Skip the first two bytes we've already read

            // Check for Gen2 V2 first
            if find_header(&mut reader, &[0x05, 0x25, 0x02]).is_some() {
                log::info!("File detected as Driver Card Gen2V2");
                return Ok(TachoFileType::DriverCardGen2V2);
            }

            // Reset reader position and check for Gen2
            reader.set_position(2);
            if find_header(&mut reader, &[0x05, 0x01, 0x02]).is_some() {
                log::info!("File detected as Driver Card Gen2");
                return Ok(TachoFileType::DriverCardGen2);
            }

            // Reset reader position and check for Gen1
            reader.set_position(2);
            if find_header(&mut reader, &[0x05, 0x01, 0x00]).is_some() {
                log::info!("File detected as Driver Card Gen1");
                return Ok(TachoFileType::DriverCardGen1);
            }

            return Err(anyhow::anyhow!("Unsupported Driver Card tacho file type"));
        }
        _ => anyhow::bail!(
            "Unsupported tacho file type, first byte: 0x{:02X}, second byte: 0x{:02X}",
            fb,
            sb
        ),
    }
}
/// Attempts to find a header in the buffer by reading 3 bytes at a time and comparing with the given header
fn find_header(reader: &mut std::io::Cursor<&[u8]>, header: &[u8]) -> Option<u64> {
    let mut buffer = [0u8; 3];
    while reader.read_exact(&mut buffer).is_ok() {
        if buffer == header {
            return Some(reader.position() - 3);
        }
        reader.set_position(reader.position() - 2);
    }
    None
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
