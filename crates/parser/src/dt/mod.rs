#![allow(dead_code)]
pub mod external;
pub mod gen1;
pub mod gen2;
pub mod gen2v2;
use crate::bytes::TakeExact;
use crate::bytes::{extract_u16_bits_into_tup, extract_u8_bits_into_tup};
use anyhow::{Context, Result};
use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, Utc};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};
use textcode;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
pub struct BCDString(pub String);
/// [BCDString: appendix 2.7.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16562)
impl BCDString {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let mut buffer = vec![0u8; size];
        cursor
            .read_exact(&mut buffer)
            .context("Failed to read BCDString")?;

        let bcd_string = buffer
            .iter()
            .map(|&byte| format!("{:02X}", byte))
            .collect::<String>();

        Ok(BCDString(bcd_string))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
pub struct IA5String(pub String);
impl IA5String {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let mut buffer = vec![0u8; size];
        cursor
            .read_exact(&mut buffer)
            .context("Failed to read IA5String")?;
        let value = textcode::utf8::decode_to_string(&buffer);
        Ok(IA5String(value.trim().to_string()))
    }
    pub fn parse_with_code_page(
        cursor: &mut Cursor<&[u8]>,
        size: usize,
        code_page: u8,
    ) -> Result<Self> {
        let mut buffer = vec![0u8; size];
        cursor
            .read_exact(&mut buffer)
            .context("Failed to read IA5String")?;
        let value = match code_page {
            1 => textcode::iso8859_1::decode_to_string(&buffer),
            2 => textcode::iso8859_2::decode_to_string(&buffer),
            3 => textcode::iso8859_3::decode_to_string(&buffer),
            4 => textcode::iso8859_4::decode_to_string(&buffer),
            5 => textcode::iso8859_5::decode_to_string(&buffer),
            6 => textcode::iso8859_6::decode_to_string(&buffer),
            7 => textcode::iso8859_7::decode_to_string(&buffer),
            8 => textcode::iso8859_8::decode_to_string(&buffer),
            9 => textcode::iso8859_9::decode_to_string(&buffer),
            13 => textcode::iso8859_13::decode_to_string(&buffer),
            14 => textcode::iso8859_14::decode_to_string(&buffer),
            15 => textcode::iso8859_15::decode_to_string(&buffer),
            16 => textcode::iso8859_16::decode_to_string(&buffer),
            80 => encoding_rs::KOI8_U.decode(&buffer).0.to_string(),
            85 => encoding_rs::KOI8_R.decode(&buffer).0.to_string(),
            // TODO: Might want to error out instead?
            // _ => anyhow::bail!("Unsupported code page: {}", code_page),
            _ => String::from_utf8_lossy(&buffer).to_string(),
        };

        Ok(IA5String(
            value.trim().trim_start_matches('\u{0001}').to_string(),
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [EmbedderIcAssemblerId: appendix 2.65.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e20005)
#[cfg_attr(feature = "napi", napi(object))]
pub struct EmbedderIcAssemblerId {
    pub country_code: IA5String,
    pub module_embedder: u16,
    pub manufacturer_information: u8, // OctetString
}
impl EmbedderIcAssemblerId {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let country_code = IA5String::parse_dyn_size(cursor, 2)?;

        let module_embedder = BCDString::parse_dyn_size(cursor, 2)?;
        let module_embedder = module_embedder
            .0
            .parse::<u16>()
            .context("Failed to parse module_embedder to a number")?;

        let manufacturer_information = cursor
            .read_u8()
            .context("Failed to read manufacturer_information")?;

        Ok(EmbedderIcAssemblerId {
            country_code,
            module_embedder,
            manufacturer_information,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardReplacementIndex: appendix 2.31.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17853)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardReplacementIndex(pub IA5String);
impl CardReplacementIndex {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = IA5String::parse_dyn_size(cursor, 1)?;
        Ok(CardReplacementIndex(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardConsecutiveIndex: appendix 2.14.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16973)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardConsecutiveIndex(pub IA5String);
impl CardConsecutiveIndex {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = IA5String::parse_dyn_size(cursor, 1)?;
        Ok(CardConsecutiveIndex(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardRenewalIndex: appendix 2.30.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17812)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardRenewalIndex(pub IA5String);
impl CardRenewalIndex {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = IA5String::parse_dyn_size(cursor, 1)?;
        Ok(CardRenewalIndex(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [CardNumber: appendix 2.26.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17629)
pub enum CardNumber {
    #[serde(rename_all(serialize = "camelCase"))]
    Driver {
        driver_identification: IA5String,
        card_replacement_index: CardReplacementIndex,
        card_renewal_index: CardRenewalIndex,
    },

    #[serde(rename_all(serialize = "camelCase"))]
    Owner {
        owner_identification: IA5String,
        card_consecutive_index: CardConsecutiveIndex,
        card_replacement_index: CardReplacementIndex,
        card_renewal_index: CardRenewalIndex,
    },
    None,
}
impl CardNumber {
    // This method is only used to consume the null bytes
    pub fn parse_unknown(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let _ = cursor
            .read_exact(&mut [0u8; 16])
            .context("Failed to read CardNumber null bytes")?;
        Ok(CardNumber::None)
    }

    pub fn parse_driver(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let driver_identification = IA5String::parse_dyn_size(cursor, 14)?;
        let card_replacement_index = CardReplacementIndex::parse(cursor)?;
        let card_renewal_index = CardRenewalIndex::parse(cursor)?;

        Ok(CardNumber::Driver {
            driver_identification,
            card_replacement_index,
            card_renewal_index,
        })
    }
    pub fn parse_owner(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let owner_identification = IA5String::parse_dyn_size(cursor, 13)?;
        let card_consecutive_index = CardConsecutiveIndex::parse(cursor)?;
        let card_replacement_index = CardReplacementIndex::parse(cursor)?;
        let card_renewal_index = CardRenewalIndex::parse(cursor)?;

        Ok(CardNumber::Owner {
            owner_identification,
            card_consecutive_index,
            card_replacement_index,
            card_renewal_index,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [TimeReal: appendix 2.162.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24993)
#[cfg_attr(feature = "napi", napi(object))]
pub struct TimeReal(pub DateTime<Utc>);
// TODO: Determine what timezone is used in the DDD files
// According to @mpi-wl, the timezone is UTC, see https://github.com/jugglingcats/tachograph-cursor/issues/54#issuecomment-603089791
impl TimeReal {
    const SIZE: usize = 4;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let unix_timestamp = cursor
            .read_u32::<BigEndian>()
            .context("Failed to read TimeReal")?;

        // Ensure we're not past max u32 timestamp and is not 0 or less
        if unix_timestamp > 2_147_483_647 || unix_timestamp < 1 {
            return Err(anyhow::anyhow!(
                "TimeReal value exceeds maximum for 32-bit timestamp (2038-01-19)"
            ));
        }

        let dt = chrono::DateTime::from_timestamp(unix_timestamp as i64, 0)
            .context("Failed to create DateTime from unix timestamp")?;

        Ok(TimeReal(dt))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CurrentDateTime: appendix 2.54.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e19437)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CurrentDateTime(pub TimeReal);
impl CurrentDateTime {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(CurrentDateTime(TimeReal::parse(cursor)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardApprovalNumber: appendix 2.11.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16800)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardApprovalNumber(pub IA5String);
impl CardApprovalNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = IA5String::parse_dyn_size(cursor, 8)?;
        Ok(CardApprovalNumber(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [WVehicleCharacteristicConstant: appendix 2.239.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e29395)
#[cfg_attr(feature = "napi", napi(object))]
pub struct WVehicleCharacteristicConstant(pub u16);
impl WVehicleCharacteristicConstant {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read WVehicleCharacteristicConstant")?;
        Ok(WVehicleCharacteristicConstant(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]

/// [KConstantOfRecordingEquipment: appendix 2.85.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21927)
#[cfg_attr(feature = "napi", napi(object))]
pub struct KConstantOfRecordingEquipment(pub u16);
impl KConstantOfRecordingEquipment {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read KConstantOfRecordingEquipment")?;
        Ok(KConstantOfRecordingEquipment(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [CardStructureVersion: appendix 2.36.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18081)
pub enum CardStructureVersion {
    Gen1,
    Gen2,
    Gen2V2,
}
impl CardStructureVersion {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let fb = cursor
            .read_u8()
            .context("Failed to read first byte of CardStructureVersion")?;
        let sb = cursor
            .read_u8()
            .context("Failed to read second byte of CardStructureVersion")?;

        let version = match (fb, sb) {
            (0x00, 0x00) => Self::Gen1,
            (0x01, 0x00) => Self::Gen2,
            (0x01, 0x01) => Self::Gen2V2,
            _ => {
                return Err(
                    anyhow::anyhow!("Invalid CardStructureVersion: {:02x} {:02x}", fb, sb).into(),
                )
            }
        };

        Ok(version)
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]

/// [LTyreCircumference: appendix 2.91.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22169)
#[cfg_attr(feature = "napi", napi(object))]
pub struct LTyreCircumference(pub u16);
impl LTyreCircumference {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read LTyreCircumference")?;
        Ok(LTyreCircumference(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [TyreSize: appendix 2.163.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25026)
#[cfg_attr(feature = "napi", napi(object))]
pub struct TyreSize(pub IA5String);
impl TyreSize {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = IA5String::parse_dyn_size(cursor, 15)?;
        Ok(TyreSize(value))
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [Speed: appendix 2.155.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24822)
#[cfg_attr(feature = "napi", napi(object))]
pub struct Speed(pub u8);
impl Speed {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor.read_u8().context("Failed to read value for Speed")?;
        Ok(Speed(value))
    }
}

/// [SpeedAuthorised: appendix 2.156.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24843)
pub type SpeedAuthorised = Speed;

/// [SpeedAverage: appendix 2.157.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24860)
pub type SpeedAverage = Speed;

/// [SpeedMax: appendix 2.158.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24877)
pub type SpeedMax = Speed;

/// [OverspeedNumber: appendix 2.116.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23023)
pub type OverspeedNumber = Speed;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [Name: appendix 2.299.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22398)
#[cfg_attr(feature = "napi", napi(object))]
pub struct Name {
    pub code_page: u8,
    pub name: IA5String,
}
impl Name {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let code_page = cursor.read_u8().context("Failed to read code page")?;
        let name = IA5String::parse_with_code_page(cursor, 35, code_page)?;
        Ok(Name { code_page, name })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [Address: appendix 2.2.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16375)
#[cfg_attr(feature = "napi", napi(object))]
pub struct Address {
    pub code_page: u8,
    pub address: IA5String,
}
impl Address {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let code_page = cursor.read_u8().context("Failed to read code page")?;
        let address = IA5String::parse_with_code_page(cursor, 35, code_page)?;
        Ok(Address { code_page, address })
    }
}

/// [VuManufacturerName: appendix 2.240.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25160)
pub type VuManufacturerName = Name;

/// [VuManufacturerAddress: appendix 2.209.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27911)
pub type VuManufacturerAddress = Address;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuSoftwareVersion: appendix 2.226.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28569)
#[cfg_attr(feature = "napi", napi(object))]
pub struct VuSoftwareVersion(pub IA5String);
impl VuSoftwareVersion {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuSoftwareVersion(IA5String::parse_dyn_size(cursor, 4)?))
    }
}
/// [VuSoftInstallationDate: appendix 2.224.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28515)
pub type VuSoftInstallationDate = TimeReal;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuSoftwareIdentification: appendix 2.225.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28538)
#[cfg_attr(feature = "napi", napi(object))]
pub struct VuSoftwareIdentification {
    pub vu_software_version: VuSoftwareVersion,
    pub vu_soft_installation_date: VuSoftInstallationDate,
}
impl VuSoftwareIdentification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let vu_software_version = VuSoftwareVersion::parse(cursor)?;
        let vu_soft_installation_date = VuSoftInstallationDate::parse(cursor)?;

        Ok(VuSoftwareIdentification {
            vu_software_version,
            vu_soft_installation_date,
        })
    }
}

/// [VuManufacturingDate: appendix 2.211.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27955)
pub type VuManufacturingDate = TimeReal;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [SimilarEventsNumber: appendix 2.151.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24591)
#[cfg_attr(feature = "napi", napi(object))]
pub struct SimilarEventsNumber(pub u8);
impl SimilarEventsNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to read value for SimilarEventsNumber")?;

        Ok(SimilarEventsNumber(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [EventFaultRecordPurpose: appendix 2.69.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e20262)
pub enum EventFaultRecordPurpose {
    OneOfTenMostRecentOrLast,
    LongestEventLastTenDays,
    OneOfFiveLongestEventsLastYear,
    LastEventLastTenDays,
    MostSeriousEventLastTenDays,
    OneOfFiveMostSeriousEventsLastYear,
    FirstEventAfterLastCalibration,
    ActiveOrOngoing,
    RFU,
    ManufacturerSpecific,
}
impl EventFaultRecordPurpose {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to read EventFaultRecordPurpose")?;
        let parsed_value = match value {
            0x00 => Self::OneOfTenMostRecentOrLast,
            0x01 => Self::LongestEventLastTenDays,
            0x02 => Self::OneOfFiveLongestEventsLastYear,
            0x03 => Self::LastEventLastTenDays,
            0x04 => Self::MostSeriousEventLastTenDays,
            0x05 => Self::OneOfFiveMostSeriousEventsLastYear,
            0x06 => Self::FirstEventAfterLastCalibration,
            0x07 => Self::ActiveOrOngoing,
            0x08..=0x7F => Self::RFU,
            0x80..=0xFF => Self::ManufacturerSpecific,
        };
        Ok(parsed_value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VehicleIdentificationNumber: appendix 2.165.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25052)
#[cfg_attr(feature = "napi", napi(object))]
pub struct VehicleIdentificationNumber(pub IA5String);
impl VehicleIdentificationNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let vin = IA5String::parse_dyn_size(cursor, 17)?;
        Ok(VehicleIdentificationNumber(vin))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VehicleRegistrationNumber: appendix 2.168.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25188)
#[cfg_attr(feature = "napi", napi(object))]
pub struct VehicleRegistrationNumber {
    pub code_page: u8,
    pub vehicle_reg_number: IA5String,
}
impl VehicleRegistrationNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let code_page = cursor.read_u8().context("Failed to read code page")?;
        let vehicle_reg_number = IA5String::parse_with_code_page(cursor, 13, code_page)?;
        Ok(VehicleRegistrationNumber {
            code_page,
            vehicle_reg_number,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
pub enum CardSlotStatus {
    NoCardInserted,
    DriverCardInserted,
    WorkshopCardInserted,
    ControlCardInserted,
    CompanyCardInserted,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardSlotsStatus: appendix 2.34.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17939)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardSlotsStatus {
    pub codriver: CardSlotStatus,
    pub driver: CardSlotStatus,
}
impl CardSlotsStatus {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let status = cursor
            .read_u8()
            .context("Failed to read card slots status")?;

        let bits = extract_u8_bits_into_tup(status);
        let codriver_status = match bits {
            (0, 0, 0, 0, _, _, _, _) => CardSlotStatus::NoCardInserted,
            (0, 0, 0, 1, _, _, _, _) => CardSlotStatus::DriverCardInserted,
            (0, 0, 1, 0, _, _, _, _) => CardSlotStatus::WorkshopCardInserted,
            (0, 0, 1, 1, _, _, _, _) => CardSlotStatus::ControlCardInserted,
            (0, 1, 0, 0, _, _, _, _) => CardSlotStatus::CompanyCardInserted,
            _ => anyhow::bail!("Invalid codriver slot status"),
        };

        let driver_status = match bits {
            (_, _, _, _, 0, 0, 0, 0) => CardSlotStatus::NoCardInserted,
            (_, _, _, _, 0, 0, 0, 1) => CardSlotStatus::DriverCardInserted,
            (_, _, _, _, 0, 0, 1, 0) => CardSlotStatus::WorkshopCardInserted,
            (_, _, _, _, 0, 0, 1, 1) => CardSlotStatus::ControlCardInserted,
            (_, _, _, _, 0, 1, 0, 0) => CardSlotStatus::CompanyCardInserted,
            _ => anyhow::bail!("Invalid driver slot status"),
        };
        Ok(CardSlotsStatus {
            codriver: codriver_status,
            driver: driver_status,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [HolderName: appendix 2.83.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21860)
#[cfg_attr(feature = "napi", napi(object))]
pub struct HolderName {
    pub holder_surname: Name,
    pub holder_first_names: Name,
}
impl HolderName {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(HolderName {
            holder_surname: Name::parse(cursor)?,
            holder_first_names: Name::parse(cursor)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [CardSlotNumber: appendix 2.33.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17911)
pub enum CardSlotNumber {
    DriverSlot,
    CoDriverSlot,
}
impl CardSlotNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to read card_slot_number value")?;
        let card_slot = match value {
            0 => CardSlotNumber::DriverSlot,
            1 => CardSlotNumber::CoDriverSlot,
            _ => anyhow::bail!("Invalid card slot number"),
        };
        Ok(card_slot)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [OdometerShort: appendix 2.113.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22854)
#[cfg_attr(feature = "napi", napi(object))]
pub struct OdometerShort(pub u32);
impl OdometerShort {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut km_buffer = [0u8; 3];
        cursor
            .read_exact(&mut km_buffer)
            .context("Failed to read odometer short km value")?;
        // odometer short is 3 bytes, so we must pad the buffer with 1 byte to use a u32
        let km = u32::from_be_bytes([0, km_buffer[0], km_buffer[1], km_buffer[2]]);
        Ok(OdometerShort(km))
    }
}
/// [OdometerValueMidnight: appendix 2.114.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22880)
pub type OdometerValueMidnight = OdometerShort;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VehicleRegistrationIdentification: appendix 2.116.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25120)
#[cfg_attr(feature = "napi", napi(object))]
pub struct VehicleRegistrationIdentification {
    pub vehicle_registration_nation: external::NationNumeric,
    pub vehicle_registration_number: VehicleRegistrationNumber,
}
impl VehicleRegistrationIdentification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let vehicle_registration_nation = external::NationNumeric::parse(cursor)?;
        let vehicle_registration_number = VehicleRegistrationNumber::parse(cursor)?;
        Ok(VehicleRegistrationIdentification {
            vehicle_registration_nation,
            vehicle_registration_number,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [ManualInputFlag: appendix 2.93.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22225)
pub enum ManualInputFlag {
    NoEntry,
    ManualEntries,
}
impl ManualInputFlag {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let manual_input_flag = cursor
            .read_u8()
            .context("Failed to read manual input flag")?;
        let manual_input_flag = match manual_input_flag {
            0 => ManualInputFlag::NoEntry,
            1 => ManualInputFlag::ManualEntries,
            _ => anyhow::bail!("Invalid manual input flag"),
        };
        Ok(manual_input_flag)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
pub enum ActivityChangeInfoSlot {
    Driver,
    CoDriver,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
pub enum ActivityChangeInfoDrivingStatus {
    Single,
    Crew,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
pub enum ActivityChangeInfoCardStatusSlot {
    Inserted,
    NotInserted,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
pub enum ActivityChangeInfoCardActivity {
    BreakRest,
    Availability,
    Work,
    Driving,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [ActivityChangeInfo: appendix 2.1.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16027)
#[cfg_attr(feature = "napi", napi(object))]
pub struct ActivityChangeInfo {
    pub slot: ActivityChangeInfoSlot,
    pub driving_status: ActivityChangeInfoDrivingStatus,
    pub slot_status: ActivityChangeInfoCardStatusSlot,
    pub activity: ActivityChangeInfoCardActivity,
    pub minutes: u16,
}
impl ActivityChangeInfo {
    pub const SIZE: usize = 2;

    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        let value_buffer = inner_cursor
            .read_u16::<BigEndian>()
            .context("Failed to read activity change info")?;
        let bits = extract_u16_bits_into_tup(value_buffer);

        let slot = match bits.0 {
            0 => ActivityChangeInfoSlot::Driver,
            1 => ActivityChangeInfoSlot::CoDriver,
            _ => anyhow::bail!("Invalid slot"),
        };

        let driving_status = match bits.1 {
            0 => ActivityChangeInfoDrivingStatus::Single,
            1 => ActivityChangeInfoDrivingStatus::Crew,
            _ => anyhow::bail!("Invalid driving status"),
        };

        let slot_status = match bits.2 {
            0 => ActivityChangeInfoCardStatusSlot::Inserted,
            1 => ActivityChangeInfoCardStatusSlot::NotInserted,
            _ => anyhow::bail!("Invalid card status slot"),
        };

        let activity = match (bits.3, bits.4) {
            (0, 0) => ActivityChangeInfoCardActivity::BreakRest,
            (0, 1) => ActivityChangeInfoCardActivity::Availability,
            (1, 0) => ActivityChangeInfoCardActivity::Work,
            (1, 1) => ActivityChangeInfoCardActivity::Driving,
            _ => anyhow::bail!("Invalid card activity"),
        };

        // Take the last 11 bits and convert them to a u16
        let minutes = (bits.5 as u16) << 10
            | (bits.6 as u16) << 9
            | (bits.7 as u16) << 8
            | (bits.8 as u16) << 7
            | (bits.9 as u16) << 6
            | (bits.10 as u16) << 5
            | (bits.11 as u16) << 4
            | (bits.12 as u16) << 3
            | (bits.13 as u16) << 2
            | (bits.14 as u16) << 1
            | (bits.15 as u16);

        Ok(ActivityChangeInfo {
            slot,
            driving_status,
            slot_status,
            activity,
            minutes,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardChipIdentification: appendix 2.1.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16027)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardChipIdentification {
    pub card_chip_identification_number: Vec<u8>,
    pub card_chip_identification_signature: Vec<u8>,
}
impl CardChipIdentification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut card_chip_identification_number = [0u8; 4];
        cursor
            .read_exact(&mut card_chip_identification_number)
            .context("Failed to read card chip identification number")?;

        let mut card_chip_identification_signature = [0u8; 4];
        cursor
            .read_exact(&mut card_chip_identification_signature)
            .context("Failed to read card chip identification signature")?;
        Ok(CardChipIdentification {
            card_chip_identification_number: card_chip_identification_number.to_vec(),
            card_chip_identification_signature: card_chip_identification_signature.to_vec(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [Datef: appendix 2.63.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e20100)
#[cfg_attr(feature = "napi", napi(object))]
pub struct Datef {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}
impl Datef {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let year = BCDString::parse_dyn_size(cursor, 2)?
            .0
            .parse::<u16>()
            .context("Failed to parse year")?;
        let month = BCDString::parse_dyn_size(cursor, 1)?
            .0
            .parse::<u8>()
            .context("Failed to parse month")?;
        let day = BCDString::parse_dyn_size(cursor, 1)?
            .0
            .parse::<u8>()
            .context("Failed to parse day")?;
        Ok(Datef { day, month, year })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
pub struct Language(pub IA5String);
impl Language {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Language(IA5String::parse_dyn_size(cursor, 2)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardIdentification: appendix 2.24.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17430)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardIdentification {
    pub card_issuing_member_state: external::NationNumeric,
    pub card_number: CardNumber,
    pub card_issuing_authority_name: Name,
    pub card_issue_date: TimeReal,
    pub card_validity_begin: TimeReal,
    pub card_expiry_date: TimeReal,
}
impl CardIdentification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_issuing_member_state = external::NationNumeric::parse(cursor)?;
        // TODO: Check if this is correct, not sure if this works with workshop/company cards, we might need to get the type of the card file
        // and parse the card number accordingly
        let card_number = CardNumber::parse_driver(cursor)?;
        let card_issuing_authority_name = Name::parse(cursor)?;
        let card_issue_date = TimeReal::parse(cursor)?;
        let card_validity_begin = TimeReal::parse(cursor)?;
        let card_expiry_date = TimeReal::parse(cursor)?;
        Ok(CardIdentification {
            card_issuing_member_state,
            card_number,
            card_issuing_authority_name,
            card_issue_date,
            card_validity_begin,
            card_expiry_date,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [DriverCardHolderIdentification: appendix 2.62.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e19928)
#[cfg_attr(feature = "napi", napi(object))]
pub struct DriverCardHolderIdentification {
    pub card_holder_number: HolderName,
    pub card_holder_birth_date: Datef,
    pub card_holder_preferred_language: Language,
}
impl DriverCardHolderIdentification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_holder_number = HolderName::parse(cursor)?;
        let card_holder_birth_date = Datef::parse(cursor)?;
        let card_holder_preferred_language = Language::parse(cursor)?;
        Ok(DriverCardHolderIdentification {
            card_holder_number,
            card_holder_birth_date,
            card_holder_preferred_language,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [Identification: appendix 4.2.2.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e41651)
#[cfg_attr(feature = "napi", napi(object))]
pub struct Identification {
    pub card_identification: CardIdentification,
    pub driver_card_holder_identification: DriverCardHolderIdentification,
}
impl Identification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_identification = CardIdentification::parse(cursor)?;
        let driver_card_holder_identification = DriverCardHolderIdentification::parse(cursor)?;
        Ok(Identification {
            card_identification,
            driver_card_holder_identification,
        })
    }
}

/// [LastCardDownload: appendix 2.89.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22044)
pub type LastCardDownload = TimeReal;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardDownload: appendix 4.2.2.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e41651)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardDownload {
    pub last_card_download: Option<LastCardDownload>,
}
impl CardDownload {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(LastCardDownload::SIZE);

        let last_card_download = LastCardDownload::parse(inner_cursor).ok();
        Ok(CardDownload { last_card_download })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardDrivingLicenceInformation: appendix 2.18.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17139)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardDrivingLicenceInformation {
    pub driving_licence_issuing_authority: Name,
    pub driving_licence_issuing_nation: external::NationNumeric,
    pub driving_licence_number: IA5String,
}
impl CardDrivingLicenceInformation {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let driving_licence_issuing_authority = Name::parse(cursor)?;
        let driving_licence_issuing_nation = external::NationNumeric::parse(cursor)?;
        let driving_licence_number = IA5String::parse_dyn_size(cursor, 16)?;
        Ok(CardDrivingLicenceInformation {
            driving_licence_issuing_authority,
            driving_licence_issuing_nation,
            driving_licence_number,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// EF Block page 281
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardDrivingLicenceInfo {
    pub card_driving_licence_information: CardDrivingLicenceInformation,
}
impl CardDrivingLicenceInfo {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_driving_licence_information = CardDrivingLicenceInformation::parse(cursor)?;
        Ok(CardDrivingLicenceInfo {
            card_driving_licence_information,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [DailyPresenceCounter: appendix 2.56.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e19510)
#[cfg_attr(feature = "napi", napi(object))]
pub struct DailyPresenceCounter(pub u16);
impl DailyPresenceCounter {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = BCDString::parse_dyn_size(cursor, 2)?;
        let value = value
            .0
            .parse::<u16>()
            .context("Failed to parse daily presence counter")?;
        Ok(DailyPresenceCounter(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [Distance: appendix 2.60.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e19665)
#[cfg_attr(feature = "napi", napi(object))]
pub struct Distance(pub u16);
impl Distance {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read distance")?;
        Ok(Distance(value))
    }
}

/// [CardActivityLengthRange: appendix 2.10.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16777)
pub type CardActivityLengthRange = u16;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardDriverActivity: appendix 2.9.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16718)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardActivityDailyRecord {
    pub activity_previous_record_length: CardActivityLengthRange,
    pub activity_record_length: CardActivityLengthRange,
    pub activity_record_date: TimeReal,
    pub activity_daily_presence_counter: DailyPresenceCounter,
    pub activity_day_distance: Distance,
    pub activity_change_info: Vec<ActivityChangeInfo>,
}
impl CardActivityDailyRecord {
    // 12 bytes of metadata =
    //      activity_previous_record_length +
    //      activity_record_length +
    //      activity_record_date +
    //      activity_daily_presence_counter +
    //      activity_day_distance
    const SIZE_OF_METADATA: u16 = 12;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let activity_previous_record_length: CardActivityLengthRange = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read activity_previous_record_length")?;
        let activity_record_length: CardActivityLengthRange = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read activity_record_length")?;

        let activity_record_date = TimeReal::parse(cursor)?;
        let activity_daily_presence_counter = DailyPresenceCounter::parse(cursor)?;
        let activity_day_distance = Distance::parse(cursor)?;

        let records_amount = (activity_record_length as usize - Self::SIZE_OF_METADATA as usize)
            / ActivityChangeInfo::SIZE;

        let mut activity_change_info = Vec::with_capacity(records_amount);
        for _ in 0..records_amount {
            if let Ok(record) = ActivityChangeInfo::parse(cursor) {
                activity_change_info.push(record);
            }
        }

        Ok(CardActivityDailyRecord {
            activity_previous_record_length,
            activity_record_length,
            activity_record_date,
            activity_daily_presence_counter,
            activity_day_distance,
            activity_change_info,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardDriverActivity: appendix 2.17.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17092)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardDriverActivity {
    pub activity_pointer_oldest_day_record: u16,
    pub activity_pointer_newest_record: u16,
    pub activity_daily_records: Vec<CardActivityDailyRecord>,
}
impl CardDriverActivity {
    const SIZE: usize = 13776;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let activity_pointer_oldest_day_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read activity_pointer_oldest_day_record")?;
        let activity_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read activity_pointer_newest_record")?;

        // Read the entire cyclic data block
        let mut cyclic_data = vec![0u8; Self::SIZE as usize];
        cursor
            .read_exact(&mut cyclic_data)
            .context("Failed to read cyclic data")?;

        let uncycled_data = Self::read_cyclic_data(
            &cyclic_data,
            activity_pointer_oldest_day_record as usize,
            activity_pointer_newest_record as usize,
        )?;

        let activity_daily_records = Self::parse_daily_records(&uncycled_data)?;

        Ok(CardDriverActivity {
            activity_pointer_oldest_day_record,
            activity_pointer_newest_record,
            activity_daily_records,
        })
    }

    fn read_cyclic_data(
        cyclic_data: &[u8],
        oldest_record: usize,
        newest_record: usize,
    ) -> Result<Vec<u8>> {
        // Get the length of the newest record
        let newest_record_length = u16::from_be_bytes([
            cyclic_data[newest_record as usize + 2],
            cyclic_data[newest_record as usize + 3],
        ]) as usize;

        // Calculate the end position of the newest record
        let end_of_newest_record = (newest_record + newest_record_length) % Self::SIZE;

        // Check if the data wraps around the end of the buffer
        let is_wrapped = end_of_newest_record < oldest_record;

        let uncycled_data = if is_wrapped {
            // If wrapped, concatenate two slices:
            // 1. From oldest_record to the end of the buffer
            // 2. From the start of the buffer to the end of the newest record
            let mut data = Vec::new();
            data.extend_from_slice(&cyclic_data[oldest_record..]);
            data.extend_from_slice(&cyclic_data[..end_of_newest_record]);
            data
        } else {
            // If not wrapped, simply take the slice from oldest to newest
            cyclic_data[oldest_record..end_of_newest_record].to_vec()
        };

        Ok(uncycled_data)
    }

    fn parse_daily_records(data: &[u8]) -> Result<Vec<CardActivityDailyRecord>> {
        let mut cursor = std::io::Cursor::new(data);
        let mut records = Vec::new();

        while cursor.position() < data.len() as u64 {
            match CardActivityDailyRecord::parse(&mut cursor) {
                Ok(record) => records.push(record),
                Err(e) => {
                    log::warn!("Failed to parse daily record: {:?}", e);
                    break;
                }
            }
        }

        Ok(records)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// EF Block page 281
#[cfg_attr(feature = "napi", napi(object))]
pub struct DriverActivityData {
    pub card_driver_activity: CardDriverActivity,
}
impl DriverActivityData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_driver_activity = CardDriverActivity::parse(cursor)?;
        Ok(DriverActivityData {
            card_driver_activity,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuDataBlockCounter: appendix 2.189.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26512)
#[cfg_attr(feature = "napi", napi(object))]
pub struct VuDataBlockCounter(pub u16);

impl VuDataBlockCounter {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = BCDString::parse_dyn_size(cursor, 2)?;

        let num_value = value
            .0
            .parse::<u16>()
            .context("Failed to parse VuDataBlockCounter from BCDString to number")?;
        if num_value > 9999 {
            anyhow::bail!("Invalid VuDataBlockCounter value: {}", num_value);
        }

        Ok(VuDataBlockCounter(num_value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardCurrentUse appendix 2.16.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17059)
#[cfg_attr(feature = "napi", napi(object))]
pub struct CardCurrentUse {
    pub session_open_time: TimeReal,
    pub session_open_vehicle: VehicleRegistrationIdentification,
}
impl CardCurrentUse {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let session_open_time = TimeReal::parse(cursor)?;
        let session_open_vehicle = VehicleRegistrationIdentification::parse(cursor)?;
        Ok(CardCurrentUse {
            session_open_time,
            session_open_vehicle,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// EF Block page 281
#[cfg_attr(feature = "napi", napi(object))]
pub struct CurrentUsage {
    pub card_current_use: CardCurrentUse,
}
impl CurrentUsage {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_current_use = CardCurrentUse::parse(cursor)?;
        Ok(CurrentUsage { card_current_use })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
pub struct MonthYear {
    pub month: u8,
    pub year: u8,
}
impl MonthYear {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let month = BCDString::parse_dyn_size(cursor, 1)?
            .0
            .parse::<u8>()
            .context("Failed to parse month from BCDString to number")?;
        let year = BCDString::parse_dyn_size(cursor, 1)?
            .0
            .parse::<u8>()
            .context("Failed to parse year from BCDString to number")?;
        Ok(MonthYear { month, year })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuDownloadablePeriod: appendix 2.193](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26674)
#[cfg_attr(feature = "napi", napi(object))]
pub struct VuDownloadablePeriod {
    pub min_downloadable_time: TimeReal,
    pub max_downloadable_time: TimeReal,
}
impl VuDownloadablePeriod {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let min_downloadable_time =
            TimeReal::parse(cursor).context("Failed to parse min_downloadable_time")?;
        let max_downloadable_time =
            TimeReal::parse(cursor).context("Failed to parse max_downloadable_time")?;

        Ok(VuDownloadablePeriod {
            min_downloadable_time,
            max_downloadable_time,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuDetailedSpeedBlock: appendix 2.224.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26534)
#[cfg_attr(feature = "napi", napi(object))]
pub struct VuDetailedSpeedBlock {
    pub speed_block_begin_date: TimeReal,
    pub speeds_per_second: Vec<Speed>,
}

impl VuDetailedSpeedBlock {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let speed_block_begin_date =
            TimeReal::parse(cursor).context("Failed to parse speed_block_begin_date")?;

        let mut speeds_per_second = Vec::with_capacity(60);
        for _ in 0..60 {
            speeds_per_second.push(Speed::parse(cursor).context("Failed to parse speed")?);
        }

        Ok(VuDetailedSpeedBlock {
            speed_block_begin_date,
            speeds_per_second,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuPartNumber: appendix 2.217.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28257)
#[cfg_attr(feature = "napi", napi(object))]
pub struct VuPartNumber(pub IA5String);
impl VuPartNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuPartNumber(IA5String::parse_dyn_size(cursor, 16)?))
    }
}
/// [SensorPairingDate: appendix 2.146.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24438)
pub type SensorPairingDate = TimeReal;
