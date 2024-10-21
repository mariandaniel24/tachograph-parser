#![allow(dead_code)]
use super::*;
use crate::bytes::{extract_u8_bits_into_tup, TakeExact};
use anyhow::{Context, Result};
use byteorder::{BigEndian, ReadBytesExt};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use std::{any::type_name, io::Read};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [RecordType: appendix 2.120.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23342)
pub enum RecordTypeGen2 {
    ActivityChangeInfo,
    CardSlotsStatus,
    CurrentDateTime,
    MemberStateCertificate,
    OdometerValueMidnight,
    DateOfDayDownloaded,
    SensorPaired,
    Signature,
    SpecificConditionRecord,
    VehicleIdentificationNumber,
    VehicleRegistrationNumber,
    VuCalibrationRecord,
    VuCardIWRecord,
    VuCardRecord,
    VuCertificate,
    VuCompanyLocksRecord,
    VuControlActivityRecord,
    VuDetailedSpeedBlock,
    VuDownloadablePeriod,
    VuDownloadActivityData,
    VuEventRecord,
    VuGNSSADRecord,
    VuITSConsentRecord,
    VuFaultRecord,
    VuIdentification,
    VuOverSpeedingControlData,
    VuOverSpeedingEventRecord,
    VuPlaceDailyWorkPeriodRecord,
    VuTimeAdjustmentGNSSRecord,
    VuTimeAdjustmentRecord,
    VuPowerSupplyInterruptionRecord,
    SensorPairedRecord,
    SensorExternalGNSSCoupledRecord,
    RFU,
    ManufacturerSpecific,
}

impl RecordTypeGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let record_type = cursor.read_u8().context("Failed to read record type")?;
        match record_type {
            0x00 => anyhow::bail!(
                "Detected record_type 0x00, this is not a valid record_type according to the spec"
            ),
            0x01 => Ok(RecordTypeGen2::ActivityChangeInfo),
            0x02 => Ok(RecordTypeGen2::CardSlotsStatus),
            0x03 => Ok(RecordTypeGen2::CurrentDateTime),
            0x04 => Ok(RecordTypeGen2::MemberStateCertificate),
            0x05 => Ok(RecordTypeGen2::OdometerValueMidnight),
            0x06 => Ok(RecordTypeGen2::DateOfDayDownloaded),
            0x07 => Ok(RecordTypeGen2::SensorPaired),
            0x08 => Ok(RecordTypeGen2::Signature),
            0x09 => Ok(RecordTypeGen2::SpecificConditionRecord),
            0x0A => Ok(RecordTypeGen2::VehicleIdentificationNumber),
            0x0B => Ok(RecordTypeGen2::VehicleRegistrationNumber),
            0x0C => Ok(RecordTypeGen2::VuCalibrationRecord),
            0x0D => Ok(RecordTypeGen2::VuCardIWRecord),
            0x0E => Ok(RecordTypeGen2::VuCardRecord),
            0x0F => Ok(RecordTypeGen2::VuCertificate),
            0x10 => Ok(RecordTypeGen2::VuCompanyLocksRecord),
            0x11 => Ok(RecordTypeGen2::VuControlActivityRecord),
            0x12 => Ok(RecordTypeGen2::VuDetailedSpeedBlock),
            0x13 => Ok(RecordTypeGen2::VuDownloadablePeriod),
            0x14 => Ok(RecordTypeGen2::VuDownloadActivityData),
            0x15 => Ok(RecordTypeGen2::VuEventRecord),
            0x16 => Ok(RecordTypeGen2::VuGNSSADRecord),
            0x17 => Ok(RecordTypeGen2::VuITSConsentRecord),
            0x18 => Ok(RecordTypeGen2::VuFaultRecord),
            0x19 => Ok(RecordTypeGen2::VuIdentification),
            0x1A => Ok(RecordTypeGen2::VuOverSpeedingControlData),
            0x1B => Ok(RecordTypeGen2::VuOverSpeedingEventRecord),
            0x1C => Ok(RecordTypeGen2::VuPlaceDailyWorkPeriodRecord),
            0x1D => Ok(RecordTypeGen2::VuTimeAdjustmentGNSSRecord),
            0x1E => Ok(RecordTypeGen2::VuTimeAdjustmentRecord),
            0x1F => Ok(RecordTypeGen2::VuPowerSupplyInterruptionRecord),
            0x20 => Ok(RecordTypeGen2::SensorPairedRecord),
            0x21 => Ok(RecordTypeGen2::SensorExternalGNSSCoupledRecord),
            0x22..=0x7F => Ok(RecordTypeGen2::RFU),
            0x80..=0xFF => Ok(RecordTypeGen2::ManufacturerSpecific),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]

/// A generic implementation for an array of records, where the record type is parameterized
/// This helper is used across various Vu blocks to parse and store their respective records
pub struct RecordArray<T> {
    record_type: RecordTypeGen2,
    record_size: u16,
    no_of_records: u16,
    pub records: Vec<T>,
}

impl<T> RecordArray<T> {
    pub fn parse<F>(cursor: &mut Cursor<&[u8]>, parse_record: F) -> Result<Self>
    where
        F: Fn(&mut Cursor<&[u8]>) -> Result<T>,
    {
        let record_type = RecordTypeGen2::parse(cursor).context("Failed to parse record type")?;
        let record_size = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read record size")?;
        let no_of_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read number of records")?;

        let data_size = record_size as usize * no_of_records as usize;

        let mut raw_data = vec![0u8; data_size];
        cursor
            .read_exact(&mut raw_data)
            .context("Failed to read raw data for record array")?;

        let mut records = Vec::with_capacity(no_of_records as usize);
        for (index, chunk) in raw_data.chunks(record_size as usize).enumerate() {
            let mut inner_cursor = Cursor::new(chunk);
            let initial_position = inner_cursor.position();

            let record = parse_record(&mut inner_cursor).with_context(|| {
                format!(
                    "Failed to parse record of type {} at index {}",
                    type_name::<T>(),
                    index
                )
            })?;

            let consumed = inner_cursor.position() - initial_position;
            if consumed < record_size as u64 {
                let unused_bytes = record_size as u64 - consumed;
                log::warn!(
                    "Record of type {} did not consume all bytes. Expected to consume {} bytes, but only consumed {}. {} bytes were unused.",
                    type_name::<T>(),
                    record_size,
                    consumed,
                    unused_bytes
                );
            }

            records.push(record);
        }

        Ok(RecordArray {
            record_type,
            record_size,
            no_of_records,
            records,
        })
    }

    pub fn parse_dyn_size<F>(cursor: &mut Cursor<&[u8]>, parse_record: F) -> Result<Self>
    where
        F: Fn(&mut Cursor<&[u8]>, usize) -> Result<T>,
    {
        let record_type = RecordTypeGen2::parse(cursor).context("Failed to parse record type")?;
        let record_size = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read record size")?;
        let no_of_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read number of records")?;

        let data_size = record_size as usize * no_of_records as usize;

        let mut raw_data = vec![0u8; data_size];
        cursor
            .read_exact(&mut raw_data)
            .context("Failed to read raw data for record array")?;

        let mut records = Vec::with_capacity(no_of_records as usize);
        for (index, chunk) in raw_data.chunks(record_size as usize).enumerate() {
            let mut inner_cursor = Cursor::new(chunk);
            let initial_position = inner_cursor.position();

            let record =
                parse_record(&mut inner_cursor, record_size as usize).with_context(|| {
                    format!(
                        "Failed to parse record of type {} at index {}",
                        type_name::<T>(),
                        index
                    )
                })?;

            let consumed = inner_cursor.position() - initial_position;
            if consumed < record_size as u64 {
                let unused_bytes = record_size as u64 - consumed;
                log::warn!(
                    "Record of type {} did not consume all bytes. Expected to consume {} bytes, but only consumed {}. {} bytes were unused.",
                    type_name::<T>(),
                    record_size,
                    consumed,
                    unused_bytes
                );
            }

            records.push(record);
        }

        Ok(RecordArray {
            record_type,
            record_size,
            no_of_records,
            records,
        })
    }
    pub fn into_inner(self) -> Vec<T> {
        self.records
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [Certificate: appendix 2.41.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18396)
pub struct CertificateGen2 {
    pub value: Vec<u8>,
}
impl CertificateGen2 {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let mut value = vec![0u8; size];
        cursor
            .read_exact(&mut value)
            .context("Failed to read value")?;
        Ok(CertificateGen2 { value })
    }
}

/// [MemberStateCertificate: appendix 2.96.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22309)
pub type MemberStateCertificate = CertificateGen2;

/// [VuCertificate: appendix 2.181.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26086)
pub type VuCertificate = CertificateGen2;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "napi", napi(string_enum = "PascalCase"))]
pub enum EquipmentTypeGen2 {
    Reserved,
    DriverCard,
    WorkshopCard,
    ControlCard,
    CompanyCard,
    ManufacturingCard,
    VehicleUnit,
    MotionSensor,
    GNSSFacility,
    RemoteCommunicationDevice,
    ITSinterfaceModule,
    Plaque,
    M1N1Adapter,
    CAERCA,
    CAMSCA,
    ExternalGNSSConnection,
    Unused,
    DriverCardSign,
    WorkshopCardSign,
    VehicleUnitSign,
    RFU,
}
impl EquipmentTypeGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let equipment_type = cursor.read_u8().context("Failed to read equipment type")?;
        match equipment_type {
            0 => Ok(EquipmentTypeGen2::Reserved),
            1 => Ok(EquipmentTypeGen2::DriverCard),
            2 => Ok(EquipmentTypeGen2::WorkshopCard),
            3 => Ok(EquipmentTypeGen2::ControlCard),
            4 => Ok(EquipmentTypeGen2::CompanyCard),
            5 => Ok(EquipmentTypeGen2::ManufacturingCard),
            6 => Ok(EquipmentTypeGen2::VehicleUnit),
            7 => Ok(EquipmentTypeGen2::MotionSensor),
            8 => Ok(EquipmentTypeGen2::GNSSFacility),
            9 => Ok(EquipmentTypeGen2::RemoteCommunicationDevice),
            10 => Ok(EquipmentTypeGen2::ITSinterfaceModule),
            11 => Ok(EquipmentTypeGen2::Plaque),
            12 => Ok(EquipmentTypeGen2::M1N1Adapter),
            13 => Ok(EquipmentTypeGen2::CAERCA),
            14 => Ok(EquipmentTypeGen2::CAMSCA),
            15 => Ok(EquipmentTypeGen2::ExternalGNSSConnection),
            16 => Ok(EquipmentTypeGen2::Unused),
            17 => Ok(EquipmentTypeGen2::DriverCardSign),
            18 => Ok(EquipmentTypeGen2::WorkshopCardSign),
            19 => Ok(EquipmentTypeGen2::VehicleUnitSign),
            20..=255 => Ok(EquipmentTypeGen2::RFU),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [FullCardNumber: appendix 2.73.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21400)
pub struct FullCardNumberGen2 {
    pub card_type: EquipmentTypeGen2,
    pub card_issuing_member_state: external::NationNumeric,
    pub card_number: CardNumber,
}
impl FullCardNumberGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_type = EquipmentTypeGen2::parse(cursor)?;
        let card_issuing_member_state = external::NationNumeric::parse(cursor)?;

        let card_number = match card_type {
            EquipmentTypeGen2::DriverCard => CardNumber::parse_driver(cursor)?,
            EquipmentTypeGen2::WorkshopCard
            | EquipmentTypeGen2::ControlCard
            | EquipmentTypeGen2::CompanyCard => CardNumber::parse_owner(cursor)?,
            _ => CardNumber::parse_unknown(cursor)?,
        };

        Ok(FullCardNumberGen2 {
            card_type,
            card_issuing_member_state,
            card_number,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [Generation: appendix 2.75.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23342)
pub enum GenerationGen2 {
    Generation1,
    Generation2,
    RFU,
}

impl GenerationGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let generation = cursor.read_u8().context("Failed to read generation")?;

        match generation {
            0x00 => Ok(GenerationGen2::RFU),
            0x01 => Ok(GenerationGen2::Generation1),
            0x02 => Ok(GenerationGen2::Generation2),
            0x03..=0xFF => Ok(GenerationGen2::RFU),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [FullCardNumberAndGeneration: appendix 2.74.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21438)
pub struct FullCardNumberAndGenerationGen2 {
    pub full_card_number: FullCardNumberGen2,
    pub generation: GenerationGen2,
}
impl FullCardNumberAndGenerationGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Option<Self> {
        let full_card_number = match FullCardNumberGen2::parse(cursor) {
            Ok(number) => number,
            Err(_) => return None,
        };
        let generation = match GenerationGen2::parse(cursor) {
            Ok(gen) => gen,
            Err(_) => return None,
        };
        let value = match generation {
            GenerationGen2::RFU => None,
            _ => Some(FullCardNumberAndGenerationGen2 {
                full_card_number,
                generation,
            }),
        };
        value
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [ControlType: appendix 2.53.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e19148)
pub struct ControlTypeGen2 {
    pub card_downloading: bool,
    pub vu_downloading: bool,
    pub printing: bool,
    pub display: bool,
    pub roadside_calibration_checking: bool,
}
impl ControlTypeGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let control_type_byte = cursor.read_u8().context("Failed to read control type")?;

        let bits = extract_u8_bits_into_tup(control_type_byte);

        Ok(ControlTypeGen2 {
            card_downloading: bits.0 == 1,
            vu_downloading: bits.1 == 1,
            printing: bits.2 == 1,
            display: bits.3 == 1,
            roadside_calibration_checking: bits.4 == 1,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [Signature: appendix 2.149.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24501)
pub struct SignatureGen2 {
    pub value: Vec<u8>,
} // Octet string
impl SignatureGen2 {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        if size < 64 || size > 132 {
            anyhow::bail!("expected signature size to be 64..132 bytes, got {}", size);
        }
        let mut signature_buffer = vec![0u8; size];
        cursor
            .read_exact(&mut signature_buffer)
            .context("Failed to read signature buffer")?;
        Ok(SignatureGen2 {
            value: signature_buffer,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [PreviousVehicleInfo: appendix 2.118.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23250)
pub struct PreviousVehicleInfoGen2 {
    pub vehicle_registration_identification: VehicleRegistrationIdentification,
    pub card_withdrawal_time: Option<TimeReal>,
    pub vu_generation: GenerationGen2,
}
impl PreviousVehicleInfoGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let vehicle_registration_identification = VehicleRegistrationIdentification::parse(cursor)?;
        let card_withdrawal_time = TimeReal::parse(cursor).ok();
        let vu_generation = GenerationGen2::parse(cursor)?;
        Ok(PreviousVehicleInfoGen2 {
            vehicle_registration_identification,
            card_withdrawal_time,
            vu_generation,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [GNSSPlaceRecord: appendix 2.80.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21772)
pub struct GNSSPlaceRecordGen2 {
    pub time_stamp: TimeReal,
    pub gnss_accuracy: GNSSAccuracyGen2,
    pub geo_coordinates: GeoCoordinatesGen2,
}
impl GNSSPlaceRecordGen2 {
    const SIZE: usize = 7;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_stamp = TimeReal::parse(cursor)?;
        let gnss_accuracy = GNSSAccuracyGen2::parse(cursor)?;
        let geo_coordinates = GeoCoordinatesGen2::parse(cursor)?;

        Ok(GNSSPlaceRecordGen2 {
            time_stamp,
            gnss_accuracy,
            geo_coordinates,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [GNSSAccuracy: appendix 2.77.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21573)
pub struct GNSSAccuracyGen2 {
    pub value: u8,
}
impl GNSSAccuracyGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor.read_u8().context("Failed to read GNSSAccuracy")?;
        if value > 100 {
            anyhow::bail!("Invalid GNSSAccuracy");
        }
        Ok(GNSSAccuracyGen2 { value })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [GeoCoordinates: appendix 2.76.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21534)
pub struct GeoCoordinatesGen2 {
    pub latitude: f64,
    pub longitude: f64,
}
impl GeoCoordinatesGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut lat_buffer = [0u8; 3];
        cursor
            .read_exact(&mut lat_buffer)
            .context("Failed to read latitude for GeoCoordinates")?;

        let mut lon_buffer = [0u8; 3];
        cursor
            .read_exact(&mut lon_buffer)
            .context("Failed to read longitude for GeoCoordinates")?;

        let latitude = Self::decode_coordinate(&lat_buffer);
        let longitude = Self::decode_coordinate(&lon_buffer);

        Ok(GeoCoordinatesGen2 {
            latitude,
            longitude,
        })
    }

    fn decode_coordinate(buffer: &[u8; 3]) -> f64 {
        let fill_byte = if (buffer[0] & 0x80) > 0 { 0xff } else { 0x00 };
        let value = i32::from_be_bytes([fill_byte, buffer[0], buffer[1], buffer[2]]);

        // Convert from DDDMM.M * 10 format to decimal degrees
        let str_value = format!("{:+07}", value);
        if str_value.len() == 7 {
            if let (Ok(deg), Ok(min_ten)) = (
                str_value[0..4].parse::<i32>(),
                str_value[4..7].parse::<i32>(),
            ) {
                let result = f64::from(deg) + (f64::from(min_ten) / 10.0) / 60.0;
                return (result * 10000.0).trunc() / 10000.0;
            }
        }

        // Return 0.0 if parsing fails
        0.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [VuGNSSADRecord: appendix 2.203.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27345)
pub struct VuGNSSADRecordGen2 {
    pub time_stamp: TimeReal,
    pub card_number_and_gen_driver_slot: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot: Option<FullCardNumberAndGenerationGen2>,
    pub gnss_place_record: GNSSPlaceRecordGen2,
    pub vehicle_odometer_value: OdometerShort,
}
impl VuGNSSADRecordGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_stamp = TimeReal::parse(cursor)?;
        let card_number_and_gen_driver_slot = FullCardNumberAndGenerationGen2::parse(cursor)
            .context("Failed to parse card_number_and_gen_driver_slot")
            .ok();
        let card_number_and_gen_codriver_slot = FullCardNumberAndGenerationGen2::parse(cursor)
            .context("Failed to parse card_number_and_gen_codriver_slot")
            .ok();
        let gnss_place_record = GNSSPlaceRecordGen2::parse(cursor)?;
        let vehicle_odometer_value = OdometerShort::parse(cursor)?;

        Ok(VuGNSSADRecordGen2 {
            time_stamp,
            card_number_and_gen_driver_slot,
            card_number_and_gen_codriver_slot,
            gnss_place_record,
            vehicle_odometer_value,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [EntryTypeDailyWorkPeriod: appendix 2.66.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e20044)
pub enum EntryTypeDailyWorkPeriodGen2 {
    BeginRelatedTimeCardInsertionTimeOrTimeOfEntry,
    EndRelatedTimeCardWithdrawalTimeOrTimeOfEntry,
    BeginRelatedTimeManuallyEntered,
    EndRelatedTimeManuallyEntered,
}

impl EntryTypeDailyWorkPeriodGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to read EntryTypeDailyWorkPeriod")?;
        match value {
            0x00 => Ok(Self::BeginRelatedTimeCardInsertionTimeOrTimeOfEntry),
            0x01 => Ok(Self::EndRelatedTimeCardWithdrawalTimeOrTimeOfEntry),
            0x02 => Ok(Self::BeginRelatedTimeManuallyEntered),
            0x03 => Ok(Self::EndRelatedTimeManuallyEntered),
            _ => anyhow::bail!("Invalid EntryTypeDailyWorkPeriod"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [PlaceRecord: appendix 2.117.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23112)
pub struct PlaceRecordGen2 {
    pub entry_time: TimeReal,
    pub entry_type_daily_work_period: EntryTypeDailyWorkPeriodGen2,
    pub daily_work_period_country: external::NationNumeric,
    pub daily_work_period_region: external::RegionNumeric,
    pub vehicle_odometer_value: OdometerShort,
    pub entry_gnss_place_record: GNSSPlaceRecordGen2,
}
impl PlaceRecordGen2 {
    const SIZE: usize = 21;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        let entry_time = TimeReal::parse(inner_cursor)?;
        let entry_type_daily_work_period = EntryTypeDailyWorkPeriodGen2::parse(inner_cursor)?;
        let daily_work_period_country = external::NationNumeric::parse(inner_cursor)?;
        let daily_work_period_region = external::RegionNumeric::parse(inner_cursor)?;
        let vehicle_odometer_value = OdometerShort::parse(inner_cursor)?;
        let entry_gnss_place_record = GNSSPlaceRecordGen2::parse(inner_cursor)?;
        if entry_time.value.timestamp() == 0 {
            anyhow::bail!("Invalid entry_time in PlaceRecord");
        }
        Ok(PlaceRecordGen2 {
            entry_time,
            entry_type_daily_work_period,
            daily_work_period_country,
            daily_work_period_region,
            vehicle_odometer_value,
            entry_gnss_place_record,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [SpecificConditionType: appendix 2.154.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24685)
pub enum SpecificConditionTypeGen2 {
    RFU,
    OutOfScopeBegin,
    OutOfScopeEnd,
    FerryTrainCrossingBegin,
    FerryTrainCrossingEnd,
}

impl SpecificConditionTypeGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to read value for SpecificConditionType")?;
        match value {
            0x0 => Ok(Self::RFU),
            0x1 => Ok(Self::OutOfScopeBegin),
            0x2 => Ok(Self::OutOfScopeEnd),
            0x3 => Ok(Self::FerryTrainCrossingBegin),
            0x4 => Ok(Self::FerryTrainCrossingEnd),
            0x5..=0xFF => Ok(Self::RFU),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [SpecificConditionRecord: appendix 2.152.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24614)
pub struct SpecificConditionRecordGen2 {
    pub entry_time: TimeReal,
    pub specific_condition_type: SpecificConditionTypeGen2,
}
impl SpecificConditionRecordGen2 {
    const SIZE: usize = 5;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        let entry_time = TimeReal::parse(inner_cursor)?;
        let specific_condition_type = SpecificConditionTypeGen2::parse(inner_cursor)?;
        Ok(SpecificConditionRecordGen2 {
            entry_time,
            specific_condition_type,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [EventFaultType: appendix 2.70.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e20338)
pub enum EventFaultTypeGen2 {
    NoFurtherDetails,
    InsertionOfNonValidCard,
    CardConflict,
    TimeOverlap,
    DrivingWithoutAppropriateCard,
    CardInsertionWhileDriving,
    LastCardSessionNotCorrectlyClosed,
    OverSpeeding,
    PowerSupplyInterruption,
    MotionDataError,
    VehicleMotionConflict,
    TimeConflict,
    CommunicationErrorWithRemoteCommunicationFacility,
    AbsenceOfPositionInfoFromGNSSReceiver,
    CommunicationErrorWithExternalGNSSFacility,
    VUSecurityBreachAttemptNoFurtherDetails,
    MotionSensorAuthenticationFailure,
    TachographCardAuthenticationFailure,
    UnauthorizedChangeOfMotionSensor,
    CardDataInputIntegrityError,
    StoredUserDataIntegrityError,
    InternalDataTransferError,
    UnauthorizedCaseOpening,
    HardwareSabotage,
    TamperDetectionOfGNSS,
    ExternalGNSSFacilityAuthenticationFailure,
    ExternalGNSSFacilityCertificateExpired,
    SensorSecurityBreachAttemptNoFurtherDetails,
    SensorAuthenticationFailure,
    SensorStoredDataIntegrityError,
    SensorInternalDataTransferError,
    SensorUnauthorizedCaseOpening,
    SensorHardwareSabotage,
    ControlDeviceFaultNoFurtherDetails,
    VUInternalFault,
    PrinterFault,
    DisplayFault,
    DownloadingFault,
    SensorFault,
    InternalGNSSReceiver,
    ExternalGNSSFacility,
    RemoteCommunicationFacility,
    ITSInterface,
    CardFaultNoFurtherDetails,
    RFU,
    ManufacturerSpecific,
}

impl EventFaultTypeGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to read value for EventFaultType")?;
        match value {
            // General events,
            0x00 => Ok(Self::NoFurtherDetails),
            0x01 => Ok(Self::InsertionOfNonValidCard),
            0x02 => Ok(Self::CardConflict),
            0x03 => Ok(Self::TimeOverlap),
            0x04 => Ok(Self::DrivingWithoutAppropriateCard),
            0x05 => Ok(Self::CardInsertionWhileDriving),
            0x06 => Ok(Self::LastCardSessionNotCorrectlyClosed),
            0x07 => Ok(Self::OverSpeeding),
            0x08 => Ok(Self::PowerSupplyInterruption),
            0x09 => Ok(Self::MotionDataError),
            0x0A => Ok(Self::VehicleMotionConflict),
            0x0B => Ok(Self::TimeConflict),
            0x0C => Ok(Self::CommunicationErrorWithRemoteCommunicationFacility),
            0x0D => Ok(Self::AbsenceOfPositionInfoFromGNSSReceiver),
            0x0E => Ok(Self::CommunicationErrorWithExternalGNSSFacility),
            0x0F => Ok(Self::RFU),

            // Vehicle unit related security breach attempt events,
            0x10 => Ok(Self::VUSecurityBreachAttemptNoFurtherDetails),
            0x11 => Ok(Self::MotionSensorAuthenticationFailure),
            0x12 => Ok(Self::TachographCardAuthenticationFailure),
            0x13 => Ok(Self::UnauthorizedChangeOfMotionSensor),
            0x14 => Ok(Self::CardDataInputIntegrityError),
            0x15 => Ok(Self::StoredUserDataIntegrityError),
            0x16 => Ok(Self::InternalDataTransferError),
            0x17 => Ok(Self::UnauthorizedCaseOpening),
            0x18 => Ok(Self::HardwareSabotage),
            0x19 => Ok(Self::TamperDetectionOfGNSS),
            0x1A => Ok(Self::ExternalGNSSFacilityAuthenticationFailure),
            0x1B => Ok(Self::ExternalGNSSFacilityCertificateExpired),
            0x1C..=0x1F => Ok(Self::RFU),

            // Sensor related security breach attempt events,
            0x20 => Ok(Self::SensorSecurityBreachAttemptNoFurtherDetails),
            0x21 => Ok(Self::SensorAuthenticationFailure),
            0x22 => Ok(Self::SensorStoredDataIntegrityError),
            0x23 => Ok(Self::SensorInternalDataTransferError),
            0x24 => Ok(Self::SensorUnauthorizedCaseOpening),
            0x25 => Ok(Self::SensorHardwareSabotage),
            0x26..=0x2F => Ok(Self::RFU),

            // Recording equipment faults,
            0x30 => Ok(Self::ControlDeviceFaultNoFurtherDetails),
            0x31 => Ok(Self::VUInternalFault),
            0x32 => Ok(Self::PrinterFault),
            0x33 => Ok(Self::DisplayFault),
            0x34 => Ok(Self::DownloadingFault),
            0x35 => Ok(Self::SensorFault),
            0x36 => Ok(Self::InternalGNSSReceiver),
            0x37 => Ok(Self::ExternalGNSSFacility),
            0x38 => Ok(Self::RemoteCommunicationFacility),
            0x39 => Ok(Self::ITSInterface),
            0x3A..=0x3F => Ok(Self::RFU),

            // Card faults,
            0x40 => Ok(Self::CardFaultNoFurtherDetails),
            0x41..=0x4F => Ok(Self::RFU),

            // Reserved for future use,
            0x50..=0x7F => Ok(Self::RFU),

            // Manufacturer specific,
            0x80..=0xFF => Ok(Self::ManufacturerSpecific),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [ManufacturerSpecificEventFaultData: appendix 2.95.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22276)
pub struct ManufacturerSpecificEventFaultDataGen2 {
    pub manufacturer_code: external::ManufacturerCode,
    pub manufacturer_specific_error_code: Vec<u8>,
}
impl ManufacturerSpecificEventFaultDataGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let manufacturer_code = external::ManufacturerCode::parse(cursor).ok();

        let mut manufacturer_specific_error_code = [0u8; 3];
        cursor
            .read_exact(&mut manufacturer_specific_error_code)
            .context("Failed to read manufacturer specific error code")?;

        if manufacturer_code.is_none() {
            anyhow::bail!("Manufacturer code is not present in ManufacturerSpecificEventFaultData");
        }

        Ok(ManufacturerSpecificEventFaultDataGen2 {
            manufacturer_code: manufacturer_code.unwrap(),
            manufacturer_specific_error_code: manufacturer_specific_error_code.to_vec(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [ExtendedSerialNumber: appendix 2.72.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21307)
pub struct ExtendedSerialNumberGen2 {
    pub serial_number: u32,
    pub month_year: MonthYear,
    pub equipment_type: EquipmentTypeGen2,
    pub manufacturer_code: external::ManufacturerCode,
}
impl ExtendedSerialNumberGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let serial_number = cursor
            .read_u32::<BigEndian>()
            .context("Failed to read serial number")?;

        let month_year = MonthYear::parse(cursor)?;
        let equipment_type = EquipmentTypeGen2::parse(cursor)?;
        let manufacturer_code = external::ManufacturerCode::parse(cursor)?;

        Ok(ExtendedSerialNumberGen2 {
            serial_number,
            month_year,
            equipment_type,
            manufacturer_code,
        })
    }
}

/// [VuSerialNumber: appendix 2.223.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28497)
pub type VuSerialNumberGen2 = ExtendedSerialNumberGen2;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [VuApprovalNumber: appendix 2.172.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25427)
pub struct VuApprovalNumberGen2 {
    pub value: IA5String,
}
impl VuApprovalNumberGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value =
            IA5String::parse_dyn_size(cursor, 16).context("Failed to parse VuApprovalNumber")?;
        Ok(VuApprovalNumberGen2 { value })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [VuAbility: appendix 2.169.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25277)
pub enum VuAbilityGen2 {
    SupportsGen1,
    SupportsGen2,
    RFU,
}

impl VuAbilityGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor.read_u8().context("Failed to read VuAbility")?;

        match extract_u8_bits_into_tup(value) {
            // TODO: check if the order is correct
            (_, _, _, _, _, _, _, 0) => Ok(VuAbilityGen2::SupportsGen1),
            (_, _, _, _, _, _, _, 1) => Ok(VuAbilityGen2::SupportsGen2),
            _ => Ok(VuAbilityGen2::RFU),
        }
    }
}
/// [SensorSerialNumber: appendix 2.148.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24483)
pub type SensorSerialNumberGen2 = ExtendedSerialNumberGen2;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [SensorApprovalNumber: appendix 2.131.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23887)
pub struct SensorApprovalNumberGen2 {
    pub value: IA5String,
}
impl SensorApprovalNumberGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = IA5String::parse_dyn_size(cursor, 16)
            .context("Failed to parse SensorApprovalNumber")?;
        Ok(SensorApprovalNumberGen2 { value })
    }
}

/// [SensorGNSSSerialNumber: appendix 2.139.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24175)
pub type SensorGNSSSerialNumber = ExtendedSerialNumberGen2;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [SensorExternalGNSSApprovalNumber: appendix 2.132.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23931)
pub struct SensorExternalGNSSApprovalNumberGen2 {
    pub value: IA5String,
}
impl SensorExternalGNSSApprovalNumberGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = IA5String::parse_dyn_size(cursor, 16)
            .context("Failed to parse SensorExternalGNSSApprovalNumber")?;
        Ok(SensorExternalGNSSApprovalNumberGen2 { value })
    }
}

pub type SensorGNSSCouplingDate = TimeReal;
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(string_enum))]
/// [CalibrationPurpose: appendix 2.8.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16597)
pub enum CalibrationPurposeGen2 {
    Reserved,
    Activation,
    FirstInstallation,
    Installation,
    PeriodicInspection,
    EntryOfVRNByCompany,
    TimeAdjustmentWithoutCalibration,
    RFU,
    ManufacturerSpecific,
}

impl CalibrationPurposeGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to read CalibrationPurpose")?;
        let purpose = match value {
            0x00 => CalibrationPurposeGen2::Reserved,
            0x01 => CalibrationPurposeGen2::Activation,
            0x02 => CalibrationPurposeGen2::FirstInstallation,
            0x03 => CalibrationPurposeGen2::Installation,
            0x04 => CalibrationPurposeGen2::PeriodicInspection,
            0x05 => CalibrationPurposeGen2::EntryOfVRNByCompany,
            0x06 => CalibrationPurposeGen2::TimeAdjustmentWithoutCalibration,
            0x07..=0x7F => CalibrationPurposeGen2::RFU,
            0x80..=0xFF => CalibrationPurposeGen2::ManufacturerSpecific,
        };
        Ok(purpose)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [ExtendedSealIdentifier: appendix 2.71.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21276)
pub struct ExtendedSealIdentifierGen2 {
    pub manufacturer_code: Vec<u8>,
    pub seal_identifier: Vec<u8>,
}
impl ExtendedSealIdentifierGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut manufacturer_code = [0u8; 2];
        cursor
            .read_exact(&mut manufacturer_code)
            .context("Failed to read manufacturer code")?;

        let mut seal_identifier = [0u8; 8];
        cursor
            .read_exact(&mut seal_identifier)
            .context("Failed to read seal identifier")?;

        Ok(ExtendedSealIdentifierGen2 {
            manufacturer_code: manufacturer_code.to_vec(),
            seal_identifier: seal_identifier.to_vec(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [SealRecord: appendix 2.130.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23854)
pub struct SealRecordGen2 {
    pub equipment_type: EquipmentTypeGen2,
    pub extended_seal_identifier: ExtendedSealIdentifierGen2,
}

impl SealRecordGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(SealRecordGen2 {
            equipment_type: EquipmentTypeGen2::parse(cursor)?,
            extended_seal_identifier: ExtendedSealIdentifierGen2::parse(cursor)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [SealDataVu: appendix 2.129.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23827)
pub struct SealDataVuGen2 {
    pub seal_records: Vec<SealRecordGen2>,
}

impl SealDataVuGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut seal_records = Vec::new();
        for _ in 0..5 {
            let seal = SealRecordGen2::parse(cursor)?;
            // if equipment type is not unused, then it is a valid seal, see page 50
            if seal.equipment_type != EquipmentTypeGen2::Unused {
                seal_records.push(seal);
            }
        }
        Ok(SealDataVuGen2 { seal_records })
    }
}

/// [NoOfEventsPerType: appendix 2.109.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22706)
pub type NoOfEventsPerTypeGen2 = u8;
/// [NoOfFaultsPerType: appendix 2.110.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22729)
pub type NoOfFaultsPerTypeGen2 = u8;
/// [NoOfCardVehicleRecords: appendix 2.105.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22612)
pub type NoOfCardVehicleRecordsGen2 = u16;
/// [NoOfCardPlaceRecords: appendix 2.104.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22566)
pub type NoOfCardPlaceRecordsGen2 = u16;
/// [NoOfGnssAdRecords: appendix 2.111.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22756)
pub type NoOfGnssAdRecordsGen2 = u16;
/// [NoOfSpecificConditionRecords: appendix 2.112.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22807)
pub type NoOfSpecificConditionRecordsGen2 = u16;
/// [NoOfCardVehicleUnitRecords: appendix 2.106.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22635)
pub type NoOfCardVehicleUnitRecordsGen2 = u16;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [DriverCardApplicationIdentification: appendix 2.61.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e19751)
#[cfg_attr(feature = "napi", napi(object))]

pub struct DriverCardApplicationIdentificationGen2 {
    pub type_of_tachograph_card_id: EquipmentTypeGen2,
    pub card_structure_version: CardStructureVersion,
    pub no_of_events_per_type: NoOfEventsPerTypeGen2,
    pub no_of_faults_per_type: NoOfFaultsPerTypeGen2,
    pub activity_structure_length: CardActivityLengthRange,
    pub no_of_card_vehicle_records: NoOfCardVehicleRecordsGen2,
    pub no_of_card_place_records: NoOfCardPlaceRecordsGen2,
    pub no_of_gnss_ad_records: NoOfGnssAdRecordsGen2,
    pub no_of_specific_condition_records: NoOfSpecificConditionRecordsGen2,
    pub no_of_card_vehicle_unit_records: NoOfCardVehicleUnitRecordsGen2,
}

impl DriverCardApplicationIdentificationGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let type_of_tachograph_card_id = EquipmentTypeGen2::parse(cursor)?;

        let card_structure_version = CardStructureVersion::parse(cursor)?;

        let no_of_events_per_type = cursor
            .read_u8()
            .context("Failed to read no_of_events_per_type")?;

        let no_of_faults_per_type = cursor
            .read_u8()
            .context("Failed to read no_of_faults_per_type")?;

        let activity_structure_length = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read activity_structure_length")?;

        let no_of_card_vehicle_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read no_of_card_vehicle_records")?;

        let no_of_card_place_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read no_of_card_place_records")?;

        let no_of_gnss_ad_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read no_of_gnss_ad_records")?;

        let no_of_specific_condition_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read no_of_specific_condition_records")?;

        let no_of_card_vehicle_unit_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read no_of_card_vehicle_unit_records")?;

        Ok(DriverCardApplicationIdentificationGen2 {
            type_of_tachograph_card_id,
            card_structure_version,
            no_of_events_per_type,
            no_of_faults_per_type,
            activity_structure_length,
            no_of_card_vehicle_records,
            no_of_card_place_records,
            no_of_gnss_ad_records,
            no_of_specific_condition_records,
            no_of_card_vehicle_unit_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ApplicationIdentificationGen2 {
    pub driver_card_application_identification: DriverCardApplicationIdentificationGen2,
}
impl ApplicationIdentificationGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let driver_card_application_identification =
            DriverCardApplicationIdentificationGen2::parse(cursor)?;
        Ok(ApplicationIdentificationGen2 {
            driver_card_application_identification,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [CardIccIdentification: appendix 2.23.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17372)
pub struct CardIccIdentificationGen2 {
    pub clock_stop: u8,
    pub card_extended_serial_number: ExtendedSerialNumberGen2,
    pub card_approval_number: CardApprovalNumber,
    pub card_personaliser_id: external::ManufacturerCode,
    pub embedder_ic_assembler_id: EmbedderIcAssemblerId,
    pub ic_identifier: Vec<u8>,
}
impl CardIccIdentificationGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let clock_stop = cursor.read_u8().context("Failed to read clock_stop")?;
        let card_extended_serial_number = ExtendedSerialNumberGen2::parse(cursor)?;
        let card_approval_number = CardApprovalNumber::parse(cursor)?;
        let card_personaliser_id = external::ManufacturerCode::parse(cursor)?;
        let embedder_ic_assembler_id = EmbedderIcAssemblerId::parse(cursor)?;
        let mut buffer = [0u8; 2];

        cursor
            .read_exact(&mut buffer)
            .context("Failed to read ic_identifier")?;
        let ic_identifier = [buffer[0], buffer[1]];

        Ok(CardIccIdentificationGen2 {
            clock_stop,
            card_extended_serial_number,
            card_approval_number,
            card_personaliser_id,
            embedder_ic_assembler_id,
            ic_identifier: ic_identifier.to_vec(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [CardEventRecord: appendix 2.20.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17247)
pub struct CardEventRecordGen2 {
    pub event_type: EventFaultTypeGen2,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub event_vehicle_registration: VehicleRegistrationIdentification,
}

impl CardEventRecordGen2 {
    const SIZE: usize = 24;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        let event_type = EventFaultTypeGen2::parse(inner_cursor)?;
        let event_begin_time = TimeReal::parse(inner_cursor)?;
        let event_end_time = TimeReal::parse(inner_cursor)?;
        let event_vehicle_registration = VehicleRegistrationIdentification::parse(inner_cursor)?;

        Ok(CardEventRecordGen2 {
            event_type,
            event_begin_time,
            event_end_time,
            event_vehicle_registration,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [CardEventData: appendix 2.19.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17180)
pub struct CardEventDataGen2 {
    pub value: Vec<Vec<CardEventRecordGen2>>,
}
impl CardEventDataGen2 {
    const OUTER_RECORDS_AMOUNT: usize = 11;
    const INNER_RECORDS_AMOUNT: usize = 1;

    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let mut card_event_records = Vec::new();
        let inner_record_amounts = size / Self::OUTER_RECORDS_AMOUNT / CardEventRecordGen2::SIZE;

        for _ in 0..Self::OUTER_RECORDS_AMOUNT {
            let mut inner_card_event_records = Vec::new();
            for _ in 0..inner_record_amounts {
                if let Ok(card_event_record) = CardEventRecordGen2::parse(cursor) {
                    inner_card_event_records.push(card_event_record);
                }
            }
            // Only include the records if there are any
            if inner_card_event_records.len() > 0 {
                card_event_records.push(inner_card_event_records);
            }
        }
        Ok(CardEventDataGen2 {
            value: card_event_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [CardFaultData: appendix 2.21.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17292)
pub struct CardFaultRecordGen2 {
    pub fault_type: EventFaultTypeGen2,
    pub fault_begin_time: TimeReal,
    pub fault_end_time: TimeReal,
    pub fault_vehicle_registration: VehicleRegistrationIdentification,
}

impl CardFaultRecordGen2 {
    pub const SIZE: usize = 24;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        let fault_type = EventFaultTypeGen2::parse(inner_cursor)?;
        let fault_begin_time = TimeReal::parse(inner_cursor)?;
        let fault_end_time = TimeReal::parse(inner_cursor)?;
        let fault_vehicle_registration = VehicleRegistrationIdentification::parse(inner_cursor)?;

        Ok(CardFaultRecordGen2 {
            fault_type,
            fault_begin_time,
            fault_end_time,
            fault_vehicle_registration,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [CardFaultData: appendix 2.22.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17340)
pub struct CardFaultDataGen2 {
    pub value: Vec<Vec<CardFaultRecordGen2>>,
}
impl CardFaultDataGen2 {
    const MAX_BLOCK_SIZE: usize = 1152;
    const OUTER_RECORDS_AMOUNT: usize = 6;

    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut card_fault_records = Vec::new();

        let max_possible_records = Self::MAX_BLOCK_SIZE / CardFaultRecordGen2::SIZE;
        let max_inner_records = max_possible_records / Self::OUTER_RECORDS_AMOUNT;

        // According to the spec, there are ALWAYS 2 outer CardFaultRecords, but we'll use the computed size just in case
        for _ in 0..Self::OUTER_RECORDS_AMOUNT {
            let mut inner_card_fault_records = Vec::new();
            for _ in 0..max_inner_records {
                match CardFaultRecordGen2::parse(cursor) {
                    Ok(card_fault_record) => inner_card_fault_records.push(card_fault_record),
                    Err(_) => {
                        // log::warn!(
                        //     "Skipping CardFaultRecord at outer idx {:?} and inner idx {:?}: {:?}",
                        //     i,
                        //     j,
                        //     e
                        // );
                        continue;
                    }
                };
            }
            // Only include the records if there are any
            if inner_card_fault_records.len() > 0 {
                card_fault_records.push(inner_card_fault_records);
            }
        }
        Ok(CardFaultDataGen2 {
            value: card_fault_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [CardVehicleRecord: appendix 2.37.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18163)
pub struct CardVehicleRecordGen2 {
    pub vehicle_odometer_begin: OdometerShort,
    pub vehicle_odometer_end: OdometerShort,
    pub vehicle_first_use: TimeReal,
    pub vehicle_last_use: TimeReal,
    pub vehicle_registration: VehicleRegistrationIdentification,
    pub vu_data_block_counter: VuDataBlockCounter,
    pub vehicle_identification_number: VehicleIdentificationNumber,
}
impl CardVehicleRecordGen2 {
    const SIZE: usize = 48;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        Ok(CardVehicleRecordGen2 {
            vehicle_odometer_begin: OdometerShort::parse(inner_cursor)?,
            vehicle_odometer_end: OdometerShort::parse(inner_cursor)?,
            vehicle_first_use: TimeReal::parse(inner_cursor)?,
            vehicle_last_use: TimeReal::parse(inner_cursor)?,
            vehicle_registration: VehicleRegistrationIdentification::parse(inner_cursor)?,
            vu_data_block_counter: VuDataBlockCounter::parse(inner_cursor)?,
            vehicle_identification_number: VehicleIdentificationNumber::parse(inner_cursor)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [CardVehiclesUsed: appendix 2.38.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18302)
pub struct CardVehiclesUsedGen2 {
    pub vehicle_pointer_newest_record: u16,
    pub card_vehicle_records: Vec<CardVehicleRecordGen2>,
}
impl CardVehiclesUsedGen2 {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let cursor = &mut cursor.take_exact(size);
        let vehicle_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read vehicle_pointer_newest_record")?;
        let mut card_vehicle_records = Vec::new();
        let amount_of_records = size as usize / CardVehicleRecordGen2::SIZE as usize;
        for i in 0..amount_of_records {
            if let Ok(card_vehicle_record) = CardVehicleRecordGen2::parse(cursor) {
                card_vehicle_records.push(card_vehicle_record);
            }
            // If we've reached the newest record, break
            if i + 1 == vehicle_pointer_newest_record as usize {
                break;
            }
        }

        Ok(CardVehiclesUsedGen2 {
            vehicle_pointer_newest_record,
            card_vehicle_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [CardPlaceDailyWorkPeriod: appendix 2.27.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17729)
pub struct CardPlaceDailyWorkPeriodGen2 {
    pub place_pointer_newest_record: NoOfCardPlaceRecordsGen2,
    pub place_records: Vec<PlaceRecordGen2>,
}
impl CardPlaceDailyWorkPeriodGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let place_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read place_pointer_newest_record")?;

        let mut place_records = Vec::new();
        let amount_of_records = size as usize / PlaceRecordGen2::SIZE as usize;

        for _ in 0..amount_of_records {
            if let Ok(place_record) = PlaceRecordGen2::parse(cursor) {
                place_records.push(place_record);
            }
        }
        // Sort the records by entry_time in ascending order
        place_records.sort_by(|a, b| {
            a.entry_time
                .value
                .timestamp()
                .cmp(&b.entry_time.value.timestamp())
        });
        Ok(CardPlaceDailyWorkPeriodGen2 {
            place_pointer_newest_record,
            place_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [CardControlActivityDataRecord appendix 2.15.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17002)
pub struct CardControlActivityDataRecordGen2 {
    pub control_type: ControlTypeGen2,
    pub control_time: Option<TimeReal>,
    pub control_card_number: FullCardNumberGen2,
    pub control_vehicle_registration: VehicleRegistrationIdentification,
    pub control_download_period_begin: Option<TimeReal>,
    pub control_download_period_end: Option<TimeReal>,
}
impl CardControlActivityDataRecordGen2 {
    const SIZE: usize = 46;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        Ok(Self {
            control_type: ControlTypeGen2::parse(inner_cursor)?,
            control_time: TimeReal::parse(inner_cursor).ok(),
            control_card_number: FullCardNumberGen2::parse(inner_cursor)?,
            control_vehicle_registration: VehicleRegistrationIdentification::parse(inner_cursor)?,
            control_download_period_begin: TimeReal::parse(inner_cursor).ok(),
            control_download_period_end: TimeReal::parse(inner_cursor).ok(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [SpecificConditions: appendix 2.153.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24644)
pub struct SpecificConditionsGen2 {
    pub condition_pointer_newest_record: NoOfSpecificConditionRecordsGen2,
    pub specific_condition_records: Vec<SpecificConditionRecordGen2>,
}
impl SpecificConditionsGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let condition_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read condition_pointer_newest_record")?;

        let mut specific_condition_records = Vec::new();
        let no_of_records = size / SpecificConditionRecordGen2::SIZE;
        for _ in 0..no_of_records {
            if let Ok(specific_condition_record) = SpecificConditionRecordGen2::parse(cursor) {
                specific_condition_records.push(specific_condition_record);
            }
        }
        // Sort the records by time_stamp in desc order
        specific_condition_records.sort_by(|a, b| {
            b.entry_time
                .value
                .timestamp()
                .cmp(&a.entry_time.value.timestamp())
        });
        Ok(SpecificConditionsGen2 {
            condition_pointer_newest_record,
            specific_condition_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [CardVehicleUnitRecord: appendix 2.39.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18302)
pub struct CardVehicleUnitRecordGen2 {
    pub time_stamp: TimeReal,
    pub manufacturer_code: external::ManufacturerCode,
    pub device_id: u8,
    pub vu_software_version: VuSoftwareVersion,
}
impl CardVehicleUnitRecordGen2 {
    const SIZE: usize = 10;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        let time_stamp = TimeReal::parse(inner_cursor)?;
        let manufacturer_code = external::ManufacturerCode::parse(inner_cursor)?;
        let device_id = inner_cursor.read_u8().context("Failed to read device_id")?;
        let vu_software_version = VuSoftwareVersion::parse(inner_cursor)?;

        if time_stamp.value.timestamp() == 0 {
            return Err(anyhow::anyhow!(
                "failed to parse CardVehicleUnitRecord, too many 0 bytes"
            ));
        }

        Ok(Self {
            time_stamp,
            manufacturer_code,
            device_id,
            vu_software_version,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [CardVehicleUnitsUsed: appendix 2.40.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18350)
pub struct CardVehicleUnitsUsedGen2 {
    pub no_of_card_vehicle_unit_records: NoOfCardVehicleUnitRecordsGen2,
    pub card_vehicle_unit_records: Vec<CardVehicleUnitRecordGen2>,
}
impl CardVehicleUnitsUsedGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let no_of_card_vehicle_unit_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read no_of_card_vehicle_unit_records")?;
        let mut vehicle_units = Vec::new();

        let no_of_records = size / CardVehicleUnitRecordGen2::SIZE;
        for _ in 0..no_of_records {
            if let Ok(vehicle_unit) = CardVehicleUnitRecordGen2::parse(cursor) {
                vehicle_units.push(vehicle_unit);
            }
        }
        // Sort the records by time_stamp in desc order
        vehicle_units.sort_by(|a, b| {
            b.time_stamp
                .value
                .timestamp()
                .cmp(&a.time_stamp.value.timestamp())
        });
        Ok(CardVehicleUnitsUsedGen2 {
            no_of_card_vehicle_unit_records,
            card_vehicle_unit_records: vehicle_units,
        })
    }
}

/// [NoOfGNSSADRecords: appendix 2.111.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22756)
pub type NoOfGNSSADRecords = u16;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [GNSSAccumulatedDrivingRecord: appendix 2.79.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21640)
pub struct GNSSAccumulatedDrivingRecordGen2 {
    pub time_stamp: TimeReal,
    pub gnss_place_record: GNSSPlaceRecordGen2,
    pub vehicle_odometer_value: OdometerShort,
}
impl GNSSAccumulatedDrivingRecordGen2 {
    pub const SIZE: usize = 18;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_stamp = TimeReal::parse(cursor)?;
        let gnss_place_record = GNSSPlaceRecordGen2::parse(cursor)?;
        let vehicle_odometer_value = OdometerShort::parse(cursor)?;

        if time_stamp.value.timestamp() == 0 {
            return Err(anyhow::anyhow!(
                "failed to parse GNSSAccumulatedDrivingRecord, too many 0 bytes"
            ));
        }

        Ok(Self {
            time_stamp,
            gnss_place_record,
            vehicle_odometer_value,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
/// [GNSSAccumulatedDriving: appendix 2.79.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21595)
pub struct GNSSAccumulatedDrivingGen2 {
    pub gnss_ad_pointer_newest_record: NoOfGNSSADRecords,
    pub gnss_accumulated_driving_records: Vec<GNSSAccumulatedDrivingRecordGen2>,
}
impl GNSSAccumulatedDrivingGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(size);
        let gnss_ad_pointer_newest_record = inner_cursor
            .read_u16::<BigEndian>()
            .context("Failed to read gnss_ad_pointer_newest_record")?;

        let mut gnss_accumulated_driving_records = Vec::new();
        let no_of_records = size as usize / GNSSAccumulatedDrivingRecordGen2::SIZE as usize;
        for _ in 0..no_of_records {
            if let Ok(gnss_accumulated_driving_record) =
                GNSSAccumulatedDrivingRecordGen2::parse(inner_cursor)
            {
                gnss_accumulated_driving_records.push(gnss_accumulated_driving_record);
            }
        }
        // Sort the records by time_stamp in ascending order
        gnss_accumulated_driving_records.sort_by(|a, b| {
            a.time_stamp
                .value
                .timestamp()
                .cmp(&b.time_stamp.value.timestamp())
        });
        Ok(GNSSAccumulatedDrivingGen2 {
            gnss_ad_pointer_newest_record,
            gnss_accumulated_driving_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct DateOfDayDownloadedGen2 {
    pub value: TimeReal,
}

impl DateOfDayDownloadedGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_real =
            TimeReal::parse(cursor).context("Failed to parse TimeReal for DateOfDayDownloaded")?;
        Ok(DateOfDayDownloadedGen2 { value: time_real })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuCardIWRecordGen2 {
    pub card_holder_name: HolderName,
    pub full_card_number_and_generation: FullCardNumberAndGenerationGen2,
    pub card_expiry_date: TimeReal,
    pub card_insertion_date: TimeReal,
    pub vehicle_odometer_value_at_insertion: OdometerShort,
    pub card_slot_number: CardSlotNumber,
    pub card_withdrawl_time: Option<TimeReal>,
    pub vehicle_odometer_value_at_withdrawal: OdometerShort,
    pub previous_vehicle_info: PreviousVehicleInfoGen2,
    pub manual_input_flag: ManualInputFlag,
}

impl VuCardIWRecordGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuCardIWRecordGen2 {
            card_holder_name: HolderName::parse(cursor)
                .context("Failed to parse card_holder_name")?,
            full_card_number_and_generation: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse full_card_number_and_generation")?,
            card_expiry_date: TimeReal::parse(cursor)
                .context("Failed to parse card_expiry_date")?,
            card_insertion_date: TimeReal::parse(cursor)
                .context("Failed to parse card_insertion_date")?,
            vehicle_odometer_value_at_insertion: OdometerShort::parse(cursor)
                .context("Failed to parse vehicle_odometer_value_at_insertion")?,
            card_slot_number: CardSlotNumber::parse(cursor)
                .context("Failed to parse card_slot_number")?,
            card_withdrawl_time: TimeReal::parse(cursor).ok(),
            vehicle_odometer_value_at_withdrawal: OdometerShort::parse(cursor)
                .context("Failed to parse vehicle_odometer_value_at_withdrawal")?,
            previous_vehicle_info: PreviousVehicleInfoGen2::parse(cursor)
                .context("Failed to parse previous_vehicle_info")?,
            manual_input_flag: ManualInputFlag::parse(cursor)
                .context("Failed to parse manual_input_flag")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuPlaceDailyWorkPeriodGen2 {
    pub full_card_number_and_generation: Option<FullCardNumberAndGenerationGen2>,
    pub place_record: PlaceRecordGen2,
}

impl VuPlaceDailyWorkPeriodGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuPlaceDailyWorkPeriodGen2 {
            full_card_number_and_generation: FullCardNumberAndGenerationGen2::parse(cursor),
            place_record: PlaceRecordGen2::parse(cursor).context("Failed to parse place_record")?,
        })
    }
}

pub type DateOfDayDownloadedRecordArrayGen2 = Vec<DateOfDayDownloadedGen2>;
pub type OdometerValueMidnightRecordArrayGen2 = Vec<OdometerValueMidnight>;
pub type VuCardIWRecordRecordArrayGen2 = Vec<VuCardIWRecordGen2>;
pub type VuActivityDailyRecordArrayGen2 = Vec<ActivityChangeInfo>;
pub type VuPlaceDailyWorkPeriodRecordArrayGen2 = Vec<VuPlaceDailyWorkPeriodGen2>;
pub type VuGNSSADRecordArrayGen2 = Vec<VuGNSSADRecordGen2>;
pub type VuSpecificConditionRecordArrayGen2 = Vec<SpecificConditionRecordGen2>;
pub type SignatureRecordArrayGen2 = Vec<SignatureGen2>;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuActivitiesBlockGen2 {
    pub date_of_day_downloaded_record_array: DateOfDayDownloadedRecordArrayGen2,
    pub odometer_value_midnight_record_array: OdometerValueMidnightRecordArrayGen2,
    pub vu_card_iw_record_array: VuCardIWRecordRecordArrayGen2,
    pub vu_activity_daily_record_array: VuActivityDailyRecordArrayGen2,
    pub vu_place_daily_work_period_record_array: VuPlaceDailyWorkPeriodRecordArrayGen2,
    pub vu_gnss_ad_record_array: VuGNSSADRecordArrayGen2,
    pub vu_specific_condition_record_array: VuSpecificConditionRecordArrayGen2,
    pub signature_record_array: SignatureRecordArrayGen2,
}

impl VuActivitiesBlockGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuActivitiesBlockGen2 {
            date_of_day_downloaded_record_array: RecordArray::parse(
                cursor,
                DateOfDayDownloadedGen2::parse,
            )
            .context("Failed to parse date_of_day_downloaded_record_array")?
            .into_inner(),

            odometer_value_midnight_record_array: RecordArray::parse(
                cursor,
                OdometerValueMidnight::parse,
            )
            .context("Failed to parse odometer_value_midnight_record_array")?
            .into_inner(),

            vu_card_iw_record_array: RecordArray::parse(cursor, VuCardIWRecordGen2::parse)
                .context("Failed to parse vu_card_iw_record_array")?
                .into_inner(),

            vu_activity_daily_record_array: RecordArray::parse(cursor, ActivityChangeInfo::parse)
                .context("Failed to parse vu_activity_daily_record_array")?
                .into_inner(),

            vu_place_daily_work_period_record_array: RecordArray::parse(
                cursor,
                VuPlaceDailyWorkPeriodGen2::parse,
            )
            .context("Failed to parse vu_place_daily_work_period_record_array")?
            .into_inner(),

            vu_gnss_ad_record_array: RecordArray::parse(cursor, VuGNSSADRecordGen2::parse)
                .context("Failed to parse vu_gnss_ad_record_array")?
                .into_inner(),

            vu_specific_condition_record_array: RecordArray::parse(
                cursor,
                SpecificConditionRecordGen2::parse,
            )
            .context("Failed to parse vu_specific_condition_record_array")?
            .into_inner(),

            signature_record_array: RecordArray::parse_dyn_size(
                cursor,
                SignatureGen2::parse_dyn_size,
            )
            .context("Failed to parse signature_record_array")?
            .into_inner(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]

pub struct VuIdentificationGen2 {
    pub vu_manufacturer_name: VuManufacturerName,
    pub vu_manufacturer_address: VuManufacturerAddress,
    pub vu_part_number: VuPartNumber,
    pub vu_serial_number: VuSerialNumberGen2,
    pub vu_software_identification: VuSoftwareIdentification,
    pub vu_manufacturing_date: VuManufacturingDate,
    pub vu_approval_number: VuApprovalNumberGen2,
    pub vu_generation: GenerationGen2,
    pub vu_ability: VuAbilityGen2,
    // pub vu_digital_map_version: VuDigitalMapVersion, // Only in Gen2V2, but for some reason it's categorized as "Generation 2" unlike other types, EU please.
}
/// [VuIdentification: appendix 2.206.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27574)
impl VuIdentificationGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuIdentificationGen2 {
            vu_manufacturer_name: VuManufacturerName::parse(cursor)
                .context("Failed to parse vu_manufacturer_name")?,
            vu_manufacturer_address: VuManufacturerAddress::parse(cursor)
                .context("Failed to parse vu_manufacturer_address")?,
            vu_part_number: VuPartNumber::parse(cursor)
                .context("Failed to parse vu_part_number")?,
            vu_serial_number: VuSerialNumberGen2::parse(cursor)
                .context("Failed to parse vu_serial_number")?,
            vu_software_identification: VuSoftwareIdentification::parse(cursor)
                .context("Failed to parse vu_software_identification")?,
            vu_manufacturing_date: VuManufacturingDate::parse(cursor)
                .context("Failed to parse vu_manufacturing_date")?,
            vu_approval_number: VuApprovalNumberGen2::parse(cursor)
                .context("Failed to parse vu_approval_number")?,
            vu_generation: GenerationGen2::parse(cursor)
                .context("Failed to parse vu_generation")?,
            vu_ability: VuAbilityGen2::parse(cursor).context("Failed to parse vu_ability")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SensorPairedRecordGen2 {
    pub sensor_serial_number: SensorSerialNumberGen2,
    pub sensor_approval_number: SensorApprovalNumberGen2,
    pub sensor_pairing_date: SensorPairingDate,
}

impl SensorPairedRecordGen2 {
    const SIZE: usize = 14;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(SensorPairedRecordGen2 {
            sensor_serial_number: SensorSerialNumberGen2::parse(cursor)
                .context("Failed to parse sensor_serial_number")?,
            sensor_approval_number: SensorApprovalNumberGen2::parse(cursor)
                .context("Failed to parse sensor_approval_number")?,
            sensor_pairing_date: SensorPairingDate::parse(cursor)
                .context("Failed to parse sensor_pairing_date")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]

pub struct SensorExternalGNSSCoupledRecordGen2 {
    pub sensor_serial_number: SensorGNSSSerialNumber,
    pub sensor_approval_number: SensorExternalGNSSApprovalNumberGen2,
    pub sensor_coupling_date: SensorGNSSCouplingDate,
}

impl SensorExternalGNSSCoupledRecordGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(SensorExternalGNSSCoupledRecordGen2 {
            sensor_serial_number: SensorGNSSSerialNumber::parse(cursor)
                .context("Failed to parse sensor_serial_number")?,
            sensor_approval_number: SensorExternalGNSSApprovalNumberGen2::parse(cursor)
                .context("Failed to parse sensor_approval_number")?,
            sensor_coupling_date: SensorGNSSCouplingDate::parse(cursor)
                .context("Failed to parse sensor_coupling_date")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]

pub struct VuCalibrationRecordGen2 {
    pub calibration_purpose: CalibrationPurposeGen2,
    pub workshop_name: Name,
    pub workshop_address: Address,
    pub workshop_card_number: FullCardNumberGen2,
    pub workshop_card_expiry_date: Option<TimeReal>,
    pub vehicle_identification_number: Option<VehicleIdentificationNumber>,
    pub vehicle_registration_identification: Option<VehicleRegistrationIdentification>,
    pub w_vehicle_characteristic_constant: WVehicleCharacteristicConstant,
    pub k_constant_of_recording_equipment: KConstantOfRecordingEquipment,
    pub l_tyre_circumference: LTyreCircumference,
    pub tyre_size: TyreSize,
    pub authorised_speed: SpeedAuthorised,
    pub old_odometer_value: OdometerShort,
    pub new_odometer_value: OdometerShort,
    pub old_time_value: Option<TimeReal>,
    pub new_time_value: Option<TimeReal>,
    pub next_calibration_date: Option<TimeReal>,
    pub seal_data_vu: SealDataVuGen2,
}

impl VuCalibrationRecordGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuCalibrationRecordGen2 {
            calibration_purpose: CalibrationPurposeGen2::parse(cursor)
                .context("Failed to parse calibration_purpose")?,
            workshop_name: Name::parse(cursor).context("Failed to parse workshop_name")?,
            workshop_address: Address::parse(cursor).context("Failed to parse workshop_address")?,
            workshop_card_number: FullCardNumberGen2::parse(cursor)
                .context("Failed to parse workshop_card_number")?,
            workshop_card_expiry_date: TimeReal::parse(cursor).ok(),
            vehicle_identification_number: VehicleIdentificationNumber::parse(cursor).ok(),
            vehicle_registration_identification: VehicleRegistrationIdentification::parse(cursor)
                .ok(),
            w_vehicle_characteristic_constant: WVehicleCharacteristicConstant::parse(cursor)
                .context("Failed to parse w_vehicle_characteristic_constant")?,
            k_constant_of_recording_equipment: KConstantOfRecordingEquipment::parse(cursor)
                .context("Failed to parse k_constant_of_recording_equipment")?,
            l_tyre_circumference: LTyreCircumference::parse(cursor)
                .context("Failed to parse l_tyre_circumference")?,
            tyre_size: TyreSize::parse(cursor).context("Failed to parse tyre_size")?,
            authorised_speed: SpeedAuthorised::parse(cursor)
                .context("Failed to parse authorised_speed")?,
            old_odometer_value: OdometerShort::parse(cursor)
                .context("Failed to parse old_odometer_value")?,
            new_odometer_value: OdometerShort::parse(cursor)
                .context("Failed to parse new_odometer_value")?,
            old_time_value: TimeReal::parse(cursor).ok(),
            new_time_value: TimeReal::parse(cursor).ok(),
            next_calibration_date: TimeReal::parse(cursor).ok(),
            seal_data_vu: SealDataVuGen2::parse(cursor).context("Failed to parse seal_data_vu")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]

pub struct VuCardRecordGen2 {
    pub card_number_and_generation_information: Option<FullCardNumberAndGenerationGen2>,
    pub card_extended_serial_number: ExtendedSerialNumberGen2,
    pub card_structure_version: CardStructureVersion,
    pub card_number: Option<CardNumber>,
}

impl VuCardRecordGen2 {
    const SIZE: usize = 28;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_number_and_generation_information = FullCardNumberAndGenerationGen2::parse(cursor)
            .context("Failed to parse card_number_and_generation_information")
            .ok();
        let card_extended_serial_number = ExtendedSerialNumberGen2::parse(cursor)
            .context("Failed to parse card_extended_serial_number")?;
        let card_structure_version = CardStructureVersion::parse(cursor)
            .context("Failed to parse card_structure_version")?;

        let card_number = match &card_number_and_generation_information {
            Some(info) => match info.full_card_number.card_type {
                EquipmentTypeGen2::DriverCard => Some(
                    CardNumber::parse_driver(cursor)
                        .context("Failed to parse driver card number")?,
                ),
                EquipmentTypeGen2::WorkshopCard => Some(
                    CardNumber::parse_owner(cursor)
                        .context("Failed to parse workshop card number")?,
                ),
                // Invalid card type, but we still have to consume the bytes
                // Otherwise we might break the parsing of the next records
                _ => {
                    Some(CardNumber::parse_unknown(cursor).context("Failed to parse card number")?)
                }
            },
            None => None,
        };

        Ok(VuCardRecordGen2 {
            card_number_and_generation_information,
            card_extended_serial_number,
            card_structure_version,
            card_number,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]

pub struct VuITSConsentRecordGen2 {
    pub card_number_and_gen: Option<FullCardNumberAndGenerationGen2>,
    pub consent: bool,
}

impl VuITSConsentRecordGen2 {
    const SIZE: usize = 20;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_number_and_gen = FullCardNumberAndGenerationGen2::parse(cursor)
            .context("Failed to parse card_number_and_gen")
            .ok();
        let consent = cursor.read_u8().context("Failed to parse consent")? != 0;
        Ok(VuITSConsentRecordGen2 {
            card_number_and_gen,
            consent,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]

pub struct VuPowerSupplyInterruptionRecordGen2 {
    pub event_type: EventFaultTypeGen2,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub card_number_and_gen_driver_slot_begin: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_driver_slot_end: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot_begin: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot_end: Option<FullCardNumberAndGenerationGen2>,
    pub similar_events_number: SimilarEventsNumber,
}

impl VuPowerSupplyInterruptionRecordGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuPowerSupplyInterruptionRecordGen2 {
            event_type: EventFaultTypeGen2::parse(cursor).context("Failed to parse event_type")?,
            event_record_purpose: EventFaultRecordPurpose::parse(cursor)
                .context("Failed to parse event_record_purpose")?,
            event_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse event_begin_time")?,
            event_end_time: TimeReal::parse(cursor).context("Failed to parse event_end_time")?,
            card_number_and_gen_driver_slot_begin: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_begin")
                .ok(),
            card_number_and_gen_driver_slot_end: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_end")
                .ok(),
            card_number_and_gen_codriver_slot_begin: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_begin")
                .ok(),
            card_number_and_gen_codriver_slot_end: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_end")
                .ok(),
            similar_events_number: SimilarEventsNumber::parse(cursor)
                .context("Failed to parse similar_events_number")?,
        })
    }
}

pub type VuIdentificationRecordArrayGen2 = Vec<VuIdentificationGen2>;
pub type VuSensorPairedRecordArrayGen2 = Vec<SensorPairedRecordGen2>;
pub type VuSensorExternalGNSSCoupledRecordArrayGen2 = Vec<SensorExternalGNSSCoupledRecordGen2>;
pub type VuCalibrationRecordArrayGen2 = Vec<VuCalibrationRecordGen2>;
pub type VuCardRecordArrayGen2 = Vec<VuCardRecordGen2>;
pub type VuITSConsentRecordArrayGen2 = Vec<VuITSConsentRecordGen2>;
pub type VuPowerSupplyInterruptionRecordArrayGen2 = Vec<VuPowerSupplyInterruptionRecordGen2>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]

pub struct VuCompanyLocksBlockGen2 {
    pub vu_identification_record_array: VuIdentificationRecordArrayGen2,
    pub vu_sensor_paired_record_array: VuSensorPairedRecordArrayGen2,
    pub vu_sensor_external_gnss_coupled_record_array: VuSensorExternalGNSSCoupledRecordArrayGen2,
    pub vu_calibration_record_array: VuCalibrationRecordArrayGen2,
    pub vu_card_record_array: VuCardRecordArrayGen2,
    pub vu_its_consent_record_array: VuITSConsentRecordArrayGen2,
    pub vu_power_supply_interruption_record_array: VuPowerSupplyInterruptionRecordArrayGen2,
    pub signature_record_array: SignatureRecordArrayGen2,
}
impl VuCompanyLocksBlockGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuCompanyLocksBlockGen2 {
            vu_identification_record_array: RecordArray::parse(cursor, VuIdentificationGen2::parse)
                .context("Failed to parse vu_identification_record_array")?
                .into_inner(),

            vu_sensor_paired_record_array: RecordArray::parse(
                cursor,
                SensorPairedRecordGen2::parse,
            )
            .context("Failed to parse vu_sensor_paired_record_array")?
            .into_inner(),

            vu_sensor_external_gnss_coupled_record_array: RecordArray::parse(
                cursor,
                SensorExternalGNSSCoupledRecordGen2::parse,
            )
            .context("Failed to parse vu_sensor_external_gnss_coupled_record_array")?
            .into_inner(),

            vu_calibration_record_array: RecordArray::parse(cursor, VuCalibrationRecordGen2::parse)
                .context("Failed to parse vu_calibration_record_array")?
                .into_inner(),

            vu_card_record_array: RecordArray::parse(cursor, VuCardRecordGen2::parse)
                .context("Failed to parse vu_card_record_array")?
                .into_inner(),

            vu_its_consent_record_array: RecordArray::parse(cursor, VuITSConsentRecordGen2::parse)
                .context("Failed to parse vu_its_consent_record_array")?
                .into_inner(),

            vu_power_supply_interruption_record_array: RecordArray::parse(
                cursor,
                VuPowerSupplyInterruptionRecordGen2::parse,
            )
            .context("Failed to parse vu_power_supply_interruption_record_array")?
            .into_inner(),

            signature_record_array: RecordArray::parse_dyn_size(
                cursor,
                SignatureGen2::parse_dyn_size,
            )
            .context("Failed to parse signature_record_array")?
            .into_inner(),
        })
    }
}

type VuDetailedSpeedBlockRecordArray = Vec<VuDetailedSpeedBlock>;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuSpeedBlockGen2 {
    pub vu_detailed_speed_block_record_array: VuDetailedSpeedBlockRecordArray,
    pub signature_record_array: SignatureRecordArrayGen2,
}

impl VuSpeedBlockGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuSpeedBlockGen2 {
            vu_detailed_speed_block_record_array: RecordArray::parse(
                cursor,
                VuDetailedSpeedBlock::parse,
            )
            .context("Failed to parse vu_detailed_speed_block_record_array")?
            .into_inner(),

            signature_record_array: RecordArray::parse_dyn_size(
                cursor,
                SignatureGen2::parse_dyn_size,
            )
            .context("Failed to parse signature_record_array")?
            .into_inner(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuFaultRecordGen2 {
    pub fault_type: EventFaultTypeGen2,
    pub fault_record_purpose: EventFaultRecordPurpose,
    pub fault_begin_time: TimeReal,
    pub fault_end_time: TimeReal,
    pub card_number_and_gen_driver_slot_begin: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot_begin: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_driver_slot_end: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot_end: Option<FullCardNumberAndGenerationGen2>,
    pub manufacturer_specific_event_fault_data: Option<ManufacturerSpecificEventFaultDataGen2>,
}

impl VuFaultRecordGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuFaultRecordGen2 {
            fault_type: EventFaultTypeGen2::parse(cursor).context("Failed to parse fault_type")?,
            fault_record_purpose: EventFaultRecordPurpose::parse(cursor)
                .context("Failed to parse fault_record_purpose")?,
            fault_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse fault_begin_time")?,
            fault_end_time: TimeReal::parse(cursor).context("Failed to parse fault_end_time")?,
            card_number_and_gen_driver_slot_begin: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_begin")
                .ok(),
            card_number_and_gen_codriver_slot_begin: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_begin")
                .ok(),
            card_number_and_gen_driver_slot_end: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_end")
                .ok(),
            card_number_and_gen_codriver_slot_end: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_end")
                .ok(),
            manufacturer_specific_event_fault_data: ManufacturerSpecificEventFaultDataGen2::parse(
                cursor,
            )
            .context("Failed to parse manufacturer_specific_event_fault_data")
            .ok(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuEventRecordGen2 {
    pub event_type: EventFaultTypeGen2,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: Option<TimeReal>,
    pub card_number_and_gen_driver_slot_begin: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot_begin: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_driver_slot_end: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot_end: Option<FullCardNumberAndGenerationGen2>,
    pub similar_events_number: SimilarEventsNumber,
    pub manufacturer_specific_event_fault_data: Option<ManufacturerSpecificEventFaultDataGen2>,
}

impl VuEventRecordGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuEventRecordGen2 {
            event_type: EventFaultTypeGen2::parse(cursor).context("Failed to parse event_type")?,
            event_record_purpose: EventFaultRecordPurpose::parse(cursor)
                .context("Failed to parse event_record_purpose")?,
            event_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse event_begin_time")?,
            event_end_time: TimeReal::parse(cursor).ok(),
            card_number_and_gen_driver_slot_begin: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_begin")
                .ok(),
            card_number_and_gen_codriver_slot_begin: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_begin")
                .ok(),
            card_number_and_gen_driver_slot_end: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_end")
                .ok(),
            card_number_and_gen_codriver_slot_end: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_end")
                .ok(),
            similar_events_number: SimilarEventsNumber::parse(cursor)
                .context("Failed to parse similar_events_number")?,
            manufacturer_specific_event_fault_data: ManufacturerSpecificEventFaultDataGen2::parse(
                cursor,
            )
            .context("Failed to parse manufacturer_specific_event_fault_data")
            .ok(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuOverSpeedingControlDataGen2 {
    pub last_overspeed_control_time: Option<TimeReal>,
    pub first_overspeed_since: Option<TimeReal>,
    pub number_of_overspeed_since: Option<OverspeedNumber>,
}

impl VuOverSpeedingControlDataGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuOverSpeedingControlDataGen2 {
            last_overspeed_control_time: TimeReal::parse(cursor).ok(),
            first_overspeed_since: TimeReal::parse(cursor).ok(),
            number_of_overspeed_since: OverspeedNumber::parse(cursor)
                .context("Failed to parse number_of_overspeed_since")
                .ok(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuOverSpeedingEventRecordGen2 {
    pub event_type: EventFaultTypeGen2,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub max_speed_value: SpeedMax,
    pub average_speed_value: SpeedAverage,
    pub card_number_and_gen_driver_slot_begin: Option<FullCardNumberAndGenerationGen2>,
    pub similar_events_number: SimilarEventsNumber,
}

impl VuOverSpeedingEventRecordGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuOverSpeedingEventRecordGen2 {
            event_type: EventFaultTypeGen2::parse(cursor).context("Failed to parse event_type")?,
            event_record_purpose: EventFaultRecordPurpose::parse(cursor)
                .context("Failed to parse event_record_purpose")?,
            event_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse event_begin_time")?,
            event_end_time: TimeReal::parse(cursor).context("Failed to parse event_end_time")?,
            max_speed_value: SpeedMax::parse(cursor).context("Failed to parse max_speed_value")?,
            average_speed_value: SpeedAverage::parse(cursor)
                .context("Failed to parse average_speed_value")?,
            card_number_and_gen_driver_slot_begin: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_begin")
                .ok(),
            similar_events_number: SimilarEventsNumber::parse(cursor)
                .context("Failed to parse similar_events_number")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
/** [VuTimeAdjustmentRecord: appendix 2.232.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28728) */
pub struct VuTimeAdjustmentRecordGen2 {
    pub old_time_value: TimeReal,
    pub new_time_value: TimeReal,
    pub workshop_name: Name,
    pub workshop_address: Address,
    pub workshop_card_number_and_generation: Option<FullCardNumberAndGenerationGen2>,
}

impl VuTimeAdjustmentRecordGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuTimeAdjustmentRecordGen2 {
            old_time_value: TimeReal::parse(cursor).context("Failed to parse old_time_value")?,
            new_time_value: TimeReal::parse(cursor).context("Failed to parse new_time_value")?,
            workshop_name: Name::parse(cursor).context("Failed to parse workshop_name")?,
            workshop_address: Address::parse(cursor).context("Failed to parse workshop_address")?,
            workshop_card_number_and_generation: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse workshop_card_number_and_generation")
                .ok(),
        })
    }
}

pub type VuFaultRecordArrayGen2 = Vec<VuFaultRecordGen2>;
pub type VuEventRecordArrayGen2 = Vec<VuEventRecordGen2>;
pub type VuOverSpeedingControlDataRecordArrayGen2 = Vec<VuOverSpeedingControlDataGen2>;
pub type VuOverSpeedingEventRecordArrayGen2 = Vec<VuOverSpeedingEventRecordGen2>;
pub type VuTimeAdjustmentRecordArrayGen2 = Vec<VuTimeAdjustmentRecordGen2>;
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuEventsAndFaultsBlockGen2 {
    pub vu_fault_record_array: VuFaultRecordArrayGen2,
    pub vu_event_record_array: VuEventRecordArrayGen2,
    pub vu_over_speeding_control_data_record_array: VuOverSpeedingControlDataRecordArrayGen2,
    pub vu_over_speeding_event_record_array: VuOverSpeedingEventRecordArrayGen2,
    pub vu_time_adjustment_record_array: VuTimeAdjustmentRecordArrayGen2,
    pub signature_record_array: SignatureRecordArrayGen2,
}

impl VuEventsAndFaultsBlockGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuEventsAndFaultsBlockGen2 {
            vu_fault_record_array: RecordArray::parse(cursor, VuFaultRecordGen2::parse)
                .context("Failed to parse vu_fault_record_array")?
                .into_inner(),
            vu_event_record_array: RecordArray::parse(cursor, VuEventRecordGen2::parse)
                .context("Failed to parse vu_event_record_array")?
                .into_inner(),
            vu_over_speeding_control_data_record_array: RecordArray::parse(
                cursor,
                VuOverSpeedingControlDataGen2::parse,
            )
            .context("Failed to parse vu_over_speeding_control_data_record_array")?
            .into_inner(),
            vu_over_speeding_event_record_array: RecordArray::parse(
                cursor,
                VuOverSpeedingEventRecordGen2::parse,
            )
            .context("Failed to parse vu_over_speeding_event_record_array")?
            .into_inner(),
            vu_time_adjustment_record_array: RecordArray::parse(
                cursor,
                VuTimeAdjustmentRecordGen2::parse,
            )
            .context("Failed to parse vu_time_adjustment_record_array")?
            .into_inner(),
            signature_record_array: RecordArray::parse_dyn_size(
                cursor,
                SignatureGen2::parse_dyn_size,
            )
            .context("Failed to parse signature_record_array")?
            .into_inner(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuDownloadActivityDataGen2 {
    pub downloading_time: Option<TimeReal>,
    pub full_card_number_and_generation: Option<FullCardNumberAndGenerationGen2>,
    pub company_or_workshop_name: Option<Name>,
}
impl VuDownloadActivityDataGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuDownloadActivityDataGen2 {
            downloading_time: TimeReal::parse(cursor).ok(),
            full_card_number_and_generation: FullCardNumberAndGenerationGen2::parse(cursor),
            company_or_workshop_name: Name::parse(cursor).ok(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuCompanyLocksGen2 {
    pub lock_in_time: TimeReal,
    pub lock_out_time: Option<TimeReal>,
    pub company_name: Name,
    pub company_address: Address,
    pub company_card_number_and_generation: FullCardNumberAndGenerationGen2,
}
impl VuCompanyLocksGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuCompanyLocksGen2 {
            lock_in_time: TimeReal::parse(cursor).context("Failed to parse lock_in_time")?,
            lock_out_time: TimeReal::parse(cursor)
                .context("Failed to parse lock_out_time")
                .ok(),
            company_name: Name::parse(cursor).context("Failed to parse company_name")?,
            company_address: Address::parse(cursor).context("Failed to parse company_address")?,
            company_card_number_and_generation: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse company_card_number_and_generation")
                .ok()
                .context("FullCardNumberAndGeneration is None")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuControlActivityGen2 {
    pub control_type: ControlTypeGen2,
    pub control_time: TimeReal,
    pub control_card_number_and_generation: FullCardNumberAndGenerationGen2,
    pub download_period_begin_time: TimeReal,
    pub download_period_end_time: TimeReal,
}

impl VuControlActivityGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuControlActivityGen2 {
            control_type: ControlTypeGen2::parse(cursor).context("Failed to parse control_type")?,
            control_time: TimeReal::parse(cursor).context("Failed to parse control_time")?,
            control_card_number_and_generation: FullCardNumberAndGenerationGen2::parse(cursor)
                .context("Failed to parse control_card_number_and_generation")
                .ok()
                .context("FullCardNumberAndGeneration is None")?,
            download_period_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse download_period_begin_time")?,
            download_period_end_time: TimeReal::parse(cursor)
                .context("Failed to parse download_period_end_time")?,
        })
    }
}

pub type MemberStateCertificateRecordArrayGen2 = Vec<MemberStateCertificate>;
pub type VuCertificateRecordArrayGen2 = Vec<CertificateGen2>;
pub type VehicleIdentificationNumberRecordArrayGen2 = Vec<VehicleIdentificationNumber>;
pub type VehicleRegistrationNumberRecordArrayGen2 = Vec<VehicleRegistrationNumber>;
pub type CurrentDateTimeRecordArrayGen2 = Vec<CurrentDateTime>;
pub type VuDownloadablePeriodRecordArrayGen2 = Vec<VuDownloadablePeriod>;
pub type CardSlotsStatusRecordArrayGen2 = Vec<CardSlotsStatus>;
pub type VuDownloadActivityDataRecordArrayGen2 = Vec<VuDownloadActivityDataGen2>;
pub type VuCompanyLocksRecordArrayGen2 = Vec<VuCompanyLocksGen2>;
pub type VuControlActivityRecordArrayGen2 = Vec<VuControlActivityGen2>;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuOverviewBlockGen2 {
    pub member_state_certificate_record_array: MemberStateCertificateRecordArrayGen2,
    pub vu_certificate_record_array: VuCertificateRecordArrayGen2,
    pub vehicle_identification_number_record_array: VehicleIdentificationNumberRecordArrayGen2,
    pub vehicle_registration_number_record_array: VehicleRegistrationNumberRecordArrayGen2,
    pub current_date_time_record_array: CurrentDateTimeRecordArrayGen2,
    pub vu_downloadable_period_record_array: VuDownloadablePeriodRecordArrayGen2,
    pub card_slots_status_record_array: CardSlotsStatusRecordArrayGen2,
    pub vu_download_activity_data_record_array: VuDownloadActivityDataRecordArrayGen2,
    pub vu_company_locks_record_array: VuCompanyLocksRecordArrayGen2,
    pub vu_control_activity_record_array: VuControlActivityRecordArrayGen2,
    pub signature_record_array: SignatureRecordArrayGen2,
}

impl VuOverviewBlockGen2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuOverviewBlockGen2 {
            member_state_certificate_record_array: RecordArray::parse_dyn_size(
                cursor,
                CertificateGen2::parse_dyn_size,
            )
            .context("Failed to parse member_state_certificate_record_array")?
            .into_inner(),
            vu_certificate_record_array: RecordArray::parse_dyn_size(
                cursor,
                CertificateGen2::parse_dyn_size,
            )
            .context("Failed to parse vu_certificate_record_array")?
            .into_inner(),
            vehicle_identification_number_record_array: RecordArray::parse(
                cursor,
                VehicleIdentificationNumber::parse,
            )
            .context("Failed to parse vehicle_identification_number_record_array")?
            .into_inner(),
            vehicle_registration_number_record_array: RecordArray::parse(
                cursor,
                VehicleRegistrationNumber::parse,
            )
            .context("Failed to parse vehicle_registration_number_record_array")?
            .into_inner(),
            current_date_time_record_array: RecordArray::parse(cursor, CurrentDateTime::parse)
                .context("Failed to parse current_date_time_record_array")?
                .into_inner(),
            vu_downloadable_period_record_array: RecordArray::parse(
                cursor,
                VuDownloadablePeriod::parse,
            )
            .context("Failed to parse vu_downloadable_period_record_array")?
            .into_inner(),
            card_slots_status_record_array: RecordArray::parse(cursor, CardSlotsStatus::parse)
                .context("Failed to parse card_slots_status_record_array")?
                .into_inner(),
            vu_download_activity_data_record_array: RecordArray::parse(
                cursor,
                VuDownloadActivityDataGen2::parse,
            )
            .context("Failed to parse vu_download_activity_data_record_array")?
            .into_inner(),
            vu_company_locks_record_array: RecordArray::parse(cursor, VuCompanyLocksGen2::parse)
                .context("Failed to parse vu_company_locks_record_array")?
                .into_inner(),
            vu_control_activity_record_array: RecordArray::parse(
                cursor,
                VuControlActivityGen2::parse,
            )
            .context("Failed to parse vu_control_activity_record_array")?
            .into_inner(),
            signature_record_array: RecordArray::parse_dyn_size(
                cursor,
                SignatureGen2::parse_dyn_size,
            )
            .context("Failed to parse signature_record_array")?
            .into_inner(),
        })
    }
}
