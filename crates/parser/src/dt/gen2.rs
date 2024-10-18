#![allow(dead_code)]
use super::*;
use crate::bytes::{extract_u8_bits_into_tup, TakeExact};
use anyhow::{Context, Result};
use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::{any::type_name, io::Read};

#[derive(Debug, Serialize, Deserialize)]
/// [RecordType: appendix 2.120.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23342)
pub enum RecordType {
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

impl RecordType {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let record_type = cursor.read_u8().context("Failed to read record type")?;
        match record_type {
            0x00 => anyhow::bail!(
                "Detected record_type 0x00, this is not a valid record_type according to the spec"
            ),
            0x01 => Ok(RecordType::ActivityChangeInfo),
            0x02 => Ok(RecordType::CardSlotsStatus),
            0x03 => Ok(RecordType::CurrentDateTime),
            0x04 => Ok(RecordType::MemberStateCertificate),
            0x05 => Ok(RecordType::OdometerValueMidnight),
            0x06 => Ok(RecordType::DateOfDayDownloaded),
            0x07 => Ok(RecordType::SensorPaired),
            0x08 => Ok(RecordType::Signature),
            0x09 => Ok(RecordType::SpecificConditionRecord),
            0x0A => Ok(RecordType::VehicleIdentificationNumber),
            0x0B => Ok(RecordType::VehicleRegistrationNumber),
            0x0C => Ok(RecordType::VuCalibrationRecord),
            0x0D => Ok(RecordType::VuCardIWRecord),
            0x0E => Ok(RecordType::VuCardRecord),
            0x0F => Ok(RecordType::VuCertificate),
            0x10 => Ok(RecordType::VuCompanyLocksRecord),
            0x11 => Ok(RecordType::VuControlActivityRecord),
            0x12 => Ok(RecordType::VuDetailedSpeedBlock),
            0x13 => Ok(RecordType::VuDownloadablePeriod),
            0x14 => Ok(RecordType::VuDownloadActivityData),
            0x15 => Ok(RecordType::VuEventRecord),
            0x16 => Ok(RecordType::VuGNSSADRecord),
            0x17 => Ok(RecordType::VuITSConsentRecord),
            0x18 => Ok(RecordType::VuFaultRecord),
            0x19 => Ok(RecordType::VuIdentification),
            0x1A => Ok(RecordType::VuOverSpeedingControlData),
            0x1B => Ok(RecordType::VuOverSpeedingEventRecord),
            0x1C => Ok(RecordType::VuPlaceDailyWorkPeriodRecord),
            0x1D => Ok(RecordType::VuTimeAdjustmentGNSSRecord),
            0x1E => Ok(RecordType::VuTimeAdjustmentRecord),
            0x1F => Ok(RecordType::VuPowerSupplyInterruptionRecord),
            0x20 => Ok(RecordType::SensorPairedRecord),
            0x21 => Ok(RecordType::SensorExternalGNSSCoupledRecord),
            0x22..=0x7F => Ok(RecordType::RFU),
            0x80..=0xFF => Ok(RecordType::ManufacturerSpecific),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]

/// A generic implementation for an array of records, where the record type is parameterized
/// This helper is used across various Vu blocks to parse and store their respective records
pub struct RecordArray<T> {
    record_type: RecordType,
    record_size: u16,
    no_of_records: u16,
    pub records: Vec<T>,
}

impl<T> RecordArray<T> {
    pub fn parse<F>(cursor: &mut Cursor<&[u8]>, parse_record: F) -> Result<Self>
    where
        F: Fn(&mut Cursor<&[u8]>) -> Result<T>,
    {
        let record_type = RecordType::parse(cursor).context("Failed to parse record type")?;
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
        let record_type = RecordType::parse(cursor).context("Failed to parse record type")?;
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
/// [Certificate: appendix 2.41.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18396)
pub struct Certificate(Vec<u8>);
impl Certificate {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let mut value = vec![0u8; size];
        cursor
            .read_exact(&mut value)
            .context("Failed to read value")?;
        Ok(Certificate(value))
    }
}

/// [MemberStateCertificate: appendix 2.96.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22309)
pub type MemberStateCertificate = Certificate;

/// [VuCertificate: appendix 2.181.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26086)
pub type VuCertificate = Certificate;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// [EquipmentType: appendix 2.67.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e20100)
pub enum EquipmentType {
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
impl EquipmentType {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let equipment_type = cursor.read_u8().context("Failed to read equipment type")?;
        match equipment_type {
            0 => Ok(EquipmentType::Reserved),
            1 => Ok(EquipmentType::DriverCard),
            2 => Ok(EquipmentType::WorkshopCard),
            3 => Ok(EquipmentType::ControlCard),
            4 => Ok(EquipmentType::CompanyCard),
            5 => Ok(EquipmentType::ManufacturingCard),
            6 => Ok(EquipmentType::VehicleUnit),
            7 => Ok(EquipmentType::MotionSensor),
            8 => Ok(EquipmentType::GNSSFacility),
            9 => Ok(EquipmentType::RemoteCommunicationDevice),
            10 => Ok(EquipmentType::ITSinterfaceModule),
            11 => Ok(EquipmentType::Plaque),
            12 => Ok(EquipmentType::M1N1Adapter),
            13 => Ok(EquipmentType::CAERCA),
            14 => Ok(EquipmentType::CAMSCA),
            15 => Ok(EquipmentType::ExternalGNSSConnection),
            16 => Ok(EquipmentType::Unused),
            17 => Ok(EquipmentType::DriverCardSign),
            18 => Ok(EquipmentType::WorkshopCardSign),
            19 => Ok(EquipmentType::VehicleUnitSign),
            20..=255 => Ok(EquipmentType::RFU),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [FullCardNumber: appendix 2.73.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21400)
pub struct FullCardNumber {
    pub card_type: EquipmentType,
    pub card_issuing_member_state: external::NationNumeric,
    pub card_number: CardNumber,
}
impl FullCardNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_type = EquipmentType::parse(cursor)?;
        let card_issuing_member_state = external::NationNumeric::parse(cursor)?;

        let card_number = match card_type {
            EquipmentType::DriverCard => CardNumber::parse_driver(cursor)?,
            EquipmentType::WorkshopCard
            | EquipmentType::ControlCard
            | EquipmentType::CompanyCard => CardNumber::parse_owner(cursor)?,
            _ => CardNumber::parse_unknown(cursor)?,
        };

        Ok(FullCardNumber {
            card_type,
            card_issuing_member_state,
            card_number,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// [Generation: appendix 2.75.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23342)
pub enum Generation {
    Generation1,
    Generation2,
    RFU,
}

impl Generation {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let generation = cursor.read_u8().context("Failed to read generation")?;

        match generation {
            0x00 => Ok(Generation::RFU),
            0x01 => Ok(Generation::Generation1),
            0x02 => Ok(Generation::Generation2),
            0x03..=0xFF => Ok(Generation::RFU),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [FullCardNumberAndGeneration: appendix 2.74.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21438)
pub struct FullCardNumberAndGeneration {
    pub full_card_number: FullCardNumber,
    pub generation: Generation,
}
impl FullCardNumberAndGeneration {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Option<Self> {
        let full_card_number = match FullCardNumber::parse(cursor) {
            Ok(number) => number,
            Err(_) => return None,
        };
        let generation = match Generation::parse(cursor) {
            Ok(gen) => gen,
            Err(_) => return None,
        };
        let value = match generation {
            Generation::RFU => None,
            _ => Some(FullCardNumberAndGeneration {
                full_card_number,
                generation,
            }),
        };
        value
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [ControlType: appendix 2.53.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e19148)
pub struct ControlType {
    pub card_downloading: bool,
    pub vu_downloading: bool,
    pub printing: bool,
    pub display: bool,
    pub roadside_calibration_checking: bool,
}
impl ControlType {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let control_type_byte = cursor.read_u8().context("Failed to read control type")?;

        let bits = extract_u8_bits_into_tup(control_type_byte);

        Ok(ControlType {
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
/// [Signature: appendix 2.149.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24501)
pub struct Signature(Vec<u8>); // Octet string
impl Signature {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        if size < 64 || size > 132 {
            anyhow::bail!("expected signature size to be 64..132 bytes, got {}", size);
        }
        let mut signature_buffer = vec![0u8; size];
        cursor
            .read_exact(&mut signature_buffer)
            .context("Failed to read signature buffer")?;
        Ok(Signature(signature_buffer))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [PreviousVehicleInfo: appendix 2.118.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23250)
pub struct PreviousVehicleInfo {
    vehicle_registration_identification: VehicleRegistrationIdentification,
    card_withdrawal_time: TimeReal,
    vu_generation: Generation,
}
impl PreviousVehicleInfo {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let vehicle_registration_identification = VehicleRegistrationIdentification::parse(cursor)?;
        let card_withdrawal_time = TimeReal::parse(cursor)?;
        let vu_generation = Generation::parse(cursor)?;
        Ok(PreviousVehicleInfo {
            vehicle_registration_identification,
            card_withdrawal_time,
            vu_generation,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [GNSSPlaceRecord: appendix 2.80.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21772)
pub struct GNSSPlaceRecord {
    time_stamp: TimeReal,
    gnss_accuracy: GNSSAccuracy,
    geo_coordinates: GeoCoordinates,
}
impl GNSSPlaceRecord {
    const SIZE: usize = 7;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_stamp = TimeReal::parse(cursor)?;
        let gnss_accuracy = GNSSAccuracy::parse(cursor)?;
        let geo_coordinates = GeoCoordinates::parse(cursor)?;

        Ok(GNSSPlaceRecord {
            time_stamp,
            gnss_accuracy,
            geo_coordinates,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [GNSSAccuracy: appendix 2.77.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21573)
pub struct GNSSAccuracy(u8);
impl GNSSAccuracy {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor.read_u8().context("Failed to read GNSSAccuracy")?;
        if value > 100 {
            anyhow::bail!("Invalid GNSSAccuracy");
        }
        Ok(GNSSAccuracy(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [GeoCoordinates: appendix 2.76.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21534)
pub struct GeoCoordinates {
    latitude: f64,
    longitude: f64,
}
impl GeoCoordinates {
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

        Ok(GeoCoordinates {
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
/// [VuGNSSADRecord: appendix 2.203.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27345)
pub struct VuGNSSADRecord {
    time_stamp: TimeReal,
    card_number_and_gen_driver_slot: Option<FullCardNumberAndGeneration>,
    card_number_and_gen_codriver_slot: Option<FullCardNumberAndGeneration>,
    gnss_place_record: GNSSPlaceRecord,
    vehicle_odometer_value: OdometerShort,
}
impl VuGNSSADRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_stamp = TimeReal::parse(cursor)?;
        let card_number_and_gen_driver_slot = FullCardNumberAndGeneration::parse(cursor)
            .context("Failed to parse card_number_and_gen_driver_slot")
            .ok();
        let card_number_and_gen_codriver_slot = FullCardNumberAndGeneration::parse(cursor)
            .context("Failed to parse card_number_and_gen_codriver_slot")
            .ok();
        let gnss_place_record = GNSSPlaceRecord::parse(cursor)?;
        let vehicle_odometer_value = OdometerShort::parse(cursor)?;

        Ok(VuGNSSADRecord {
            time_stamp,
            card_number_and_gen_driver_slot,
            card_number_and_gen_codriver_slot,
            gnss_place_record,
            vehicle_odometer_value,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
/// [EntryTypeDailyWorkPeriod: appendix 2.66.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e20044)
pub enum EntryTypeDailyWorkPeriod {
    BeginRelatedTimeCardInsertionTimeOrTimeOfEntry,
    EndRelatedTimeCardWithdrawalTimeOrTimeOfEntry,
    BeginRelatedTimeManuallyEntered,
    EndRelatedTimeManuallyEntered,
}

impl EntryTypeDailyWorkPeriod {
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
/// [PlaceRecord: appendix 2.117.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23112)
pub struct PlaceRecord {
    pub entry_time: TimeReal,
    pub entry_type_daily_work_period: EntryTypeDailyWorkPeriod,
    pub daily_work_period_country: external::NationNumeric,
    pub daily_work_period_region: RegionNumeric,
    pub vehicle_odometer_value: OdometerShort,
    pub entry_gnss_place_record: GNSSPlaceRecord,
}
impl PlaceRecord {
    const SIZE: usize = 21;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        let entry_time = TimeReal::parse(inner_cursor)?;
        let entry_type_daily_work_period = EntryTypeDailyWorkPeriod::parse(inner_cursor)?;
        let daily_work_period_country = external::NationNumeric::parse(inner_cursor)?;
        let daily_work_period_region = RegionNumeric::parse(inner_cursor)?;
        let vehicle_odometer_value = OdometerShort::parse(inner_cursor)?;
        let entry_gnss_place_record = GNSSPlaceRecord::parse(inner_cursor)?;
        if entry_time.0.timestamp() == 0 {
            anyhow::bail!("Invalid entry_time in PlaceRecord");
        }
        Ok(PlaceRecord {
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
/// [SpecificConditionType: appendix 2.154.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24685)
pub enum SpecificConditionType {
    RFU,
    OutOfScopeBegin,
    OutOfScopeEnd,
    FerryTrainCrossingBegin,
    FerryTrainCrossingEnd,
}

impl SpecificConditionType {
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
/// [SpecificConditionRecord: appendix 2.152.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24614)
pub struct SpecificConditionRecord {
    entry_time: TimeReal,
    specific_condition_type: SpecificConditionType,
}
impl SpecificConditionRecord {
    const SIZE: usize = 5;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        let entry_time = TimeReal::parse(inner_cursor)?;
        let specific_condition_type = SpecificConditionType::parse(inner_cursor)?;
        Ok(SpecificConditionRecord {
            entry_time,
            specific_condition_type,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// [EventFaultType: appendix 2.70.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e20338)
pub enum EventFaultType {
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

impl EventFaultType {
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
/// [ManufacturerSpecificEventFaultData: appendix 2.95.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22276)
pub struct ManufacturerSpecificEventFaultData {
    pub manufacturer_code: external::ManufacturerCode,
    pub manufacturer_specific_error_code: [u8; 3],
}
impl ManufacturerSpecificEventFaultData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let manufacturer_code = external::ManufacturerCode::parse(cursor).ok();

        let mut manufacturer_specific_error_code = [0u8; 3];
        cursor
            .read_exact(&mut manufacturer_specific_error_code)
            .context("Failed to read manufacturer specific error code")?;

        if manufacturer_code.is_none() {
            anyhow::bail!("Manufacturer code is not present in ManufacturerSpecificEventFaultData");
        }

        Ok(ManufacturerSpecificEventFaultData {
            manufacturer_code: manufacturer_code.unwrap(),
            manufacturer_specific_error_code,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [ExtendedSerialNumber: appendix 2.72.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21307)
pub struct ExtendedSerialNumber {
    pub serial_number: u32,
    pub month_year: MonthYear,
    pub equipment_type: EquipmentType,
    pub manufacturer_code: external::ManufacturerCode,
}
impl ExtendedSerialNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let serial_number = cursor
            .read_u32::<BigEndian>()
            .context("Failed to read serial number")?;

        let month_year = MonthYear::parse(cursor)?;
        let equipment_type = EquipmentType::parse(cursor)?;
        let manufacturer_code = external::ManufacturerCode::parse(cursor)?;

        Ok(ExtendedSerialNumber {
            serial_number,
            month_year,
            equipment_type,
            manufacturer_code,
        })
    }
}

/// [VuSerialNumber: appendix 2.223.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28497)
pub type VuSerialNumber = ExtendedSerialNumber;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuApprovalNumber: appendix 2.172.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25427)
pub struct VuApprovalNumber(IA5String);
impl VuApprovalNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuApprovalNumber(IA5String::parse_dyn_size(cursor, 16)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// [VuAbility: appendix 2.169.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25277)
pub enum VuAbility {
    SupportsGen1,
    SupportsGen2,
    RFU,
}

impl VuAbility {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor.read_u8().context("Failed to read VuAbility")?;

        match extract_u8_bits_into_tup(value) {
            // TODO: check if the order is correct
            (_, _, _, _, _, _, _, 0) => Ok(VuAbility::SupportsGen1),
            (_, _, _, _, _, _, _, 1) => Ok(VuAbility::SupportsGen2),
            _ => Ok(VuAbility::RFU),
        }
    }
}
/// [SensorSerialNumber: appendix 2.148.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24483)
pub type SensorSerialNumber = ExtendedSerialNumber;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [SensorApprovalNumber: appendix 2.131.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23887)
pub struct SensorApprovalNumber(IA5String);
impl SensorApprovalNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = IA5String::parse_dyn_size(cursor, 16)?;
        Ok(SensorApprovalNumber(value))
    }
}

/// [SensorGNSSSerialNumber: appendix 2.139.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24175)
pub type SensorGNSSSerialNumber = ExtendedSerialNumber;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [SensorExternalGNSSApprovalNumber: appendix 2.132.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23931)
pub struct SensorExternalGNSSApprovalNumber(IA5String);
impl SensorExternalGNSSApprovalNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = IA5String::parse_dyn_size(cursor, 16)?;
        Ok(SensorExternalGNSSApprovalNumber(value))
    }
}

pub type SensorGNSSCouplingDate = TimeReal;
#[derive(Debug, Serialize, Deserialize)]
/// [CalibrationPurpose: appendix 2.8.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16597)
pub enum CalibrationPurpose {
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

impl CalibrationPurpose {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to read CalibrationPurpose")?;
        let purpose = match value {
            0x00 => CalibrationPurpose::Reserved,
            0x01 => CalibrationPurpose::Activation,
            0x02 => CalibrationPurpose::FirstInstallation,
            0x03 => CalibrationPurpose::Installation,
            0x04 => CalibrationPurpose::PeriodicInspection,
            0x05 => CalibrationPurpose::EntryOfVRNByCompany,
            0x06 => CalibrationPurpose::TimeAdjustmentWithoutCalibration,
            0x07..=0x7F => CalibrationPurpose::RFU,
            0x80..=0xFF => CalibrationPurpose::ManufacturerSpecific,
        };
        Ok(purpose)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [ExtendedSealIdentifier: appendix 2.71.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21276)
pub struct ExtendedSealIdentifier {
    pub manufacturer_code: [u8; 2],
    pub seal_identifier: [u8; 8],
}
impl ExtendedSealIdentifier {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut manufacturer_code = [0u8; 2];
        cursor
            .read_exact(&mut manufacturer_code)
            .context("Failed to read manufacturer code")?;

        let mut seal_identifier = [0u8; 8];
        cursor
            .read_exact(&mut seal_identifier)
            .context("Failed to read seal identifier")?;

        Ok(ExtendedSealIdentifier {
            manufacturer_code,
            seal_identifier,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [SealRecord: appendix 2.130.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23854)
pub struct SealRecord {
    pub equipment_type: EquipmentType,
    pub extended_seal_identifier: ExtendedSealIdentifier,
}

impl SealRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(SealRecord {
            equipment_type: EquipmentType::parse(cursor)?,
            extended_seal_identifier: ExtendedSealIdentifier::parse(cursor)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [SealDataVu: appendix 2.129.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23827)
pub struct SealDataVu {
    seal_records: Vec<SealRecord>,
}

impl SealDataVu {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut seal_records = Vec::new();
        for _ in 0..5 {
            let seal = SealRecord::parse(cursor)?;
            // if equipment type is not unused, then it is a valid seal, see page 50
            if seal.equipment_type != EquipmentType::Unused {
                seal_records.push(seal);
            }
        }
        Ok(SealDataVu { seal_records })
    }
}

/// [NoOfEventsPerType: appendix 2.109.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22706)
pub type NoOfEventsPerType = u8;
/// [NoOfFaultsPerType: appendix 2.110.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22729)
pub type NoOfFaultsPerType = u8;
/// [NoOfCardVehicleRecords: appendix 2.105.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22612)
pub type NoOfCardVehicleRecords = u16;
/// [NoOfCardPlaceRecords: appendix 2.104.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22566)
pub type NoOfCardPlaceRecords = u16;
/// [NoOfGnssAdRecords: appendix 2.111.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22756)
pub type NoOfGnssAdRecords = u16;
/// [NoOfSpecificConditionRecords: appendix 2.112.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22807)
pub type NoOfSpecificConditionRecords = u16;
/// [NoOfCardVehicleUnitRecords: appendix 2.106.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22635)
pub type NoOfCardVehicleUnitRecords = u16;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [DriverCardApplicationIdentification: appendix 2.61.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e19751)

pub struct DriverCardApplicationIdentification {
    pub type_of_tachograph_card_id: EquipmentType,
    pub card_structure_version: CardStructureVersion,
    pub no_of_events_per_type: NoOfEventsPerType,
    pub no_of_faults_per_type: NoOfFaultsPerType,
    pub activity_structure_length: CardActivityLengthRange,
    pub no_of_card_vehicle_records: NoOfCardVehicleRecords,
    pub no_of_card_place_records: NoOfCardPlaceRecords,
    pub no_of_gnss_ad_records: NoOfGnssAdRecords,
    pub no_of_specific_condition_records: NoOfSpecificConditionRecords,
    pub no_of_card_vehicle_unit_records: NoOfCardVehicleUnitRecords,
}

impl DriverCardApplicationIdentification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let type_of_tachograph_card_id = EquipmentType::parse(cursor)?;

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

        Ok(DriverCardApplicationIdentification {
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
#[serde(rename_all(serialize = "camelCase"))]
pub struct ApplicationIdentification {
    pub driver_card_application_identification: DriverCardApplicationIdentification,
}
impl ApplicationIdentification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let driver_card_application_identification =
            DriverCardApplicationIdentification::parse(cursor)?;
        Ok(ApplicationIdentification {
            driver_card_application_identification,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardIccIdentification: appendix 2.23.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17372)
pub struct CardIccIdentification {
    pub clock_stop: u8,
    pub card_extended_serial_number: ExtendedSerialNumber,
    pub card_approval_number: CardApprovalNumber,
    pub card_personaliser_id: external::ManufacturerCode,
    pub embedder_ic_assembler_id: EmbedderIcAssemblerId,
    pub ic_identifier: [u8; 2],
}
impl CardIccIdentification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let clock_stop = cursor.read_u8().context("Failed to read clock_stop")?;
        let card_extended_serial_number = ExtendedSerialNumber::parse(cursor)?;
        let card_approval_number = CardApprovalNumber::parse(cursor)?;
        let card_personaliser_id = external::ManufacturerCode::parse(cursor)?;
        let embedder_ic_assembler_id = EmbedderIcAssemblerId::parse(cursor)?;
        let mut buffer = [0u8; 2];

        cursor
            .read_exact(&mut buffer)
            .context("Failed to read ic_identifier")?;
        let ic_identifier = [buffer[0], buffer[1]];

        Ok(CardIccIdentification {
            clock_stop,
            card_extended_serial_number,
            card_approval_number,
            card_personaliser_id,
            embedder_ic_assembler_id,
            ic_identifier,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardEventRecord: appendix 2.20.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17247)
pub struct CardEventRecord {
    pub event_type: EventFaultType,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub event_vehicle_registration: VehicleRegistrationIdentification,
}

impl CardEventRecord {
    const SIZE: usize = 24;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        let event_type = EventFaultType::parse(inner_cursor)?;
        let event_begin_time = TimeReal::parse(inner_cursor)?;
        let event_end_time = TimeReal::parse(inner_cursor)?;
        let event_vehicle_registration = VehicleRegistrationIdentification::parse(inner_cursor)?;

        Ok(CardEventRecord {
            event_type,
            event_begin_time,
            event_end_time,
            event_vehicle_registration,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardEventData: appendix 2.19.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17180)
pub struct CardEventData(Vec<Vec<CardEventRecord>>);
impl CardEventData {
    const OUTER_RECORDS_AMOUNT: usize = 11;
    const INNER_RECORDS_AMOUNT: usize = 1;

    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let mut card_event_records = Vec::new();
        let inner_record_amounts = size / Self::OUTER_RECORDS_AMOUNT / CardEventRecord::SIZE;

        for _ in 0..Self::OUTER_RECORDS_AMOUNT {
            let mut inner_card_event_records = Vec::new();
            for _ in 0..inner_record_amounts {
                if let Ok(card_event_record) = CardEventRecord::parse(cursor) {
                    inner_card_event_records.push(card_event_record);
                }
            }
            // Only include the records if there are any
            if inner_card_event_records.len() > 0 {
                card_event_records.push(inner_card_event_records);
            }
        }
        Ok(CardEventData(card_event_records))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardFaultData: appendix 2.21.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17292)
pub struct CardFaultRecord {
    pub fault_type: EventFaultType,
    pub fault_begin_time: TimeReal,
    pub fault_end_time: TimeReal,
    pub fault_vehicle_registration: VehicleRegistrationIdentification,
}

impl CardFaultRecord {
    pub const SIZE: usize = 24;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        let fault_type = EventFaultType::parse(inner_cursor)?;
        let fault_begin_time = TimeReal::parse(inner_cursor)?;
        let fault_end_time = TimeReal::parse(inner_cursor)?;
        let fault_vehicle_registration = VehicleRegistrationIdentification::parse(inner_cursor)?;

        Ok(CardFaultRecord {
            fault_type,
            fault_begin_time,
            fault_end_time,
            fault_vehicle_registration,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardFaultData: appendix 2.22.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17340)
pub struct CardFaultData(Vec<Vec<CardFaultRecord>>);
impl CardFaultData {
    const MAX_BLOCK_SIZE: usize = 1152;
    const OUTER_RECORDS_AMOUNT: usize = 6;

    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut card_fault_records = Vec::new();

        let max_possible_records = Self::MAX_BLOCK_SIZE / CardFaultRecord::SIZE;
        let max_inner_records = max_possible_records / Self::OUTER_RECORDS_AMOUNT;

        // According to the spec, there are ALWAYS 2 outer CardFaultRecords, but we'll use the computed size just in case
        for _ in 0..Self::OUTER_RECORDS_AMOUNT {
            let mut inner_card_fault_records = Vec::new();
            for _ in 0..max_inner_records {
                match CardFaultRecord::parse(cursor) {
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
        Ok(CardFaultData(card_fault_records))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardVehicleRecord: appendix 2.37.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18163)
pub struct CardVehicleRecord {
    pub vehicle_odometer_begin: OdometerShort,
    pub vehicle_odometer_end: OdometerShort,
    pub vehicle_first_use: TimeReal,
    pub vehicle_last_use: TimeReal,
    pub vehicle_registration: VehicleRegistrationIdentification,
    pub vu_data_block_counter: VuDataBlockCounter,
    pub vehicle_identification_number: VehicleIdentificationNumber,
}
impl CardVehicleRecord {
    const SIZE: usize = 48;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        Ok(CardVehicleRecord {
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
/// [CardVehiclesUsed: appendix 2.38.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18302)
pub struct CardVehiclesUsed {
    vehicle_pointer_newest_record: u16,
    card_vehicle_records: Vec<CardVehicleRecord>,
}
impl CardVehiclesUsed {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let cursor = &mut cursor.take_exact(size);
        let vehicle_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read vehicle_pointer_newest_record")?;
        let mut card_vehicle_records = Vec::new();
        let amount_of_records = size as usize / CardVehicleRecord::SIZE as usize;
        for i in 0..amount_of_records {
            if let Ok(card_vehicle_record) = CardVehicleRecord::parse(cursor) {
                card_vehicle_records.push(card_vehicle_record);
            }
            // If we've reached the newest record, break
            if i + 1 == vehicle_pointer_newest_record as usize {
                break;
            }
        }

        Ok(CardVehiclesUsed {
            vehicle_pointer_newest_record,
            card_vehicle_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardPlaceDailyWorkPeriod: appendix 2.27.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17729)
pub struct CardPlaceDailyWorkPeriod {
    place_pointer_newest_record: NoOfCardPlaceRecords,
    place_records: Vec<PlaceRecord>,
}
impl CardPlaceDailyWorkPeriod {
    pub fn parse(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let place_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read place_pointer_newest_record")?;

        let mut place_records = Vec::new();
        let amount_of_records = size as usize / PlaceRecord::SIZE as usize;

        for _ in 0..amount_of_records {
            if let Ok(place_record) = PlaceRecord::parse(cursor) {
                place_records.push(place_record);
            }
        }
        // Sort the records by entry_time in ascending order
        place_records.sort_by(|a, b| a.entry_time.0.timestamp().cmp(&b.entry_time.0.timestamp()));
        Ok(CardPlaceDailyWorkPeriod {
            place_pointer_newest_record,
            place_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardControlActivityDataRecord appendix 2.15.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17002)
pub struct CardControlActivityDataRecord {
    pub control_type: ControlType,
    pub control_time: TimeReal,
    pub control_card_number: FullCardNumber,
    pub control_vehicle_registration: VehicleRegistrationIdentification,
    pub control_download_period_begin: TimeReal,
    pub control_download_period_end: TimeReal,
}
impl CardControlActivityDataRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            control_type: ControlType::parse(cursor)?,
            control_time: TimeReal::parse(cursor)?,
            control_card_number: FullCardNumber::parse(cursor)?,
            control_vehicle_registration: VehicleRegistrationIdentification::parse(cursor)?,
            control_download_period_begin: TimeReal::parse(cursor)?,
            control_download_period_end: TimeReal::parse(cursor)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [SpecificConditions: appendix 2.153.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24644)
pub struct SpecificConditions {
    pub condition_pointer_newest_record: NoOfSpecificConditionRecords,
    pub specific_condition_records: Vec<SpecificConditionRecord>,
}
impl SpecificConditions {
    pub fn parse(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let condition_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read condition_pointer_newest_record")?;

        let mut specific_condition_records = Vec::new();
        let no_of_records = size / SpecificConditionRecord::SIZE;
        for _ in 0..no_of_records {
            if let Ok(specific_condition_record) = SpecificConditionRecord::parse(cursor) {
                specific_condition_records.push(specific_condition_record);
            }
        }
        // Sort the records by time_stamp in desc order
        specific_condition_records
            .sort_by(|a, b| b.entry_time.0.timestamp().cmp(&a.entry_time.0.timestamp()));
        Ok(SpecificConditions {
            condition_pointer_newest_record,
            specific_condition_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardVehicleUnitRecord: appendix 2.39.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18302)
pub struct CardVehicleUnitRecord {
    pub time_stamp: TimeReal,
    pub manufacturer_code: external::ManufacturerCode,
    pub device_id: u8,
    pub vu_software_version: VuSoftwareVersion,
}
impl CardVehicleUnitRecord {
    const SIZE: usize = 10;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

        let time_stamp = TimeReal::parse(inner_cursor)?;
        let manufacturer_code = external::ManufacturerCode::parse(inner_cursor)?;
        let device_id = inner_cursor.read_u8().context("Failed to read device_id")?;
        let vu_software_version = VuSoftwareVersion::parse(inner_cursor)?;

        if time_stamp.0.timestamp() == 0 {
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
/// [CardVehicleUnitsUsed: appendix 2.40.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18350)
pub struct CardVehicleUnitsUsed {
    pub no_of_card_vehicle_unit_records: NoOfCardVehicleUnitRecords,
    pub card_vehicle_unit_records: Vec<CardVehicleUnitRecord>,
}
impl CardVehicleUnitsUsed {
    pub fn parse(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let no_of_card_vehicle_unit_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read no_of_card_vehicle_unit_records")?;
        let mut vehicle_units = Vec::new();

        let no_of_records = size / CardVehicleUnitRecord::SIZE;
        for _ in 0..no_of_records {
            if let Ok(vehicle_unit) = CardVehicleUnitRecord::parse(cursor) {
                vehicle_units.push(vehicle_unit);
            }
        }
        // Sort the records by time_stamp in desc order
        vehicle_units.sort_by(|a, b| b.time_stamp.0.timestamp().cmp(&a.time_stamp.0.timestamp()));
        Ok(CardVehicleUnitsUsed {
            no_of_card_vehicle_unit_records,
            card_vehicle_unit_records: vehicle_units,
        })
    }
}

/// [NoOfGNSSADRecords: appendix 2.111.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22756)
pub type NoOfGNSSADRecords = u16;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [GNSSAccumulatedDrivingRecord: appendix 2.79.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21640)
pub struct GNSSAccumulatedDrivingRecord {
    pub time_stamp: TimeReal,
    pub gnss_place_record: GNSSPlaceRecord,
    pub vehicle_odometer_value: OdometerShort,
}
impl GNSSAccumulatedDrivingRecord {
    pub const SIZE: usize = 18;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_stamp = TimeReal::parse(cursor)?;
        let gnss_place_record = GNSSPlaceRecord::parse(cursor)?;
        let vehicle_odometer_value = OdometerShort::parse(cursor)?;

        if time_stamp.0.timestamp() == 0 {
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
/// [GNSSAccumulatedDriving: appendix 2.79.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21595)
pub struct GNSSAccumulatedDriving {
    pub gnss_ad_pointer_newest_record: NoOfGNSSADRecords,
    pub gnss_accumulated_driving_records: Vec<GNSSAccumulatedDrivingRecord>,
}
impl GNSSAccumulatedDriving {
    pub fn parse(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let gnss_ad_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read gnss_ad_pointer_newest_record")?;

        let mut gnss_accumulated_driving_records = Vec::new();
        let no_of_records = size as usize / GNSSAccumulatedDrivingRecord::SIZE as usize;
        for _ in 0..no_of_records {
            if let Ok(gnss_accumulated_driving_record) = GNSSAccumulatedDrivingRecord::parse(cursor)
            {
                gnss_accumulated_driving_records.push(gnss_accumulated_driving_record);
            }
        }
        // Sort the records by time_stamp in ascending order
        gnss_accumulated_driving_records
            .sort_by(|a, b| a.time_stamp.0.timestamp().cmp(&b.time_stamp.0.timestamp()));
        Ok(GNSSAccumulatedDriving {
            gnss_ad_pointer_newest_record,
            gnss_accumulated_driving_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct DateOfDayDownloaded(TimeReal);

impl DateOfDayDownloaded {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_real =
            TimeReal::parse(cursor).context("Failed to parse TimeReal for DateOfDayDownloaded")?;
        Ok(DateOfDayDownloaded(time_real))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuCardIWRecord {
    pub card_holder_name: HolderName,
    pub full_card_number_and_generation: FullCardNumberAndGeneration,
    pub card_expiry_date: TimeReal,
    pub card_insertion_date: TimeReal,
    pub vehicle_odometer_value_at_insertion: OdometerShort,
    pub card_slot_number: CardSlotNumber,
    pub card_withdrawl_time: TimeReal,
    pub vehicle_odometer_value_at_withdrawal: OdometerShort,
    pub previous_vehicle_info: PreviousVehicleInfo,
    pub manual_input_flag: ManualInputFlag,
}

impl VuCardIWRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuCardIWRecord {
            card_holder_name: HolderName::parse(cursor)
                .context("Failed to parse card_holder_name")?,
            full_card_number_and_generation: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse full_card_number_and_generation")?,
            card_expiry_date: TimeReal::parse(cursor)
                .context("Failed to parse card_expiry_date")?,
            card_insertion_date: TimeReal::parse(cursor)
                .context("Failed to parse card_insertion_date")?,
            vehicle_odometer_value_at_insertion: OdometerShort::parse(cursor)
                .context("Failed to parse vehicle_odometer_value_at_insertion")?,
            card_slot_number: CardSlotNumber::parse(cursor)
                .context("Failed to parse card_slot_number")?,
            card_withdrawl_time: TimeReal::parse(cursor)
                .context("Failed to parse card_withdrawl_time")?,
            vehicle_odometer_value_at_withdrawal: OdometerShort::parse(cursor)
                .context("Failed to parse vehicle_odometer_value_at_withdrawal")?,
            previous_vehicle_info: PreviousVehicleInfo::parse(cursor)
                .context("Failed to parse previous_vehicle_info")?,
            manual_input_flag: ManualInputFlag::parse(cursor)
                .context("Failed to parse manual_input_flag")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuPlaceDailyWorkPeriod {
    pub full_card_number_and_generation: FullCardNumberAndGeneration,
    pub place_record: PlaceRecord,
}

impl VuPlaceDailyWorkPeriod {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuPlaceDailyWorkPeriod {
            full_card_number_and_generation: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse full_card_number_and_generation")?,
            place_record: PlaceRecord::parse(cursor).context("Failed to parse place_record")?,
        })
    }
}

pub type DateOfDayDownloadedRecordArray = Vec<DateOfDayDownloaded>;
pub type OdometerValueMidnightRecordArray = Vec<OdometerValueMidnight>;
pub type VuCardIWRecordRecordArray = Vec<VuCardIWRecord>;
pub type VuActivityDailyRecordArray = Vec<ActivityChangeInfo>;
pub type VuPlaceDailyWorkPeriodRecordArray = Vec<VuPlaceDailyWorkPeriod>;
pub type VuGNSSADRecordArray = Vec<VuGNSSADRecord>;
pub type VuSpecificConditionRecordArray = Vec<SpecificConditionRecord>;
pub type SignatureRecordArray = Vec<Signature>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuActivitiesBlock {
    date_of_day_downloaded_record_array: DateOfDayDownloadedRecordArray,
    odometer_value_midnight_record_array: OdometerValueMidnightRecordArray,
    vu_card_iw_record_array: VuCardIWRecordRecordArray,
    vu_activity_daily_record_array: VuActivityDailyRecordArray,
    vu_place_daily_work_period_record_array: VuPlaceDailyWorkPeriodRecordArray,
    vu_gnss_ad_record_array: VuGNSSADRecordArray,
    vu_specific_condition_record_array: VuSpecificConditionRecordArray,
    signature_record_array: SignatureRecordArray,
}

impl VuActivitiesBlock {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuActivitiesBlock {
            date_of_day_downloaded_record_array: RecordArray::parse(
                cursor,
                DateOfDayDownloaded::parse,
            )
            .context("Failed to parse date_of_day_downloaded_record_array")?
            .into_inner(),

            odometer_value_midnight_record_array: RecordArray::parse(
                cursor,
                OdometerValueMidnight::parse,
            )
            .context("Failed to parse odometer_value_midnight_record_array")?
            .into_inner(),

            vu_card_iw_record_array: RecordArray::parse(cursor, VuCardIWRecord::parse)
                .context("Failed to parse vu_card_iw_record_array")?
                .into_inner(),

            vu_activity_daily_record_array: RecordArray::parse(cursor, ActivityChangeInfo::parse)
                .context("Failed to parse vu_activity_daily_record_array")?
                .into_inner(),

            vu_place_daily_work_period_record_array: RecordArray::parse(
                cursor,
                VuPlaceDailyWorkPeriod::parse,
            )
            .context("Failed to parse vu_place_daily_work_period_record_array")?
            .into_inner(),

            vu_gnss_ad_record_array: RecordArray::parse(cursor, VuGNSSADRecord::parse)
                .context("Failed to parse vu_gnss_ad_record_array")?
                .into_inner(),

            vu_specific_condition_record_array: RecordArray::parse(
                cursor,
                SpecificConditionRecord::parse,
            )
            .context("Failed to parse vu_specific_condition_record_array")?
            .into_inner(),

            signature_record_array: RecordArray::parse_dyn_size(cursor, Signature::parse_dyn_size)
                .context("Failed to parse signature_record_array")?
                .into_inner(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]

pub struct VuIdentification {
    pub vu_manufacturer_name: VuManufacturerName,
    pub vu_manufacturer_address: VuManufacturerAddress,
    pub vu_part_number: VuPartNumber,
    pub vu_serial_number: VuSerialNumber,
    pub vu_software_identification: VuSoftwareIdentification,
    pub vu_manufacturing_date: VuManufacturingDate,
    pub vu_approval_number: VuApprovalNumber,
    pub vu_generation: Generation,
    pub vu_ability: VuAbility,
    // pub vu_digital_map_version: VuDigitalMapVersion, // Only in Gen2V2, but for some reason it's categorized as "Generation 2" unlike other types, EU please.
}
/// [VuIdentification: appendix 2.206.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27574)
impl VuIdentification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuIdentification {
            vu_manufacturer_name: VuManufacturerName::parse(cursor)
                .context("Failed to parse vu_manufacturer_name")?,
            vu_manufacturer_address: VuManufacturerAddress::parse(cursor)
                .context("Failed to parse vu_manufacturer_address")?,
            vu_part_number: VuPartNumber::parse(cursor)
                .context("Failed to parse vu_part_number")?,
            vu_serial_number: VuSerialNumber::parse(cursor)
                .context("Failed to parse vu_serial_number")?,
            vu_software_identification: VuSoftwareIdentification::parse(cursor)
                .context("Failed to parse vu_software_identification")?,
            vu_manufacturing_date: VuManufacturingDate::parse(cursor)
                .context("Failed to parse vu_manufacturing_date")?,
            vu_approval_number: VuApprovalNumber::parse(cursor)
                .context("Failed to parse vu_approval_number")?,
            vu_generation: Generation::parse(cursor).context("Failed to parse vu_generation")?,
            vu_ability: VuAbility::parse(cursor).context("Failed to parse vu_ability")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SensorPairedRecord {
    pub sensor_serial_number: SensorSerialNumber,
    pub sensor_approval_number: SensorApprovalNumber,
    pub sensor_pairing_date: SensorPairingDate,
}

impl SensorPairedRecord {
    const SIZE: usize = 14;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(SensorPairedRecord {
            sensor_serial_number: SensorSerialNumber::parse(cursor)
                .context("Failed to parse sensor_serial_number")?,
            sensor_approval_number: SensorApprovalNumber::parse(cursor)
                .context("Failed to parse sensor_approval_number")?,
            sensor_pairing_date: SensorPairingDate::parse(cursor)
                .context("Failed to parse sensor_pairing_date")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]

pub struct SensorExternalGNSSCoupledRecord {
    pub sensor_serial_number: SensorGNSSSerialNumber,
    pub sensor_approval_number: SensorExternalGNSSApprovalNumber,
    pub sensor_coupling_date: SensorGNSSCouplingDate,
}

impl SensorExternalGNSSCoupledRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(SensorExternalGNSSCoupledRecord {
            sensor_serial_number: SensorGNSSSerialNumber::parse(cursor)
                .context("Failed to parse sensor_serial_number")?,
            sensor_approval_number: SensorExternalGNSSApprovalNumber::parse(cursor)
                .context("Failed to parse sensor_approval_number")?,
            sensor_coupling_date: SensorGNSSCouplingDate::parse(cursor)
                .context("Failed to parse sensor_coupling_date")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]

pub struct VuCalibrationRecord {
    pub calibration_purpose: CalibrationPurpose,
    pub workshop_name: Name,
    pub workshop_address: Address,
    pub workshop_card_number: FullCardNumber,
    pub workshop_card_expiry_date: TimeReal,
    pub vehicle_identification_number: VehicleIdentificationNumber,
    pub vehicle_registration_identification: VehicleRegistrationIdentification,
    pub w_vehicle_characteristic_constant: WVehicleCharacteristicConstant,
    pub k_constant_of_recording_equipment: KConstantOfRecordingEquipment,
    pub l_tyre_circumference: LTyreCircumference,
    pub tyre_size: TyreSize,
    pub authorised_speed: SpeedAuthorised,
    pub old_odometer_value: OdometerShort,
    pub new_odometer_value: OdometerShort,
    pub old_time_value: TimeReal,
    pub new_time_value: Option<TimeReal>,
    pub next_calibration_date: TimeReal,
    pub seal_data_vu: SealDataVu,
}

impl VuCalibrationRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuCalibrationRecord {
            calibration_purpose: CalibrationPurpose::parse(cursor)
                .context("Failed to parse calibration_purpose")?,
            workshop_name: Name::parse(cursor).context("Failed to parse workshop_name")?,
            workshop_address: Address::parse(cursor).context("Failed to parse workshop_address")?,
            workshop_card_number: FullCardNumber::parse(cursor)
                .context("Failed to parse workshop_card_number")?,
            workshop_card_expiry_date: TimeReal::parse(cursor)
                .context("Failed to parse workshop_card_expiry_date")?,
            vehicle_identification_number: VehicleIdentificationNumber::parse(cursor)
                .context("Failed to parse vehicle_identification_number")?,
            vehicle_registration_identification: VehicleRegistrationIdentification::parse(cursor)
                .context(
                "Failed to parse vehicle_registration_identification",
            )?,
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
            old_time_value: TimeReal::parse(cursor).context("Failed to parse old_time_value")?,
            new_time_value: TimeReal::parse(cursor)
                .context("Failed to parse new_time_value")
                .ok(),
            next_calibration_date: TimeReal::parse(cursor)
                .context("Failed to parse next_calibration_date")?,
            seal_data_vu: SealDataVu::parse(cursor).context("Failed to parse seal_data_vu")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]

pub struct VuCardRecord {
    pub card_number_and_generation_information: Option<FullCardNumberAndGeneration>,
    pub card_extended_serial_number: ExtendedSerialNumber,
    pub card_structure_version: CardStructureVersion,
    pub card_number: Option<CardNumber>,
}

impl VuCardRecord {
    const SIZE: usize = 28;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_number_and_generation_information = FullCardNumberAndGeneration::parse(cursor)
            .context("Failed to parse card_number_and_generation_information")
            .ok();
        let card_extended_serial_number = ExtendedSerialNumber::parse(cursor)
            .context("Failed to parse card_extended_serial_number")?;
        let card_structure_version = CardStructureVersion::parse(cursor)
            .context("Failed to parse card_structure_version")?;

        let card_number = match &card_number_and_generation_information {
            Some(info) => match info.full_card_number.card_type {
                EquipmentType::DriverCard => Some(
                    CardNumber::parse_driver(cursor)
                        .context("Failed to parse driver card number")?,
                ),
                EquipmentType::WorkshopCard => Some(
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

        Ok(VuCardRecord {
            card_number_and_generation_information,
            card_extended_serial_number,
            card_structure_version,
            card_number,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]

pub struct VuITSConsentRecord {
    pub card_number_and_gen: Option<FullCardNumberAndGeneration>,
    pub consent: bool,
}

impl VuITSConsentRecord {
    const SIZE: usize = 20;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_number_and_gen = FullCardNumberAndGeneration::parse(cursor)
            .context("Failed to parse card_number_and_gen")
            .ok();
        let consent = cursor.read_u8().context("Failed to parse consent")? != 0;
        Ok(VuITSConsentRecord {
            card_number_and_gen,
            consent,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]

pub struct VuPowerSupplyInterruptionRecord {
    pub event_type: EventFaultType,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub card_number_and_gen_driver_slot_begin: Option<FullCardNumberAndGeneration>,
    pub card_number_and_gen_driver_slot_end: Option<FullCardNumberAndGeneration>,
    pub card_number_and_gen_codriver_slot_begin: Option<FullCardNumberAndGeneration>,
    pub card_number_and_gen_codriver_slot_end: Option<FullCardNumberAndGeneration>,
    pub similar_events_number: SimilarEventsNumber,
}

impl VuPowerSupplyInterruptionRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuPowerSupplyInterruptionRecord {
            event_type: EventFaultType::parse(cursor).context("Failed to parse event_type")?,
            event_record_purpose: EventFaultRecordPurpose::parse(cursor)
                .context("Failed to parse event_record_purpose")?,
            event_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse event_begin_time")?,
            event_end_time: TimeReal::parse(cursor).context("Failed to parse event_end_time")?,
            card_number_and_gen_driver_slot_begin: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_begin")
                .ok(),
            card_number_and_gen_driver_slot_end: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_end")
                .ok(),
            card_number_and_gen_codriver_slot_begin: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_begin")
                .ok(),
            card_number_and_gen_codriver_slot_end: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_end")
                .ok(),
            similar_events_number: SimilarEventsNumber::parse(cursor)
                .context("Failed to parse similar_events_number")?,
        })
    }
}

pub type VuIdentificationRecordArray = Vec<VuIdentification>;
pub type VuSensorPairedRecordArray = Vec<SensorPairedRecord>;
pub type VuSensorExternalGNSSCoupledRecordArray = Vec<SensorExternalGNSSCoupledRecord>;
pub type VuCalibrationRecordArray = Vec<VuCalibrationRecord>;
pub type VuCardRecordArray = Vec<VuCardRecord>;
pub type VuITSConsentRecordArray = Vec<VuITSConsentRecord>;
pub type VuPowerSupplyInterruptionRecordArray = Vec<VuPowerSupplyInterruptionRecord>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]

pub struct VuCompanyLocksBlock {
    vu_identification_record_array: VuIdentificationRecordArray,
    vu_sensor_paired_record_array: VuSensorPairedRecordArray,
    vu_sensor_external_gnss_coupled_record_array: VuSensorExternalGNSSCoupledRecordArray,
    vu_calibration_record_array: VuCalibrationRecordArray,
    vu_card_record_array: VuCardRecordArray,
    vu_its_consent_record_array: VuITSConsentRecordArray,
    vu_power_supply_interruption_record_array: VuPowerSupplyInterruptionRecordArray,
    signature_record_array: SignatureRecordArray,
}
impl VuCompanyLocksBlock {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuCompanyLocksBlock {
            vu_identification_record_array: RecordArray::parse(cursor, VuIdentification::parse)
                .context("Failed to parse vu_identification_record_array")?
                .into_inner(),

            vu_sensor_paired_record_array: RecordArray::parse(cursor, SensorPairedRecord::parse)
                .context("Failed to parse vu_sensor_paired_record_array")?
                .into_inner(),

            vu_sensor_external_gnss_coupled_record_array: RecordArray::parse(
                cursor,
                SensorExternalGNSSCoupledRecord::parse,
            )
            .context("Failed to parse vu_sensor_external_gnss_coupled_record_array")?
            .into_inner(),

            vu_calibration_record_array: RecordArray::parse(cursor, VuCalibrationRecord::parse)
                .context("Failed to parse vu_calibration_record_array")?
                .into_inner(),

            vu_card_record_array: RecordArray::parse(cursor, VuCardRecord::parse)
                .context("Failed to parse vu_card_record_array")?
                .into_inner(),

            vu_its_consent_record_array: RecordArray::parse(cursor, VuITSConsentRecord::parse)
                .context("Failed to parse vu_its_consent_record_array")?
                .into_inner(),

            vu_power_supply_interruption_record_array: RecordArray::parse(
                cursor,
                VuPowerSupplyInterruptionRecord::parse,
            )
            .context("Failed to parse vu_power_supply_interruption_record_array")?
            .into_inner(),

            signature_record_array: RecordArray::parse_dyn_size(cursor, Signature::parse_dyn_size)
                .context("Failed to parse signature_record_array")?
                .into_inner(),
        })
    }
}

type VuDetailedSpeedBlockRecordArray = Vec<VuDetailedSpeedBlock>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuSpeedBlock {
    pub vu_detailed_speed_block_record_array: VuDetailedSpeedBlockRecordArray,
    pub signature_record_array: SignatureRecordArray,
}

impl VuSpeedBlock {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuSpeedBlock {
            vu_detailed_speed_block_record_array: RecordArray::parse(
                cursor,
                VuDetailedSpeedBlock::parse,
            )
            .context("Failed to parse vu_detailed_speed_block_record_array")?
            .into_inner(),

            signature_record_array: RecordArray::parse_dyn_size(cursor, Signature::parse_dyn_size)
                .context("Failed to parse signature_record_array")?
                .into_inner(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuFaultRecord {
    pub fault_type: EventFaultType,
    pub fault_record_purpose: EventFaultRecordPurpose,
    pub fault_begin_time: TimeReal,
    pub fault_end_time: TimeReal,
    pub card_number_and_gen_driver_slot_begin: Option<FullCardNumberAndGeneration>,
    pub card_number_and_gen_codriver_slot_begin: Option<FullCardNumberAndGeneration>,
    pub card_number_and_gen_driver_slot_end: Option<FullCardNumberAndGeneration>,
    pub card_number_and_gen_codriver_slot_end: Option<FullCardNumberAndGeneration>,
    pub manufacturer_specific_event_fault_data: Option<ManufacturerSpecificEventFaultData>,
}

impl VuFaultRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuFaultRecord {
            fault_type: EventFaultType::parse(cursor).context("Failed to parse fault_type")?,
            fault_record_purpose: EventFaultRecordPurpose::parse(cursor)
                .context("Failed to parse fault_record_purpose")?,
            fault_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse fault_begin_time")?,
            fault_end_time: TimeReal::parse(cursor).context("Failed to parse fault_end_time")?,
            card_number_and_gen_driver_slot_begin: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_begin")
                .ok(),
            card_number_and_gen_codriver_slot_begin: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_begin")
                .ok(),
            card_number_and_gen_driver_slot_end: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_end")
                .ok(),
            card_number_and_gen_codriver_slot_end: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_end")
                .ok(),
            manufacturer_specific_event_fault_data: ManufacturerSpecificEventFaultData::parse(
                cursor,
            )
            .context("Failed to parse manufacturer_specific_event_fault_data")
            .ok(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuEventRecord {
    pub event_type: EventFaultType,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub card_number_and_gen_driver_slot_begin: Option<FullCardNumberAndGeneration>,
    pub card_number_and_gen_codriver_slot_begin: Option<FullCardNumberAndGeneration>,
    pub card_number_and_gen_driver_slot_end: Option<FullCardNumberAndGeneration>,
    pub card_number_and_gen_codriver_slot_end: Option<FullCardNumberAndGeneration>,
    pub similar_events_number: SimilarEventsNumber,
    pub manufacturer_specific_event_fault_data: Option<ManufacturerSpecificEventFaultData>,
}

impl VuEventRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuEventRecord {
            event_type: EventFaultType::parse(cursor).context("Failed to parse event_type")?,
            event_record_purpose: EventFaultRecordPurpose::parse(cursor)
                .context("Failed to parse event_record_purpose")?,
            event_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse event_begin_time")?,
            event_end_time: TimeReal::parse(cursor).context("Failed to parse event_end_time")?,
            card_number_and_gen_driver_slot_begin: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_begin")
                .ok(),
            card_number_and_gen_codriver_slot_begin: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_begin")
                .ok(),
            card_number_and_gen_driver_slot_end: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_end")
                .ok(),
            card_number_and_gen_codriver_slot_end: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_codriver_slot_end")
                .ok(),
            similar_events_number: SimilarEventsNumber::parse(cursor)
                .context("Failed to parse similar_events_number")?,
            manufacturer_specific_event_fault_data: ManufacturerSpecificEventFaultData::parse(
                cursor,
            )
            .context("Failed to parse manufacturer_specific_event_fault_data")
            .ok(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuOverSpeedingControlData {
    pub last_overspeed_control_time: Option<TimeReal>,
    pub first_overspeed_since: Option<TimeReal>,
    pub number_of_overspeed_since: Option<OverspeedNumber>,
}

impl VuOverSpeedingControlData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuOverSpeedingControlData {
            last_overspeed_control_time: TimeReal::parse(cursor).ok(),
            first_overspeed_since: TimeReal::parse(cursor).ok(),
            number_of_overspeed_since: OverspeedNumber::parse(cursor)
                .context("Failed to parse number_of_overspeed_since")
                .ok(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuOverSpeedingEventRecord {
    pub event_type: EventFaultType,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub max_speed_value: SpeedMax,
    pub average_speed_value: SpeedAverage,
    pub card_number_and_gen_driver_slot_begin: Option<FullCardNumberAndGeneration>,
    pub similar_events_number: SimilarEventsNumber,
}

impl VuOverSpeedingEventRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuOverSpeedingEventRecord {
            event_type: EventFaultType::parse(cursor).context("Failed to parse event_type")?,
            event_record_purpose: EventFaultRecordPurpose::parse(cursor)
                .context("Failed to parse event_record_purpose")?,
            event_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse event_begin_time")?,
            event_end_time: TimeReal::parse(cursor).context("Failed to parse event_end_time")?,
            max_speed_value: SpeedMax::parse(cursor).context("Failed to parse max_speed_value")?,
            average_speed_value: SpeedAverage::parse(cursor)
                .context("Failed to parse average_speed_value")?,
            card_number_and_gen_driver_slot_begin: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse card_number_and_gen_driver_slot_begin")
                .ok(),
            similar_events_number: SimilarEventsNumber::parse(cursor)
                .context("Failed to parse similar_events_number")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuTimeAdjustmentRecord {
    pub old_time_value: TimeReal,
    pub new_time_value: TimeReal,
    pub workshop_name: Name,
    pub workshop_address: Address,
    pub workshop_card_number_and_generation: Option<FullCardNumberAndGeneration>,
}

impl VuTimeAdjustmentRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuTimeAdjustmentRecord {
            old_time_value: TimeReal::parse(cursor).context("Failed to parse old_time_value")?,
            new_time_value: TimeReal::parse(cursor).context("Failed to parse new_time_value")?,
            workshop_name: Name::parse(cursor).context("Failed to parse workshop_name")?,
            workshop_address: Address::parse(cursor).context("Failed to parse workshop_address")?,
            workshop_card_number_and_generation: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse workshop_card_number_and_generation")
                .ok(),
        })
    }
}

pub type VuFaultRecordArray = Vec<VuFaultRecord>;
pub type VuEventRecordArray = Vec<VuEventRecord>;
pub type VuOverSpeedingControlDataRecordArray = Vec<VuOverSpeedingControlData>;
pub type VuOverSpeedingEventRecordArray = Vec<VuOverSpeedingEventRecord>;
pub type VuTimeAdjustmentRecordArray = Vec<VuTimeAdjustmentRecord>;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuEventsAndFaultsBlock {
    pub vu_fault_record_array: VuFaultRecordArray,
    pub vu_event_record_array: VuEventRecordArray,
    pub vu_over_speeding_control_data_record_array: VuOverSpeedingControlDataRecordArray,
    pub vu_over_speeding_event_record_array: VuOverSpeedingEventRecordArray,
    pub vu_time_adjustment_record_array: VuTimeAdjustmentRecordArray,
    pub signature_record_array: SignatureRecordArray,
}

impl VuEventsAndFaultsBlock {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuEventsAndFaultsBlock {
            vu_fault_record_array: RecordArray::parse(cursor, VuFaultRecord::parse)
                .context("Failed to parse vu_fault_record_array")?
                .into_inner(),
            vu_event_record_array: RecordArray::parse(cursor, VuEventRecord::parse)
                .context("Failed to parse vu_event_record_array")?
                .into_inner(),
            vu_over_speeding_control_data_record_array: RecordArray::parse(
                cursor,
                VuOverSpeedingControlData::parse,
            )
            .context("Failed to parse vu_over_speeding_control_data_record_array")?
            .into_inner(),
            vu_over_speeding_event_record_array: RecordArray::parse(
                cursor,
                VuOverSpeedingEventRecord::parse,
            )
            .context("Failed to parse vu_over_speeding_event_record_array")?
            .into_inner(),
            vu_time_adjustment_record_array: RecordArray::parse(
                cursor,
                VuTimeAdjustmentRecord::parse,
            )
            .context("Failed to parse vu_time_adjustment_record_array")?
            .into_inner(),
            signature_record_array: RecordArray::parse_dyn_size(cursor, Signature::parse_dyn_size)
                .context("Failed to parse signature_record_array")?
                .into_inner(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuDownloadActivityData {
    pub downloading_time: TimeReal,
    pub full_card_number_and_generation: FullCardNumberAndGeneration,
    pub company_or_workshop_name: Name,
}
impl VuDownloadActivityData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuDownloadActivityData {
            downloading_time: TimeReal::parse(cursor)
                .context("Failed to parse downloading_time")?,
            full_card_number_and_generation: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse full_card_number_and_generation")
                .ok()
                .context("FullCardNumberAndGeneration is None")?,
            company_or_workshop_name: Name::parse(cursor)
                .context("Failed to parse company_or_workshop_name")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuCompanyLocks {
    pub lock_in_time: TimeReal,
    pub lock_out_time: Option<TimeReal>,
    pub company_name: Name,
    pub company_address: Address,
    pub company_card_number_and_generation: FullCardNumberAndGeneration,
}
impl VuCompanyLocks {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuCompanyLocks {
            lock_in_time: TimeReal::parse(cursor).context("Failed to parse lock_in_time")?,
            lock_out_time: TimeReal::parse(cursor)
                .context("Failed to parse lock_out_time")
                .ok(),
            company_name: Name::parse(cursor).context("Failed to parse company_name")?,
            company_address: Address::parse(cursor).context("Failed to parse company_address")?,
            company_card_number_and_generation: FullCardNumberAndGeneration::parse(cursor)
                .context("Failed to parse company_card_number_and_generation")
                .ok()
                .context("FullCardNumberAndGeneration is None")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuControlActivity {
    pub control_type: ControlType,
    pub control_time: TimeReal,
    pub control_card_number_and_generation: FullCardNumberAndGeneration,
    pub download_period_begin_time: TimeReal,
    pub download_period_end_time: TimeReal,
}

impl VuControlActivity {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuControlActivity {
            control_type: ControlType::parse(cursor).context("Failed to parse control_type")?,
            control_time: TimeReal::parse(cursor).context("Failed to parse control_time")?,
            control_card_number_and_generation: FullCardNumberAndGeneration::parse(cursor)
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

pub type MemberStateCertificateRecordArray = Vec<MemberStateCertificate>;
pub type VuCertificateRecordArray = Vec<Certificate>;
pub type VehicleIdentificationNumberRecordArray = Vec<VehicleIdentificationNumber>;
pub type VehicleRegistrationNumberRecordArray = Vec<VehicleRegistrationNumber>;
pub type CurrentDateTimeRecordArray = Vec<CurrentDateTime>;
pub type VuDownloadablePeriodRecordArray = Vec<VuDownloadablePeriod>;
pub type CardSlotsStatusRecordArray = Vec<CardSlotsStatus>;
pub type VuDownloadActivityDataRecordArray = Vec<VuDownloadActivityData>;
pub type VuCompanyLocksRecordArray = Vec<VuCompanyLocks>;
pub type VuControlActivityRecordArray = Vec<VuControlActivity>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuOverviewBlock {
    pub member_state_certificate_record_array: MemberStateCertificateRecordArray,
    pub vu_certificate_record_array: VuCertificateRecordArray,
    pub vehicle_identification_number_record_array: VehicleIdentificationNumberRecordArray,
    pub vehicle_registration_number_record_array: VehicleRegistrationNumberRecordArray,
    pub current_date_time_record_array: CurrentDateTimeRecordArray,
    pub vu_downloadable_period_record_array: VuDownloadablePeriodRecordArray,
    pub card_slots_status_record_array: CardSlotsStatusRecordArray,
    pub vu_download_activity_data_record_array: VuDownloadActivityDataRecordArray,
    pub vu_company_locks_record_array: VuCompanyLocksRecordArray,
    pub vu_control_activity_record_array: VuControlActivityRecordArray,
    pub signature_record_array: SignatureRecordArray,
}

impl VuOverviewBlock {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuOverviewBlock {
            member_state_certificate_record_array: RecordArray::parse_dyn_size(
                cursor,
                Certificate::parse_dyn_size,
            )
            .context("Failed to parse member_state_certificate_record_array")?
            .into_inner(),
            vu_certificate_record_array: RecordArray::parse_dyn_size(
                cursor,
                Certificate::parse_dyn_size,
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
                VuDownloadActivityData::parse,
            )
            .context("Failed to parse vu_download_activity_data_record_array")?
            .into_inner(),
            vu_company_locks_record_array: RecordArray::parse(cursor, VuCompanyLocks::parse)
                .context("Failed to parse vu_company_locks_record_array")?
                .into_inner(),
            vu_control_activity_record_array: RecordArray::parse(cursor, VuControlActivity::parse)
                .context("Failed to parse vu_control_activity_record_array")?
                .into_inner(),
            signature_record_array: RecordArray::parse_dyn_size(cursor, Signature::parse_dyn_size)
                .context("Failed to parse signature_record_array")?
                .into_inner(),
        })
    }
}
