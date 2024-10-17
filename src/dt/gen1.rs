#![allow(dead_code)]
use crate::dt::*;
use anyhow::{Context, Result};
use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum EquipmentType {
    Reserved,
    DriverCard,
    WorkshopCard,
    ControlCard,
    CompanyCard,
    ManufacturingCard,
    VehicleUnit,
    MotionSensor,
    RFU,
}
/// [EquipmentType: appendix 2.67.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e20100)
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
            8..=255 => Ok(EquipmentType::RFU),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [ExtendedSerialNumber: appendix 2.72.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21307)
pub struct ExtendedSerialNumber {
    pub serial_number: u32,
    pub month_year: MonthYear, // Spec says it's a BCDString, but it looks ugly when parsed to a string
    pub type_: u8,
    pub manufacturer_code: external::ManufacturerCode,
}
impl ExtendedSerialNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let serial_number = cursor
            .read_u32::<BigEndian>()
            .context("Failed to read serial number")?;
        let month_year = MonthYear::parse(cursor)?;
        let type_ = cursor.read_u8().context("Failed to read type_")?;
        let manufacturer_code = external::ManufacturerCode::parse(cursor)?;

        Ok(ExtendedSerialNumber {
            serial_number,
            month_year,
            type_,
            manufacturer_code,
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
/// [CalibrationPurpose: appendix 2.8.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16597)
pub enum CalibrationPurpose {
    Reserved,
    Activation,
    FirstInstallation,
    Installation,
    PeriodicInspection,
}

impl CalibrationPurpose {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to read CalibrationPurpose")?;
        match value {
            0x00 => Ok(CalibrationPurpose::Reserved),
            0x01 => Ok(CalibrationPurpose::Activation),
            0x02 => Ok(CalibrationPurpose::FirstInstallation),
            0x03 => Ok(CalibrationPurpose::Installation),
            0x04 => Ok(CalibrationPurpose::PeriodicInspection),
            _ => anyhow::bail!("Invalid CalibrationPurpose value: {}", value),
        }
    }
}

pub type SensorSerialNumber = ExtendedSerialNumber;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [SensorApprovalNumber: appendix 2.131.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23887)
pub struct SensorApprovalNumber(IA5String);
impl SensorApprovalNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = IA5String::parse_dyn_size(cursor, 8)?;
        Ok(SensorApprovalNumber(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuApprovalNumber: appendix 2.172.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25427)
pub struct VuApprovalNumber(IA5String);
impl VuApprovalNumber {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = IA5String::parse_dyn_size(cursor, 8)?;
        Ok(VuApprovalNumber(value))
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
    VUSecurityBreachAttemptNoFurtherDetails,
    MotionSensorAuthenticationFailure,
    TachographCardAuthenticationFailure,
    UnauthorizedChangeOfMotionSensor,
    CardDataInputIntegrityError,
    StoredUserDataIntegrityError,
    InternalDataTransferError,
    UnauthorizedCaseOpening,
    HardwareSabotage,
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
            0x00 => Ok(EventFaultType::NoFurtherDetails),
            0x01 => Ok(EventFaultType::InsertionOfNonValidCard),
            0x02 => Ok(EventFaultType::CardConflict),
            0x03 => Ok(EventFaultType::TimeOverlap),
            0x04 => Ok(EventFaultType::DrivingWithoutAppropriateCard),
            0x05 => Ok(EventFaultType::CardInsertionWhileDriving),
            0x06 => Ok(EventFaultType::LastCardSessionNotCorrectlyClosed),
            0x07 => Ok(EventFaultType::OverSpeeding),
            0x08 => Ok(EventFaultType::PowerSupplyInterruption),
            0x09 => Ok(EventFaultType::MotionDataError),
            0x0A => Ok(EventFaultType::VehicleMotionConflict),
            0x0B..=0x0F => Ok(EventFaultType::RFU),

            // Vehicle unit related security breach attempt events,
            0x10 => Ok(EventFaultType::VUSecurityBreachAttemptNoFurtherDetails),
            0x11 => Ok(EventFaultType::MotionSensorAuthenticationFailure),
            0x12 => Ok(EventFaultType::TachographCardAuthenticationFailure),
            0x13 => Ok(EventFaultType::UnauthorizedChangeOfMotionSensor),
            0x14 => Ok(EventFaultType::CardDataInputIntegrityError),
            0x15 => Ok(EventFaultType::StoredUserDataIntegrityError),
            0x16 => Ok(EventFaultType::InternalDataTransferError),
            0x17 => Ok(EventFaultType::UnauthorizedCaseOpening),
            0x18 => Ok(EventFaultType::HardwareSabotage),
            0x19..=0x1F => Ok(EventFaultType::RFU),

            // Sensor related security breach attempt events,
            0x20 => Ok(EventFaultType::SensorSecurityBreachAttemptNoFurtherDetails),
            0x21 => Ok(EventFaultType::SensorAuthenticationFailure),
            0x22 => Ok(EventFaultType::SensorStoredDataIntegrityError),
            0x23 => Ok(EventFaultType::SensorInternalDataTransferError),
            0x24 => Ok(EventFaultType::SensorUnauthorizedCaseOpening),
            0x25 => Ok(EventFaultType::SensorHardwareSabotage),
            0x26..=0x2F => Ok(EventFaultType::RFU),

            // Recording equipment faults,
            0x30 => Ok(EventFaultType::ControlDeviceFaultNoFurtherDetails),
            0x31 => Ok(EventFaultType::VUInternalFault),
            0x32 => Ok(EventFaultType::PrinterFault),
            0x33 => Ok(EventFaultType::DisplayFault),
            0x34 => Ok(EventFaultType::DownloadingFault),
            0x35 => Ok(EventFaultType::SensorFault),
            0x36..=0x3F => Ok(EventFaultType::RFU),

            // Card faults,
            0x40 => Ok(EventFaultType::NoFurtherDetails),
            0x41..=0x4F => Ok(EventFaultType::RFU),

            // Reserved for future use,
            0x50..=0x7F => Ok(EventFaultType::RFU),

            // Manufacturer specific,
            0x80..=0xFF => Ok(EventFaultType::ManufacturerSpecific),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// [SpecificConditionType: appendix 2.154.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24685)
pub enum SpecificConditionType {
    RFU,
    OutOfScopeBegin,
    OutOfScopeEnd,
    FerryTrainCrossingBegin,
}

impl SpecificConditionType {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to read value for SpecificConditionType")?;
        match value {
            0x0 => Ok(SpecificConditionType::RFU),
            0x1 => Ok(SpecificConditionType::OutOfScopeBegin),
            0x2 => Ok(SpecificConditionType::OutOfScopeEnd),
            0x3 => Ok(SpecificConditionType::FerryTrainCrossingBegin),
            0x4..=0xFF => Ok(SpecificConditionType::RFU),
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
        let mut buf = vec![0u8; Self::SIZE];
        cursor.read_exact(&mut buf).context("Failed to read buf")?;
        let mut inner_cursor = Cursor::new(buf.as_slice());

        let entry_time = TimeReal::parse(&mut inner_cursor)?;
        let specific_condition_type = SpecificConditionType::parse(&mut inner_cursor)?;
        if specific_condition_type == SpecificConditionType::RFU {
            return Err(anyhow::anyhow!(
                "RFU value found in SpecificConditionRecord"
            ));
        }
        Ok(SpecificConditionRecord {
            entry_time,
            specific_condition_type,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [SpecificConditions: appendix 2.153.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24644)
pub struct SpecificConditions {
    pub specific_condition_records: Vec<SpecificConditionRecord>,
}
impl SpecificConditions {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let mut specific_condition_records = Vec::new();
        let no_of_records = size as usize / SpecificConditionRecord::SIZE as usize;
        for _ in 0..no_of_records {
            if let Ok(specific_condition_record) = SpecificConditionRecord::parse(cursor) {
                specific_condition_records.push(specific_condition_record);
            }
        }
        // Sort the records by time_stamp in ascending order
        specific_condition_records
            .sort_by(|a, b| a.entry_time.0.timestamp().cmp(&b.entry_time.0.timestamp()));
        Ok(SpecificConditions {
            specific_condition_records: specific_condition_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [Certificate: appendix 2.41.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e18396)
pub struct Certificate(Vec<u8>);
impl Certificate {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut value = vec![0u8; 194];
        cursor
            .read_exact(&mut value)
            .context("Failed to read certificate")?;
        Ok(Certificate(value))
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
        if card_type == EquipmentType::RFU {
            return Err(anyhow::anyhow!("RFU value found in FullCardNumber"));
        }

        Ok(FullCardNumber {
            card_type,
            card_issuing_member_state,
            card_number,
        })
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
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [Signature: appendix 2.149.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24501)
pub struct Signature(Vec<u8>); // Octet string
impl Signature {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut signature_buffer = vec![0u8; 128];
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
}
impl PreviousVehicleInfo {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let vehicle_registration_identification = VehicleRegistrationIdentification::parse(cursor)?;
        let card_withdrawal_time = TimeReal::parse(cursor)?;
        Ok(PreviousVehicleInfo {
            vehicle_registration_identification,
            card_withdrawal_time,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// [RegionNumeric: appendix 2.122.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23612)
pub enum RegionNumeric {
    NoInformation,
    Andalucia,
    Aragon,
    Asturias,
    Cantabria,
    Cataluna,
    CastillaLeon,
    CastillaLaMancha,
    Valencia,
    Extremadura,
    Galicia,
    Baleares,
    Canarias,
    LaRioja,
    Madrid,
    Murcia,
    Navarra,
    PaisVasco,
}

impl RegionNumeric {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor.read_u8().context("Failed to read region_numeric")?;
        match value {
            0x00 => Ok(RegionNumeric::NoInformation),
            0x01 => Ok(RegionNumeric::Andalucia),
            0x02 => Ok(RegionNumeric::Aragon),
            0x03 => Ok(RegionNumeric::Asturias),
            0x04 => Ok(RegionNumeric::Cantabria),
            0x05 => Ok(RegionNumeric::Cataluna),
            0x06 => Ok(RegionNumeric::CastillaLeon),
            0x07 => Ok(RegionNumeric::CastillaLaMancha),
            0x08 => Ok(RegionNumeric::Valencia),
            0x09 => Ok(RegionNumeric::Extremadura),
            0x0A => Ok(RegionNumeric::Galicia),
            0x0B => Ok(RegionNumeric::Baleares),
            0x0C => Ok(RegionNumeric::Canarias),
            0x0D => Ok(RegionNumeric::LaRioja),
            0x0E => Ok(RegionNumeric::Madrid),
            0x0F => Ok(RegionNumeric::Murcia),
            0x10 => Ok(RegionNumeric::Navarra),
            0x11 => Ok(RegionNumeric::PaisVasco),
            _ => anyhow::bail!("Invalid RegionNumeric value: {}", value),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// [EntryTypeDailyWorkPeriod: appendix 2.66.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e20044)
pub enum EntryTypeDailyWorkPeriod {
    BeginRelatedTimeCardInsertionTimeOrTimeOfEntry,
    EndRelatedTimeCardWithdrawalTimeOrTimeOfEntry,
    BeginRelatedTimeManuallyEntered,
    EndRelatedTimeManuallyEntered,
    BeginRelatedTimeAssumedByVU,
    EndRelatedTimeAssumedByVU,
}

impl EntryTypeDailyWorkPeriod {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to read EntryTypeDailyWorkPeriod")?;
        match value {
            0x00 => Ok(EntryTypeDailyWorkPeriod::BeginRelatedTimeCardInsertionTimeOrTimeOfEntry),
            0x01 => Ok(EntryTypeDailyWorkPeriod::EndRelatedTimeCardWithdrawalTimeOrTimeOfEntry),
            0x02 => Ok(EntryTypeDailyWorkPeriod::BeginRelatedTimeManuallyEntered),
            0x03 => Ok(EntryTypeDailyWorkPeriod::EndRelatedTimeManuallyEntered),
            0x04 => Ok(EntryTypeDailyWorkPeriod::BeginRelatedTimeAssumedByVU),
            0x05 => Ok(EntryTypeDailyWorkPeriod::EndRelatedTimeAssumedByVU),
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
}
impl PlaceRecord {
    const SIZE: usize = 10;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut buf = vec![0u8; Self::SIZE];
        cursor.read_exact(&mut buf).context("Failed to read buf")?;
        let mut inner_cursor = Cursor::new(buf.as_slice());

        let entry_time = TimeReal::parse(&mut inner_cursor)?;
        let entry_type_daily_work_period = EntryTypeDailyWorkPeriod::parse(&mut inner_cursor)?;
        let daily_work_period_country = external::NationNumeric::parse(&mut inner_cursor)?;
        let daily_work_period_region = RegionNumeric::parse(&mut inner_cursor)?;
        let vehicle_odometer_value = OdometerShort::parse(&mut inner_cursor)?;
        if entry_time.0.timestamp() == 0 {
            anyhow::bail!("Invalid entry_time in PlaceRecord");
        }
        Ok(PlaceRecord {
            entry_time,
            entry_type_daily_work_period,
            daily_work_period_country,
            daily_work_period_region,
            vehicle_odometer_value,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [DriverCardApplicationIdentification: appendix 2.61.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e19751)
pub struct DriverCardApplicationIdentification {
    pub type_of_tachograph_card_id: EquipmentType,
    pub card_structure_version: [u8; 2],
    pub no_of_events_per_type: u8,
    pub no_of_faults_per_type: u8,
    pub activity_structure_length: u16,
    pub no_of_card_vehicle_records: u16,
    pub no_of_card_place_records: u8,
}

impl DriverCardApplicationIdentification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let type_of_tachograph_card_id = EquipmentType::parse(cursor)?;

        let mut card_structure_version = [0u8; 2];
        cursor
            .read_exact(&mut card_structure_version)
            .context("Failed to read card_structure_version")?;

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
            .read_u8()
            .context("Failed to read no_of_card_place_records")?;

        Ok(DriverCardApplicationIdentification {
            type_of_tachograph_card_id,
            card_structure_version,
            no_of_events_per_type,
            no_of_faults_per_type,
            activity_structure_length,
            no_of_card_vehicle_records,
            no_of_card_place_records,
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
        let mut buf = vec![0u8; Self::SIZE];
        cursor.read_exact(&mut buf).context("Failed to read buf")?;
        let mut inner_cursor = Cursor::new(buf.as_slice());

        let event_type = EventFaultType::parse(&mut inner_cursor)?;
        let event_begin_time = TimeReal::parse(&mut inner_cursor)?;
        let event_end_time = TimeReal::parse(&mut inner_cursor)?;
        let event_vehicle_registration =
            VehicleRegistrationIdentification::parse(&mut inner_cursor)?;

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
    const OUTER_RECORDS_AMOUNT: usize = 6;

    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let mut card_event_records = Vec::new();
        let inner_record_amounts = size / Self::OUTER_RECORDS_AMOUNT / CardEventRecord::SIZE;

        // According to the spec, there are ALWAYS 6 outer CardEventRecords, but we'll use the size from header anyway
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
    const SIZE: usize = 24;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut buf = vec![0u8; Self::SIZE];
        cursor.read_exact(&mut buf).context("Failed to read buf")?;
        let mut inner_cursor = Cursor::new(buf.as_slice());

        let fault_type = EventFaultType::parse(&mut inner_cursor)?;
        let fault_begin_time = TimeReal::parse(&mut inner_cursor)?;
        let fault_end_time = TimeReal::parse(&mut inner_cursor)?;
        let fault_vehicle_registration =
            VehicleRegistrationIdentification::parse(&mut inner_cursor)?;

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
    const OUTER_RECORDS_AMOUNT: usize = 2;

    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let mut card_fault_records = Vec::new();

        let max_inner_records = size / Self::OUTER_RECORDS_AMOUNT / CardFaultRecord::SIZE;

        // According to the spec, there are ALWAYS 2 outer CardFaultRecords, but we'll use the computed size just in case
        for _ in 0..Self::OUTER_RECORDS_AMOUNT {
            let mut inner_card_fault_records = Vec::new();
            for _ in 0..max_inner_records {
                if let Ok(card_fault_record) = CardFaultRecord::parse(cursor) {
                    inner_card_fault_records.push(card_fault_record);
                }
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
}
impl CardVehicleRecord {
    const SIZE: usize = 31;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut buf = vec![0u8; Self::SIZE];
        cursor.read_exact(&mut buf).context("Failed to read buf")?;
        let mut inner_cursor = Cursor::new(buf.as_slice());

        Ok(CardVehicleRecord {
            vehicle_odometer_begin: OdometerShort::parse(&mut inner_cursor)?,
            vehicle_odometer_end: OdometerShort::parse(&mut inner_cursor)?,
            vehicle_first_use: TimeReal::parse(&mut inner_cursor)?,
            vehicle_last_use: TimeReal::parse(&mut inner_cursor)?,
            vehicle_registration: VehicleRegistrationIdentification::parse(&mut inner_cursor)?,
            vu_data_block_counter: VuDataBlockCounter::parse(&mut inner_cursor)?,
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
        let vehicle_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read vehicle_pointer_newest_record")?;

        let mut card_vehicle_records = Vec::new();
        let amount_of_records = size as usize / CardVehicleRecord::SIZE as usize;
        for _ in 0..amount_of_records {
            if let Ok(card_vehicle_record) = CardVehicleRecord::parse(cursor) {
                card_vehicle_records.push(card_vehicle_record);
            }
        }

        Ok(CardVehiclesUsed {
            vehicle_pointer_newest_record,
            card_vehicle_records,
        })
    }
}

/// [NoOfCardPlaceRecords: appendix 2.104.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22566)
type NoOfCardPlaceRecords = u8;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [CardPlaceDailyWorkPeriod: appendix 2.27.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17729)
pub struct CardPlaceDailyWorkPeriod {
    place_pointer_newest_record: NoOfCardPlaceRecords,
    place_records: Vec<PlaceRecord>,
}
impl CardPlaceDailyWorkPeriod {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let place_pointer_newest_record = cursor
            .read_u8()
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
/// [VuDownloadActivityData: appendix 2.195.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26758)
pub struct VuDownloadActivityData {
    pub downloading_time: TimeReal,
    pub full_card_number: FullCardNumber,
    pub company_or_workshop_name: Name,
}

impl VuDownloadActivityData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            downloading_time: TimeReal::parse(cursor)
                .context("Failed to parse downloading_time")?,
            full_card_number: FullCardNumber::parse(cursor)
                .context("Failed to parse full_card_number")?,
            company_or_workshop_name: Name::parse(cursor)
                .context("Failed to parse company_or_workshop_name")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuCompanyLocksData: appendix 2.183.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26258)
pub struct VuCompanyLocksData {
    pub no_of_locks: u8,
    pub vu_company_locks_records: Vec<VuCompanyLocksRecord>,
}

impl VuCompanyLocksData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_locks = cursor.read_u8().context("Failed to read no_of_locks")?;
        let mut vu_company_locks_records = Vec::with_capacity(no_of_locks as usize);
        for _ in 0..no_of_locks {
            vu_company_locks_records.push(
                VuCompanyLocksRecord::parse(cursor)
                    .context("Failed to parse VuCompanyLocksRecord")?,
            );
        }

        Ok(Self {
            no_of_locks,
            vu_company_locks_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuCompanyLocksRecord: appendix 2.184.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26153)
pub struct VuCompanyLocksRecord {
    pub lock_in_time: TimeReal,
    pub lock_out_time: Option<TimeReal>,
    pub company_name: Name,
    pub company_address: Address,
    pub company_card_number: FullCardNumber,
}
impl VuCompanyLocksRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            lock_in_time: TimeReal::parse(cursor).context("Failed to parse lock_in_time")?,
            lock_out_time: TimeReal::parse(cursor)
                .context("Failed to parse lock_out_time")
                .ok(),
            company_name: Name::parse(cursor).context("Failed to parse company_name")?,
            company_address: Address::parse(cursor).context("Failed to parse company_address")?,
            company_card_number: FullCardNumber::parse(cursor)
                .context("Failed to parse company_card_number")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuControlActivityData: appendix 2.186.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26342)
pub struct VuControlActivityData {
    pub no_of_controls: u8,
    pub vu_control_activity_records: Vec<VuControlActivityRecord>,
}

impl VuControlActivityData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_controls = cursor.read_u8().context("Failed to read no_of_controls")?;

        let mut vu_control_activity_records = Vec::with_capacity(no_of_controls as usize);
        for _ in 0..no_of_controls {
            vu_control_activity_records.push(
                VuControlActivityRecord::parse(cursor)
                    .context("Failed to parse VuControlActivityRecord")?,
            );
        }

        Ok(Self {
            no_of_controls,
            vu_control_activity_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuControlActivityRecord: appendix 2.187.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26392)
pub struct VuControlActivityRecord {
    pub control_type: ControlType,
    pub control_time: TimeReal,
    pub control_card_number: FullCardNumber,
    pub download_period_begin_time: TimeReal,
    pub download_period_end_time: TimeReal,
}

impl VuControlActivityRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            control_type: ControlType::parse(cursor).context("Failed to parse control_type")?,
            control_time: TimeReal::parse(cursor).context("Failed to parse control_time")?,
            control_card_number: FullCardNumber::parse(cursor)
                .context("Failed to parse control_card_number")?,
            download_period_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse download_period_begin_time")?,
            download_period_end_time: TimeReal::parse(cursor)
                .context("Failed to parse download_period_end_time")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuOverviewBlock page 342]
pub struct VuOverviewBlock {
    pub member_state_certificate: Certificate,
    pub vu_certificate: Certificate,
    pub vehicle_identification_number: VehicleIdentificationNumber,
    pub vehicle_registration_identification: VehicleRegistrationIdentification,
    pub current_date_time: TimeReal,
    pub vu_downloadable_period: VuDownloadablePeriod,
    pub card_slots_status: CardSlotsStatus,
    pub vu_download_activity_data: VuDownloadActivityData,
    pub vu_company_locks_data: VuCompanyLocksData,
    pub vu_control_activity_data: VuControlActivityData,
    pub signature: Signature,
}

impl VuOverviewBlock {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let member_state_certificate =
            Certificate::parse(cursor).context("Failed to parse member_state_certificate")?;
        let vu_certificate =
            Certificate::parse(cursor).context("Failed to parse vu_certificate")?;
        let vehicle_identification_number = VehicleIdentificationNumber::parse(cursor)
            .context("Failed to parse vehicle_identification_number")?;
        let vehicle_registration_identification = VehicleRegistrationIdentification::parse(cursor)
            .context("Failed to parse vehicle_registration_identification")?;
        let current_date_time =
            TimeReal::parse(cursor).context("Failed to parse current_date_time")?;
        let vu_downloadable_period = VuDownloadablePeriod::parse(cursor)
            .context("Failed to parse vu_downloadable_period")?;
        let card_slots_status =
            CardSlotsStatus::parse(cursor).context("Failed to parse card_slots_status")?;
        let vu_download_activity_data = VuDownloadActivityData::parse(cursor)
            .context("Failed to parse vu_download_activity_data")?;
        let vu_company_locks_data =
            VuCompanyLocksData::parse(cursor).context("Failed to parse vu_company_locks_data")?;
        let vu_control_activity_data = VuControlActivityData::parse(cursor)
            .context("Failed to parse vu_control_activity_data")?;
        let signature = Signature::parse(cursor).context("Failed to parse signature")?;

        Ok(Self {
            member_state_certificate,
            vu_certificate,
            vehicle_identification_number,
            vehicle_registration_identification,
            current_date_time,
            vu_downloadable_period,
            card_slots_status,
            vu_download_activity_data,
            vu_company_locks_data,
            vu_control_activity_data,
            signature,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuCardIWData {
    pub no_of_iw_records: u16,
    pub vu_card_iw_records: Vec<VuCardIWRecord>,
}

impl VuCardIWData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_iw_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read number of VuCardIWRecords")?;

        let mut vu_card_iw_records = Vec::with_capacity(no_of_iw_records as usize);
        for _ in 0..no_of_iw_records {
            vu_card_iw_records
                .push(VuCardIWRecord::parse(cursor).context("Failed to parse VuCardIWRecord")?);
        }

        Ok(Self {
            no_of_iw_records,
            vu_card_iw_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuCardIWRecord: appendix 2.177.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25809)
pub struct VuCardIWRecord {
    pub card_holder_name: HolderName,
    pub full_card_number: FullCardNumber,
    pub card_expiry_date: TimeReal,
    pub card_insertion_time: TimeReal,
    pub vehicle_odometer_value_at_insertion: OdometerShort,
    pub card_slot_number: CardSlotNumber,
    pub card_withdrawal_time: Option<TimeReal>,
    pub vehicle_odometer_value_at_withdrawal: OdometerShort,
    pub previous_vehicle_info: PreviousVehicleInfo,
    pub manual_entry_flag: ManualInputFlag,
}

impl VuCardIWRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            card_holder_name: HolderName::parse(cursor)
                .context("Failed to parse card_holder_name")?,
            full_card_number: FullCardNumber::parse(cursor)
                .context("Failed to parse full_card_number")?,
            card_expiry_date: TimeReal::parse(cursor)
                .context("Failed to parse card_expiry_date")?,
            card_insertion_time: TimeReal::parse(cursor)
                .context("Failed to parse card_insertion_time")?,
            vehicle_odometer_value_at_insertion: OdometerShort::parse(cursor)
                .context("Failed to parse vehicle_odometer_value_at_insertion")?,
            card_slot_number: CardSlotNumber::parse(cursor)
                .context("Failed to parse card_slot_number")?,
            card_withdrawal_time: TimeReal::parse(cursor)
                .context("Failed to parse card_withdrawal_time")
                .ok(),
            vehicle_odometer_value_at_withdrawal: OdometerShort::parse(cursor)
                .context("Failed to parse vehicle_odometer_value_at_withdrawal")?,
            previous_vehicle_info: PreviousVehicleInfo::parse(cursor)
                .context("Failed to parse previous_vehicle_info")?,
            manual_entry_flag: ManualInputFlag::parse(cursor)
                .context("Failed to parse manual_entry_flag")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuActivityDailyData: appendix 2.170.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25344)
pub struct VuActivityDailyData {
    pub no_of_activity_changes: u16,
    pub activity_change_infos: Vec<ActivityChangeInfo>,
}

impl VuActivityDailyData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_activity_changes = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read no_of_activity_changes")?;

        let mut activity_change_infos = Vec::with_capacity(no_of_activity_changes as usize);
        for _ in 0..no_of_activity_changes {
            activity_change_infos.push(
                ActivityChangeInfo::parse(cursor).context("Failed to parse ActivityChangeInfo")?,
            );
        }

        Ok(Self {
            no_of_activity_changes,
            activity_change_infos,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuPlaceDailyWorkPeriodRecord: appendix 2.219.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28313)
pub struct VuPlaceDailyWorkPeriodRecord {
    pub full_card_number: FullCardNumber,
    pub place_record: PlaceRecord,
}

impl VuPlaceDailyWorkPeriodRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            full_card_number: FullCardNumber::parse(cursor)
                .context("Failed to parse full_card_number")?,
            place_record: PlaceRecord::parse(cursor).context("Failed to parse place_record")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuPlaceDailyWorkPeriodData: appendix 2.218.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28280)
pub struct VuPlaceDailyWorkPeriodData {
    pub no_of_place_records: u8,
    pub vu_place_daily_work_period_records: Vec<VuPlaceDailyWorkPeriodRecord>,
}

impl VuPlaceDailyWorkPeriodData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_place_records = cursor
            .read_u8()
            .context("Failed to read no_of_place_records")?;

        let mut vu_place_daily_work_period_records =
            Vec::with_capacity(no_of_place_records as usize);
        for _ in 0..no_of_place_records {
            vu_place_daily_work_period_records.push(
                VuPlaceDailyWorkPeriodRecord::parse(cursor)
                    .context("Failed to parse VuPlaceDailyWorkPeriodRecord")?,
            );
        }

        Ok(Self {
            no_of_place_records,
            vu_place_daily_work_period_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuSpecificConditionRecord: appendix 2.152.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24614)
pub struct VuSpecificConditionRecord {
    pub entry_time: TimeReal,
    pub specific_condition_type: SpecificConditionType,
}

impl VuSpecificConditionRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            entry_time: TimeReal::parse(cursor).context("Failed to parse entry_time")?,
            specific_condition_type: SpecificConditionType::parse(cursor)
                .context("Failed to parse specific_condition_type")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuSpecificConditionData: appendix 2.227.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28591)
pub struct VuSpecificConditionData {
    pub no_of_specific_conditions: u16,
    pub specific_condition_records: Vec<SpecificConditionRecord>,
}

impl VuSpecificConditionData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_specific_conditions = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read no_of_specific_conditions")?;

        let mut specific_condition_records = Vec::with_capacity(no_of_specific_conditions as usize);
        for _ in 0..no_of_specific_conditions {
            specific_condition_records.push(
                SpecificConditionRecord::parse(cursor)
                    .context("Failed to parse SpecificConditionRecord")?,
            );
        }

        Ok(Self {
            no_of_specific_conditions,
            specific_condition_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// Page 344 TREP 02
pub struct VuActivitiesBlock {
    pub time_real: TimeReal,
    pub odometer_value_midnight: OdometerValueMidnight,
    pub vu_card_iw_data: VuCardIWData,
    pub vu_activity_daily_data: VuActivityDailyData,
    pub vu_place_daily_work_period_data: VuPlaceDailyWorkPeriodData,
    pub vu_specific_condition_data: VuSpecificConditionData,
    pub signature: Signature,
}
impl VuActivitiesBlock {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            time_real: TimeReal::parse(cursor).context("Failed to parse time_real")?,
            odometer_value_midnight: OdometerShort::parse(cursor)
                .context("Failed to parse odometer_value_midnight")?,
            vu_card_iw_data: VuCardIWData::parse(cursor)
                .context("Failed to parse vu_card_iw_data")?,
            vu_activity_daily_data: VuActivityDailyData::parse(cursor)
                .context("Failed to parse vu_activity_daily_data")?,
            vu_place_daily_work_period_data: VuPlaceDailyWorkPeriodData::parse(cursor)
                .context("Failed to parse vu_place_daily_work_period_data")?,
            vu_specific_condition_data: VuSpecificConditionData::parse(cursor)
                .context("Failed to parse vu_specific_condition_data")?,
            signature: Signature::parse(cursor).context("Failed to parse signature")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuFaultRecord: appendix 2.201.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27156)
pub struct VuFaultRecord {
    pub fault_type: EventFaultType,
    pub fault_record_purpose: EventFaultRecordPurpose,
    pub fault_begin_time: TimeReal,
    pub fault_end_time: TimeReal,
    pub card_number_driver_slot_begin: Option<FullCardNumber>,
    pub card_number_codriver_slot_begin: Option<FullCardNumber>,
    pub card_number_driver_slot_end: Option<FullCardNumber>,
    pub card_number_codriver_slot_end: Option<FullCardNumber>,
}
impl VuFaultRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            fault_type: EventFaultType::parse(cursor).context("Failed to parse fault_type")?,
            fault_record_purpose: EventFaultRecordPurpose::parse(cursor)
                .context("Failed to parse fault_record_purpose")?,
            fault_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse fault_begin_time")?,
            fault_end_time: TimeReal::parse(cursor).context("Failed to parse fault_end_time")?,
            card_number_driver_slot_begin: FullCardNumber::parse(cursor)
                .context("Failed to parse card_number_driver_slot_begin")
                .ok(),
            card_number_codriver_slot_begin: FullCardNumber::parse(cursor)
                .context("Failed to parse card_number_codriver_slot_begin")
                .ok(),
            card_number_driver_slot_end: FullCardNumber::parse(cursor)
                .context("Failed to parse card_number_driver_slot_end")
                .ok(),
            card_number_codriver_slot_end: FullCardNumber::parse(cursor)
                .context("Failed to parse card_number_codriver_slot_end")
                .ok(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuFaultRecord: appendix 2.200.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27122)
pub struct VuFaultData {
    pub no_of_vu_fault_records: u8,
    pub vu_fault_records: Vec<VuFaultRecord>,
}
impl VuFaultData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_vu_fault_records = cursor
            .read_u8()
            .context("Failed to read no_of_vu_fault_records")?;
        let mut vu_fault_records = Vec::with_capacity(no_of_vu_fault_records as usize);
        for _ in 0..no_of_vu_fault_records {
            vu_fault_records
                .push(VuFaultRecord::parse(cursor).context("Failed to parse VuFaultRecord")?);
        }

        Ok(Self {
            no_of_vu_fault_records,
            vu_fault_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuEventRecord: appendix 2.197.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26910)
pub struct VuEventRecord {
    pub event_type: EventFaultType,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub card_number_driver_slot_begin: Option<FullCardNumber>,
    pub card_number_codriver_slot_begin: Option<FullCardNumber>,
    pub card_number_driver_slot_end: Option<FullCardNumber>,
    pub card_number_codriver_slot_end: Option<FullCardNumber>,
    pub similar_events_number: SimilarEventsNumber,
}
impl VuEventRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            event_type: EventFaultType::parse(cursor).context("Failed to parse event_type")?,
            event_record_purpose: EventFaultRecordPurpose::parse(cursor)
                .context("Failed to parse event_record_purpose")?,
            event_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse event_begin_time")?,
            event_end_time: TimeReal::parse(cursor).context("Failed to parse event_end_time")?,
            card_number_driver_slot_begin: FullCardNumber::parse(cursor)
                .context("Failed to parse card_number_driver_slot_begin")
                .ok(),
            card_number_codriver_slot_begin: FullCardNumber::parse(cursor)
                .context("Failed to parse card_number_codriver_slot_begin")
                .ok(),
            card_number_driver_slot_end: FullCardNumber::parse(cursor)
                .context("Failed to parse card_number_driver_slot_end")
                .ok(),
            card_number_codriver_slot_end: FullCardNumber::parse(cursor)
                .context("Failed to parse card_number_codriver_slot_end")
                .ok(),
            similar_events_number: SimilarEventsNumber::parse(cursor)
                .context("Failed to parse similar_events_number")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuEventData: appendix 2.197.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26876)
pub struct VuEventData {
    pub no_of_vu_event_records: u8,
    pub vu_event_records: Vec<VuEventRecord>,
}
impl VuEventData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_vu_event_records = cursor
            .read_u8()
            .context("Failed to read no_of_vu_event_records")?;
        let mut vu_event_records = Vec::with_capacity(no_of_vu_event_records as usize);
        for _ in 0..no_of_vu_event_records {
            vu_event_records
                .push(VuEventRecord::parse(cursor).context("Failed to parse VuEventRecord")?);
        }

        Ok(Self {
            no_of_vu_event_records,
            vu_event_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuOverSpeedingControlData: appendix 2.212.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27978)
pub struct VuOverSpeedingControlData {
    pub last_overspeed_control_time: TimeReal,
    pub first_overspeed_since: TimeReal,
    pub number_of_overspeed_since: OverspeedNumber,
}
impl VuOverSpeedingControlData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            last_overspeed_control_time: TimeReal::parse(cursor)
                .context("Failed to parse last_overspeed_control_time")?,
            first_overspeed_since: TimeReal::parse(cursor)
                .context("Failed to parse first_overspeed_since")?,
            number_of_overspeed_since: OverspeedNumber::parse(cursor)
                .context("Failed to parse number_of_overspeed_since")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuOverSpeedingEventRecord: appendix 2.215.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28097)
pub struct VuOverSpeedingEventRecord {
    pub event_type: EventFaultType,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub max_speed_value: SpeedMax,
    pub average_speed_value: SpeedAverage,
    pub card_number_driver_slot: FullCardNumber,
    pub similar_events_number: SimilarEventsNumber,
}
impl VuOverSpeedingEventRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            event_type: EventFaultType::parse(cursor).context("Failed to parse event_type")?,
            event_record_purpose: EventFaultRecordPurpose::parse(cursor)
                .context("Failed to parse event_record_purpose")?,
            event_begin_time: TimeReal::parse(cursor)
                .context("Failed to parse event_begin_time")?,
            event_end_time: TimeReal::parse(cursor).context("Failed to parse event_end_time")?,
            max_speed_value: SpeedMax::parse(cursor).context("Failed to parse max_speed_value")?,
            average_speed_value: SpeedAverage::parse(cursor)
                .context("Failed to parse average_speed_value")?,
            card_number_driver_slot: FullCardNumber::parse(cursor)
                .context("Failed to parse card_number_driver_slot")?,
            similar_events_number: SimilarEventsNumber::parse(cursor)
                .context("Failed to parse similar_events_number")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuOverSpeedingEventData: appendix 2.214.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28064)
pub struct VuOverSpeedingEventData {
    pub no_of_vu_over_speeding_events: u8,
    pub vu_over_speeding_event_records: Vec<VuOverSpeedingEventRecord>,
}
impl VuOverSpeedingEventData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_vu_over_speeding_events = cursor
            .read_u8()
            .context("Failed to read no_of_vu_over_speeding_events")?;
        let mut vu_over_speeding_event_records =
            Vec::with_capacity(no_of_vu_over_speeding_events as usize);
        for _ in 0..no_of_vu_over_speeding_events {
            vu_over_speeding_event_records.push(
                VuOverSpeedingEventRecord::parse(cursor)
                    .context("Failed to parse VuOverSpeedingEventRecord")?,
            );
        }

        Ok(Self {
            no_of_vu_over_speeding_events,
            vu_over_speeding_event_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuTimeAdjustmentRecord: appendix 2.232.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28728)
pub struct VuTimeAdjustmentRecord {
    pub old_time_value: TimeReal,
    pub new_time_value: TimeReal,
    pub workshop_name: Name,
    pub workshop_address: Address,
    pub workshop_card_number: FullCardNumber,
}
impl VuTimeAdjustmentRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            old_time_value: TimeReal::parse(cursor).context("Failed to parse old_time_value")?,
            new_time_value: TimeReal::parse(cursor).context("Failed to parse new_time_value")?,
            workshop_name: Name::parse(cursor).context("Failed to parse workshop_name")?,
            workshop_address: Address::parse(cursor).context("Failed to parse workshop_address")?,
            workshop_card_number: FullCardNumber::parse(cursor)
                .context("Failed to parse workshop_card_number")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuTimeAdjustmentData: appendix 2.229.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28675)
pub struct VuTimeAdjustmentData {
    pub no_of_vu_time_adj_records: u8,
    pub vu_time_adjustment_records: Vec<VuTimeAdjustmentRecord>,
}
impl VuTimeAdjustmentData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_vu_time_adj_records = cursor
            .read_u8()
            .context("Failed to read no_of_vu_time_adj_records")?;
        let mut vu_time_adjustment_records = Vec::with_capacity(no_of_vu_time_adj_records as usize);
        for _ in 0..no_of_vu_time_adj_records {
            vu_time_adjustment_records.push(
                VuTimeAdjustmentRecord::parse(cursor)
                    .context("Failed to parse VuTimeAdjustmentRecord")?,
            );
        }

        Ok(Self {
            no_of_vu_time_adj_records,
            vu_time_adjustment_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// TREP 03 page 346
pub struct VuEventsAndFaultsBlock {
    pub vu_fault_data: VuFaultData,
    pub vu_event_data: VuEventData,
    pub vu_over_speeding_control_data: VuOverSpeedingControlData,
    pub vu_over_speeding_event_data: VuOverSpeedingEventData,
    pub vu_time_adjustment_data: VuTimeAdjustmentData,
    pub signature: Signature,
}
impl VuEventsAndFaultsBlock {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            vu_fault_data: VuFaultData::parse(cursor).context("Failed to parse vu_fault_data")?,
            vu_event_data: VuEventData::parse(cursor).context("Failed to parse vu_event_data")?,
            vu_over_speeding_control_data: VuOverSpeedingControlData::parse(cursor)
                .context("Failed to parse vu_over_speeding_control_data")?,
            vu_over_speeding_event_data: VuOverSpeedingEventData::parse(cursor)
                .context("Failed to parse vu_over_speeding_event_data")?,
            vu_time_adjustment_data: VuTimeAdjustmentData::parse(cursor)
                .context("Failed to parse vu_time_adjustment_data")?,
            signature: Signature::parse(cursor).context("Failed to parse signature")?,
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuDetailedSpeedData: appendix 2.192.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26618)
pub struct VuDetailedSpeedData {
    pub no_of_speed_blocks: u8,
    pub vu_detailed_speed_records: Vec<VuDetailedSpeedBlock>,
}
impl VuDetailedSpeedData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_speed_blocks = cursor
            .read_u8()
            .context("Failed to read no_of_speed_blocks")?;
        let mut vu_detailed_speed_records = Vec::with_capacity(no_of_speed_blocks as usize);
        for _ in 0..no_of_speed_blocks {
            vu_detailed_speed_records.push(
                VuDetailedSpeedBlock::parse(cursor)
                    .context("Failed to parse VuDetailedSpeedBlock")?,
            );
        }

        Ok(Self {
            no_of_speed_blocks,
            vu_detailed_speed_records,
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
}
impl VuIdentification {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
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
        })
    }
}

/// [VuSerialNumber: appendix 2.223.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28497)
pub type VuSerialNumber = ExtendedSerialNumber;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [SensorPaired: appendix 2.144.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e24360)
pub struct SensorPaired {
    pub sensor_serial_number: SensorSerialNumber,
    pub sensor_approval_number: SensorApprovalNumber,
    pub sensor_pairing_date_first: SensorPairingDate,
}
impl SensorPaired {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            sensor_serial_number: SensorSerialNumber::parse(cursor)
                .context("Failed to parse sensor_serial_number")?,
            sensor_approval_number: SensorApprovalNumber::parse(cursor)
                .context("Failed to parse sensor_approval_number")?,
            sensor_pairing_date_first: SensorPairingDate::parse(cursor)
                .context("Failed to parse sensor_pairing_date_first")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuCalibrationRecord: appendix 2.174.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25500)
pub struct VuCalibrationRecord {
    pub calibration_purpose: CalibrationPurpose,
    pub workshop_name: Name,
    pub workshop_address: Address,
    pub workshop_card_number: FullCardNumber,
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
}
impl VuCalibrationRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            calibration_purpose: CalibrationPurpose::parse(cursor)
                .context("Failed to parse calibration_purpose")?,
            workshop_name: Name::parse(cursor).context("Failed to parse workshop_name")?,
            workshop_address: Address::parse(cursor).context("Failed to parse workshop_address")?,
            workshop_card_number: FullCardNumber::parse(cursor)
                .context("Failed to parse workshop_card_number")?,
            workshop_card_expiry_date: TimeReal::parse(cursor)
                .context("Failed to parse workshop_card_expiry_date")
                .ok(),
            vehicle_identification_number: VehicleIdentificationNumber::parse(cursor)
                .context("Failed to parse vehicle_identification_number")
                .ok(),
            vehicle_registration_identification: VehicleRegistrationIdentification::parse(cursor)
                .context("Failed to parse vehicle_registration_identification")
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
            old_time_value: TimeReal::parse(cursor)
                .context("Failed to parse old_time_value")
                .ok(),
            new_time_value: TimeReal::parse(cursor)
                .context("Failed to parse new_time_value")
                .ok(),
            next_calibration_date: TimeReal::parse(cursor)
                .context("Failed to parse next_calibration_date")
                .ok(),
        })
    }
}

/// [VuCalibrationData: appendix 2.173.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25471)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct VuCalibrationData {
    pub no_of_vu_calibration_records: u8,
    pub vu_calibration_records: Vec<VuCalibrationRecord>,
}
impl VuCalibrationData {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let no_of_vu_calibration_records = cursor
            .read_u8()
            .context("Failed to read no_of_vu_calibration_records")?;
        let mut vu_calibration_records = Vec::with_capacity(no_of_vu_calibration_records as usize);
        for _ in 0..no_of_vu_calibration_records {
            let record = VuCalibrationRecord::parse(cursor)
                .context("Failed to parse VuCalibrationRecord")?;
            vu_calibration_records.push(record);
        }

        Ok(Self {
            no_of_vu_calibration_records,
            vu_calibration_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
/// [VuCompanyLocksBlock: appendix 2.236.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28868)
pub struct VuCompanyLocksBlock {
    pub vu_identification: VuIdentification,
    pub sensor_paired: SensorPaired,
    pub vu_calibration_data: VuCalibrationData,
    pub signature: Signature,
}
impl VuCompanyLocksBlock {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let vu_identification =
            VuIdentification::parse(cursor).context("Failed to parse vu_identification")?;
        let sensor_paired = SensorPaired::parse(cursor).context("Failed to parse sensor_paired")?;
        let vu_calibration_data =
            VuCalibrationData::parse(cursor).context("Failed to parse vu_calibration_data")?;
        let signature = Signature::parse(cursor).context("Failed to parse signature")?;
        Ok(Self {
            vu_identification,
            sensor_paired,
            vu_calibration_data,
            signature,
        })
    }
}
