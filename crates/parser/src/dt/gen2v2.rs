use std::io::BufRead;

use crate::dt::gen2;
use crate::dt::gen2::RecordArray;
use crate::dt::*;
#[cfg(feature = "ts")]
use ts_rs::TS;

use super::gen2::FullCardNumberAndGenerationGen2;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [VuOverviewBlock: appendix 2.183.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28866)
pub struct VuOverviewBlockGen2V2 {
    /// Member state certificate
    pub member_state_certificate_record_array: Vec<gen2::MemberStateCertificateGen2>,
    /// VU certificate
    pub vu_certificate_record_array: Vec<gen2::VuCertificateGen2>,
    /// Vehicle identification
    pub vehicle_identification_number_record_array: Vec<VehicleIdentificationNumber>,
    /// Vehicle registration number
    pub vehicle_registration_number_record_array: Vec<VehicleRegistrationNumberGen2V2>,
    /// VU current date and time
    pub current_date_time_record_array: Vec<TimeReal>,
    /// Downloadable period
    pub vu_downloadable_period_record_array: Vec<VuDownloadablePeriod>,
    /// Type of cards inserted in the VU
    pub card_slots_status_record_array: Vec<CardSlotsStatus>,
    /// Previous VU download
    pub vu_download_activity_data_record_array: Vec<gen2::VuDownloadActivityDataGen2>,
    /// All company locks stored.
    pub vu_company_locks_record_array: Vec<gen2::VuCompanyLocksRecordGen2>,
    /// All control records stored in the VU.
    pub vu_control_activity_record_array: Vec<gen2::VuControlActivityRecordGen2>,
    /// ECC signature of all preceding data except the certificates
    pub signature_record_array: Vec<gen2::SignatureGen2>,
}

impl VuOverviewBlockGen2V2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let member_state_certificate_record_array =
            RecordArray::parse_dyn_size(cursor, gen2::MemberStateCertificateGen2::parse_dyn_size)
                .context("Failed to parse member_state_certificate_record_array")?
                .into_inner();

        let vu_certificate_record_array =
            RecordArray::parse_dyn_size(cursor, gen2::VuCertificateGen2::parse_dyn_size)
                .context("Failed to parse vu_certificate_record_array")?
                .into_inner();

        let vehicle_identification_number_record_array =
            RecordArray::parse(cursor, VehicleIdentificationNumber::parse)
                .context("Failed to parse vehicle_identification_number_record_array")?
                .into_inner();

        let vehicle_registration_number_record_array =
            RecordArray::parse(cursor, VehicleRegistrationNumberGen2V2::parse)
                .context("Failed to parse vehicle_registration_number_record_array")?
                .into_inner();

        let current_date_time_record_array = RecordArray::parse(cursor, TimeReal::parse)
            .context("Failed to parse current_date_time_record_array")?
            .into_inner();

        let vu_downloadable_period_record_array =
            RecordArray::parse(cursor, VuDownloadablePeriod::parse)
                .context("Failed to parse vu_downloadable_period_record_array")?
                .into_inner();

        let card_slots_status_record_array = RecordArray::parse(cursor, CardSlotsStatus::parse)
            .context("Failed to parse card_slots_status_record_array")?
            .into_inner();

        let vu_download_activity_data_record_array =
            RecordArray::parse(cursor, gen2::VuDownloadActivityDataGen2::parse)
                .context("Failed to parse vu_download_activity_data_record_array")?
                .into_inner();

        let vu_company_locks_record_array =
            RecordArray::parse(cursor, gen2::VuCompanyLocksRecordGen2::parse)
                .context("Failed to parse vu_company_locks_record_array")?
                .into_inner();

        let vu_control_activity_record_array =
            RecordArray::parse(cursor, gen2::VuControlActivityRecordGen2::parse)
                .context("Failed to parse vu_control_activity_record_array")?
                .into_inner();

        let signature_record_array =
            RecordArray::parse_dyn_size(cursor, gen2::SignatureGen2::parse_dyn_size)
                .context("Failed to parse signature_record_array")?
                .into_inner();

        Ok(VuOverviewBlockGen2V2 {
            member_state_certificate_record_array,
            vu_certificate_record_array,
            vehicle_identification_number_record_array,
            vehicle_registration_number_record_array,
            current_date_time_record_array,
            vu_downloadable_period_record_array,
            card_slots_status_record_array,
            vu_download_activity_data_record_array,
            vu_company_locks_record_array,
            vu_control_activity_record_array,
            signature_record_array,
        })
    }
}

/// [LengthOfFollowingData: appendix 2.89a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22067)
type LengthOfFollowingData = u16;

/// [NoOfBorderCrossingRecords: appendix 2.101a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22475)
type NoOfBorderCrossingRecords = u16;

/// [NoOfLoadUnloadRecords: appendix 2.111a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22786)
type NoOfLoadUnloadRecords = u16;

/// [NoOfLoadTypeEntryRecords: appendix 2.112a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22833)
type NoOfLoadTypeEntryRecords = u16;

/// [VuConfigurationLengthRange: appendix 2.185a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26321)
type VuConfigurationLengthRange = u16;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [DriverCardApplicationIdentificationGen2V2: appendix 2.61a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e19892)
pub struct DriverCardApplicationIdentificationGen2V2 {
    pub length_of_following_data: LengthOfFollowingData,
    pub no_of_border_crossing_records: NoOfBorderCrossingRecords,
    pub no_of_load_unload_records: NoOfLoadUnloadRecords,
    pub no_of_load_type_entry_records: NoOfLoadTypeEntryRecords,
    pub vu_configuration_length_range: VuConfigurationLengthRange,
}
impl DriverCardApplicationIdentificationGen2V2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let length_of_following_data = cursor
            .read_u16::<BigEndian>()
            .context("Failed to parse length_of_following_data")?;
        let no_of_border_crossing_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to parse no_of_border_crossing_records")?;
        let no_of_load_unload_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to parse no_of_load_unload_records")?;
        let no_of_load_type_entry_records = cursor
            .read_u16::<BigEndian>()
            .context("Failed to parse no_of_load_type_entry_records")?;
        let vu_configuration_length_range = cursor
            .read_u16::<BigEndian>()
            .context("Failed to parse vu_configuration_length_range")?;

        Ok(DriverCardApplicationIdentificationGen2V2 {
            length_of_following_data,
            no_of_border_crossing_records,
            no_of_load_unload_records,
            no_of_load_type_entry_records,
            vu_configuration_length_range,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
/// [PositionAuthenticationStatus: appendix 2.117a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23200)
pub enum PositionAuthenticationStatus {
    NotAuthenticated,
    Authenticated,
    RFU,
}
impl PositionAuthenticationStatus {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor
            .read_u8()
            .context("Failed to parse position_authentication_status")?;
        Ok(match value {
            0x00 => PositionAuthenticationStatus::NotAuthenticated,
            0x01 => PositionAuthenticationStatus::Authenticated,
            0x02..=0xFF => PositionAuthenticationStatus::RFU,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [PlaceAuthStatusRecord: appendix 2.116b.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23087)
pub struct PlaceAuthStatusRecord {
    pub entry_time: TimeReal,
    pub authentication_status: PositionAuthenticationStatus,
}
impl PlaceAuthStatusRecord {
    pub const SIZE: usize = 5;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE).context(format!(
            "Failed to take inner cursor for {}, size: {}",
            std::any::type_name::<Self>(),
            Self::SIZE
        ))?;

        let entry_time = TimeReal::parse(inner_cursor).context("Failed to parse entry_time")?;
        let authentication_status = PositionAuthenticationStatus::parse(inner_cursor)
            .context("Failed to parse authentication_status")?;
        Ok(PlaceAuthStatusRecord {
            entry_time,
            authentication_status,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [CardPlacesAuthDailyWorkPeriod: appendix 2.26a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17697)
pub struct CardPlacesAuthDailyWorkPeriod {
    pub place_auth_pointer_newest_record: gen2::NoOfCardPlaceRecordsGen2,
    pub place_auth_status_records: Vec<PlaceAuthStatusRecord>,
}

impl CardPlacesAuthDailyWorkPeriod {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let cursor = &mut cursor.take_exact(size).context(format!(
            "Failed to take cursor for {}, size: {}",
            std::any::type_name::<Self>(),
            size
        ))?;

        let place_auth_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to parse place_auth_pointer_newest_record")?;

        let mut place_auth_status_records = Vec::new();
        let amount_of_records = (size - 2) / PlaceAuthStatusRecord::SIZE;
        for _ in 0..amount_of_records {
            if let Ok(place_auth_status_record) = PlaceAuthStatusRecord::parse(cursor) {
                place_auth_status_records.push(place_auth_status_record);
            } else {
                break;
            }
        }

        Ok(CardPlacesAuthDailyWorkPeriod {
            place_auth_pointer_newest_record,
            place_auth_status_records,
        })
    }
}

/// [NoOfGNSSAdRecords: appendix 2.111.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22756)
pub type NoOfGNSSAdRecords = u16;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [GNSSAuthStatusADRecord: appendix 2.79a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21683)
pub struct GNSSAuthStatusADRecord {
    pub time_stamp: TimeReal,
    pub authentication_status: PositionAuthenticationStatus,
}
impl GNSSAuthStatusADRecord {
    pub const SIZE: usize = 5;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let inner_cursor = &mut cursor.take_exact(Self::SIZE).context(format!(
            "Failed to take inner cursor for {}, size: {}",
            std::any::type_name::<Self>(),
            Self::SIZE
        ))?;

        let time_stamp = TimeReal::parse(inner_cursor).context("Failed to parse time_stamp")?;
        let authentication_status = PositionAuthenticationStatus::parse(inner_cursor)
            .context("Failed to parse authentication_status")?;

        Ok(GNSSAuthStatusADRecord {
            time_stamp,
            authentication_status,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [GNSSAuthAccumulatedDriving: appendix 2.79a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21683)
pub struct GNSSAuthAccumulatedDriving {
    pub gnss_auth_ad_pointer_newest_record: NoOfGNSSAdRecords,
    pub gnss_auth_status_ad_records: Vec<GNSSAuthStatusADRecord>,
}

impl GNSSAuthAccumulatedDriving {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let cursor = &mut cursor.take_exact(size).context(format!(
            "Failed to take cursor for {}, size: {}",
            std::any::type_name::<Self>(),
            size
        ))?;

        let gnss_auth_ad_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to parse gnss_auth_ad_pointer_newest_record")?;

        // 2 bytes for the pointer size
        let no_of_records = (size - 2) / GNSSAuthStatusADRecord::SIZE;

        let mut gnss_auth_status_ad_records = Vec::new();
        for _ in 0..no_of_records {
            if let Ok(gnss_auth_status_ad_record) = GNSSAuthStatusADRecord::parse(cursor) {
                gnss_auth_status_ad_records.push(gnss_auth_status_ad_record);
            } else {
                break;
            }
        }

        Ok(GNSSAuthAccumulatedDriving {
            gnss_auth_ad_pointer_newest_record,
            gnss_auth_status_ad_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [GNSSPlaceAuthRecord: appendix 2.79c.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e21739)
pub struct GNSSPlaceAuthRecord {
    pub time_stamp: TimeReal,
    pub gnss_accuracy: gen2::GnssAccuracyGen2,
    pub gnss_coordinates: gen2::GeoCoordinatesGen2,
    pub authentication_status: PositionAuthenticationStatus,
}
impl GNSSPlaceAuthRecord {
    pub const SIZE: usize = 12;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_stamp = TimeReal::parse(cursor).context("Failed to parse time_stamp")?;
        let gnss_accuracy =
            gen2::GnssAccuracyGen2::parse(cursor).context("Failed to parse gnss_accuracy")?;
        let gnss_coordinates =
            gen2::GeoCoordinatesGen2::parse(cursor).context("Failed to parse gnss_coordinates")?;
        let authentication_status = PositionAuthenticationStatus::parse(cursor)
            .context("Failed to parse authentication_status")?;

        Ok(GNSSPlaceAuthRecord {
            time_stamp,
            gnss_accuracy,
            gnss_coordinates,
            authentication_status,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [CardBorderCrossingRecord: appendix 2.11b.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16857)
pub struct CardBorderCrossingRecord {
    /// `countryLeft` is the country which was left by the vehicle, or `'no information available'` according to Annex IC requirement 147b.
    /// `'Rest of the World'` (`NationNumeric` code `'FF'H`) shall be used when the vehicle unit is not able to determine the country where
    /// the vehicle is located (e.g. the current country is not part of the stored digital maps).
    pub country_left: external::NationNumeric,
    /// `countryEntered` is the country into which the vehicle has entered, or the country in which the vehicle is located at card insertion time.
    /// `'Rest of the World'` (`NationNumeric` code `'FF'H`) shall be used when the vehicle unit is not able to determine the country where
    /// the vehicle is located (e.g. the current country is not part of the stored digital maps).
    pub country_entered: external::NationNumeric,
    pub gnss_place_auth_record: GNSSPlaceAuthRecord,
    pub vehicle_odometer_value: OdometerShort,
}
impl CardBorderCrossingRecord {
    pub const SIZE: usize = 17;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let cursor = &mut cursor.take_exact(Self::SIZE).context(format!(
            "Failed to take cursor for {}, size: {}",
            std::any::type_name::<Self>(),
            Self::SIZE
        ))?;

        let country_left =
            external::NationNumeric::parse(cursor).context("Failed to parse country_left")?;
        let country_entered =
            external::NationNumeric::parse(cursor).context("Failed to parse country_entered")?;
        let gnss_place_auth_record =
            GNSSPlaceAuthRecord::parse(cursor).context("Failed to parse gnss_place_auth_record")?;
        let vehicle_odometer_value =
            OdometerShort::parse(cursor).context("Failed to parse vehicle_odometer_value")?;

        Ok(CardBorderCrossingRecord {
            country_left,
            country_entered,
            gnss_place_auth_record,
            vehicle_odometer_value,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [CardBorderCrossings: appendix 2.11a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e16826)
pub struct CardBorderCrossings {
    pub border_crossing_pointer_newest_record: NoOfBorderCrossingRecords,
    pub card_border_crossing_records: Vec<CardBorderCrossingRecord>,
}
impl CardBorderCrossings {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let cursor = &mut cursor.take_exact(size).context(format!(
            "Failed to take cursor for {}, size: {}",
            std::any::type_name::<Self>(),
            size
        ))?;

        let border_crossing_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to parse border_crossing_pointer_newest_record")?;

        let no_of_records = (size - 2) / CardBorderCrossingRecord::SIZE;
        let mut card_border_crossing_records = Vec::new();
        for _ in 0..no_of_records {
            if let Ok(card_border_crossing_record) = CardBorderCrossingRecord::parse(cursor) {
                card_border_crossing_records.push(card_border_crossing_record);
            } else {
                break;
            }
        }

        Ok(CardBorderCrossings {
            border_crossing_pointer_newest_record,
            card_border_crossing_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
/// [OperationType: appendix 2.114a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22905)
pub enum OperationType {
    RFU,
    LoadOperation,
    UnloadOperation,
    SimultaneousLoadUnloadOperation,
}
impl OperationType {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor.read_u8().context("Failed to parse operation_type")?;
        Ok(match value {
            0x00 => OperationType::RFU,
            0x01 => OperationType::LoadOperation,
            0x02 => OperationType::UnloadOperation,
            0x03 => OperationType::SimultaneousLoadUnloadOperation,
            0x04..=0xFF => OperationType::RFU,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [CardLoadUnloadRecord: appendix 2.24d.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17576)
pub struct CardLoadUnloadRecord {
    pub time_stamp: TimeReal,
    pub operation_type: OperationType,
    pub gnss_place_auth_record: GNSSPlaceAuthRecord,
    pub vehicle_odometer_value: OdometerShort,
}
impl CardLoadUnloadRecord {
    pub const SIZE: usize = 20;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_stamp = TimeReal::parse(cursor).context("Failed to parse time_stamp")?;
        let operation_type =
            OperationType::parse(cursor).context("Failed to parse operation_type")?;
        let gnss_place_auth_record =
            GNSSPlaceAuthRecord::parse(cursor).context("Failed to parse gnss_place_auth_record")?;
        let vehicle_odometer_value =
            OdometerShort::parse(cursor).context("Failed to parse vehicle_odometer_value")?;

        Ok(CardLoadUnloadRecord {
            time_stamp,
            operation_type,
            gnss_place_auth_record,
            vehicle_odometer_value,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [CardLoadUnloadOperations: appendix 2.24c.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17544)
pub struct CardLoadUnloadOperations {
    pub load_unload_pointer_newest_record: NoOfLoadUnloadRecords,
    pub card_load_unload_records: Vec<CardLoadUnloadRecord>,
}
impl CardLoadUnloadOperations {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let cursor = &mut cursor.take_exact(size).context(format!(
            "Failed to take cursor for {}, size: {}",
            std::any::type_name::<Self>(),
            size
        ))?;

        let load_unload_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to parse load_unload_pointer_newest_record")?;

        let no_of_records = (size - 2) / CardLoadUnloadRecord::SIZE;
        let mut card_load_unload_records = Vec::new();
        for _ in 0..no_of_records {
            if let Ok(card_load_unload_record) = CardLoadUnloadRecord::parse(cursor) {
                card_load_unload_records.push(card_load_unload_record);
            } else {
                break;
            }
        }

        Ok(CardLoadUnloadOperations {
            load_unload_pointer_newest_record,
            card_load_unload_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
/// [LoadType: appendix 2.90a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e22110)
pub enum LoadType {
    UndefinedLoadType,
    Goods,
    Passengers,
    RFU,
}
impl LoadType {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value = cursor.read_u8().context("Failed to parse load_type")?;
        Ok(match value {
            0x00 => LoadType::UndefinedLoadType,
            0x01 => LoadType::Goods,
            0x02 => LoadType::Passengers,
            0x03..=0xFF => LoadType::RFU,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [CardLoadTypeEntryRecord: appendix 2.24b.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17521)
pub struct CardLoadTypeEntryRecord {
    pub time_stamp: TimeReal,
    pub load_type_entered: LoadType,
}
impl CardLoadTypeEntryRecord {
    pub const SIZE: usize = 5;
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_stamp = TimeReal::parse(cursor).context("Failed to parse time_stamp")?;
        let load_type_entered =
            LoadType::parse(cursor).context("Failed to parse load_type_entered")?;

        Ok(CardLoadTypeEntryRecord {
            time_stamp,
            load_type_entered,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [CardLoadTypeEntries: appendix 2.24a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e17490)
pub struct CardLoadTypeEntries {
    pub load_type_pointer_newest_record: NoOfLoadTypeEntryRecords,
    pub card_load_type_entry_records: Vec<CardLoadTypeEntryRecord>,
}
impl CardLoadTypeEntries {
    pub fn parse_dyn_size(cursor: &mut Cursor<&[u8]>, size: usize) -> Result<Self> {
        let cursor = &mut cursor.take_exact(size).context(format!(
            "Failed to take cursor for {}, size: {}",
            std::any::type_name::<Self>(),
            size
        ))?;

        let load_type_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to parse load_type_pointer_newest_record")?;

        let no_of_records = (size - 2) / CardLoadTypeEntryRecord::SIZE;
        let mut card_load_type_entry_records = Vec::new();
        for _ in 0..no_of_records {
            if let Ok(card_load_type_entry_record) = CardLoadTypeEntryRecord::parse(cursor) {
                card_load_type_entry_records.push(card_load_type_entry_record);
            } else {
                break;
            }
        }

        Ok(CardLoadTypeEntries {
            load_type_pointer_newest_record,
            card_load_type_entry_records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
/// VuConfigurations: there is no documentation for this block in the spec, none at all, so we'll just process the bytes
pub struct VuConfigurations(pub Vec<u8>);
impl VuConfigurations {
    const SIZE: usize = 3072; // fixed size
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut buf = [0u8; Self::SIZE];
        cursor
            .read_exact(&mut buf)
            .context("Failed to parse VuConfigurations")?;
        Ok(VuConfigurations(buf.to_vec()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// [VehicleRegistrationNumber: appendix 2.168.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25188)
#[cfg_attr(feature = "ts", derive(TS))]
pub struct VehicleRegistrationNumberGen2V2 {
    pub code_page: u8,
    pub vehicle_reg_number: IA5String,
}
impl VehicleRegistrationNumberGen2V2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let code_page = cursor.read_u8().context("Failed to read code page")?;
        // Vu Gen2v2 uses 14 bytes for vehicle registration number, even though the spec says 13 ¯\_(ツ)_/¯
        let vehicle_reg_number = IA5String::parse_with_code_page(cursor, 14, code_page)
            .context("Failed to parse VehicleRegistrationNumberGen2V2")?;
        Ok(VehicleRegistrationNumberGen2V2 {
            code_page,
            vehicle_reg_number,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
#[serde(rename_all = "camelCase")]
pub struct VuActivitiesBlockGen2V2 {
    /// Date of day downloaded
    pub date_of_day_downloaded_record_array: Vec<gen2::DateOfDayDownloadedGen2>,
    /// Odometer at end of downloaded day
    pub odometer_value_midnight_record_array: Vec<OdometerValueMidnight>,
    /// Cards insertion withdrawal cycles data. If no data available, array has noOfRecords = 0.
    /// When a record crosses 00:00 (insertion on previous day) or 24:00 (withdrawal next day),
    /// it appears in full within both days involved.
    pub vu_card_iw_record_array: Vec<gen2::VuCardIwRecordGen2>,
    /// Slots status at 00:00 and activity changes recorded for the day downloaded
    pub vu_activity_daily_record_array: Vec<CardActivityChangeInfo>,
    /// Places related data recorded for the day downloaded.
    pub vu_place_daily_work_period_record_array: Vec<VuPlaceDailyWorkPeriodRecordGen2V2>,
    /// GNSS positions when accumulated driving time reaches multiple of 3 hours.
    pub vu_gnss_ad_record_array: Vec<VuGNSSADRecordGen2V2>,
    /// Specific conditions data recorded for the day downloaded.
    pub vu_specific_condition_record_array: Vec<gen2::SpecificConditionRecordGen2>,
    /// Border crossings for the day downloaded.
    pub vu_border_crossing_record_array: Vec<VuBorderCrossingRecord>,
    /// Load/unload operations for the day downloaded.
    pub vu_load_unload_record_array: Vec<VuLoadUnloadRecord>,
    /// ECC signature of all preceding data
    pub signature_record_array: Vec<gen2::SignatureGen2>,
}

impl VuActivitiesBlockGen2V2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let date_of_day_downloaded_record_array =
            RecordArray::parse(cursor, gen2::DateOfDayDownloadedGen2::parse)
                .context("Failed to parse date_of_day_downloaded_record_array")?
                .into_inner();

        let odometer_value_midnight_record_array =
            RecordArray::parse(cursor, OdometerValueMidnight::parse)
                .context("Failed to parse odometer_value_midnight_record_array")?
                .into_inner();

        let vu_card_iw_record_array = RecordArray::parse(cursor, gen2::VuCardIwRecordGen2::parse)
            .context("Failed to parse vu_card_iw_record_array")?
            .into_inner();

        let vu_activity_daily_record_array =
            RecordArray::parse(cursor, CardActivityChangeInfo::parse)
                .context("Failed to parse vu_activity_daily_record_array")?
                .into_inner();

        let vu_place_daily_work_period_record_array =
            RecordArray::parse(cursor, VuPlaceDailyWorkPeriodRecordGen2V2::parse)
                .context("Failed to parse vu_place_daily_work_period_record_array")?
                .into_inner();

        let vu_gnss_ad_record_array = RecordArray::parse(cursor, VuGNSSADRecordGen2V2::parse)
            .context("Failed to parse vu_gnss_ad_record_array")?
            .into_inner();

        let vu_specific_condition_record_array = RecordArray::parse(cursor, |cursor| {
            gen2::SpecificConditionRecordGen2::parse(cursor)
        })
        .context("Failed to parse vu_specific_condition_record_array")?
        .into_inner();

        let vu_border_crossing_record_array =
            RecordArray::parse(cursor, VuBorderCrossingRecord::parse)
                .context("Failed to parse vu_border_crossing_record_array")?
                .into_inner();

        let vu_load_unload_record_array = RecordArray::parse(cursor, VuLoadUnloadRecord::parse)
            .context("Failed to parse vu_load_unload_record_array")?
            .into_inner();

        let signature_record_array =
            RecordArray::parse_dyn_size(cursor, gen2::SignatureGen2::parse_dyn_size)
                .context("Failed to parse signature_record_array")?
                .into_inner();

        Ok(VuActivitiesBlockGen2V2 {
            date_of_day_downloaded_record_array,
            odometer_value_midnight_record_array,
            vu_card_iw_record_array,
            vu_activity_daily_record_array,
            vu_place_daily_work_period_record_array,
            vu_gnss_ad_record_array,
            vu_specific_condition_record_array,
            vu_border_crossing_record_array,
            vu_load_unload_record_array,
            signature_record_array,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
#[serde(rename_all = "camelCase")]
/// [PlaceAuthRecord: appendix 2.216a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e23047)
pub struct PlaceAuthRecord {
    pub entry_time: TimeReal,
    pub entry_type_daily_work_period: gen2::EntryTypeDailyWorkPeriodGen2,
    pub daily_work_period_country: external::NationNumeric,
    pub daily_work_period_region: external::RegionNumeric,
    pub vehicle_odometer_value: OdometerShort,
    pub entry_gnss_place_auth_record: GNSSPlaceAuthRecord,
}

impl PlaceAuthRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let entry_time = TimeReal::parse(cursor).context("Failed to parse entry_time")?;
        let entry_type_daily_work_period = gen2::EntryTypeDailyWorkPeriodGen2::parse(cursor)
            .context("Failed to parse entry_type_daily_work_period")?;
        let daily_work_period_country = external::NationNumeric::parse(cursor)
            .context("Failed to parse daily_work_period_country")?;
        let daily_work_period_region = external::RegionNumeric::parse(cursor)
            .context("Failed to parse daily_work_period_region")?;
        let vehicle_odometer_value =
            OdometerShort::parse(cursor).context("Failed to parse vehicle_odometer_value")?;
        let entry_gnss_place_auth_record = GNSSPlaceAuthRecord::parse(cursor)
            .context("Failed to parse entry_gnss_place_auth_record")?;

        Ok(PlaceAuthRecord {
            entry_time,
            entry_type_daily_work_period,
            daily_work_period_country,
            daily_work_period_region,
            vehicle_odometer_value,
            entry_gnss_place_auth_record,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
#[serde(rename_all = "camelCase")]
/// [VuPlaceDailyWorkPeriod: appendix 2.219.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e28313)
pub struct VuPlaceDailyWorkPeriodRecordGen2V2 {
    pub full_card_number_and_generation: Option<gen2::FullCardNumberAndGenerationGen2>,
    pub place_record: PlaceAuthRecord,
}

impl VuPlaceDailyWorkPeriodRecordGen2V2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuPlaceDailyWorkPeriodRecordGen2V2 {
            full_card_number_and_generation: gen2::FullCardNumberAndGenerationGen2::parse(cursor),
            place_record: PlaceAuthRecord::parse(cursor).context("Failed to parse place_record")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
#[serde(rename_all = "camelCase")]
/// [VuGNSSADRecord: appendix 2.203.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27345)
pub struct VuGNSSADRecordGen2V2 {
    pub timestamp: TimeReal,
    pub card_number_and_gen_driver_slot: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot: Option<FullCardNumberAndGenerationGen2>,
    pub gnss_place_auth_record: GNSSPlaceAuthRecord,
    pub vehicle_odometer_value: OdometerShort,
}
impl VuGNSSADRecordGen2V2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let timestamp = TimeReal::parse(cursor).context("Failed to parse timestamp")?;
        let card_number_and_gen_driver_slot = FullCardNumberAndGenerationGen2::parse(cursor)
            .context("Failed to parse card_number_and_gen_driver_slot")
            .ok();
        let card_number_and_gen_codriver_slot = FullCardNumberAndGenerationGen2::parse(cursor)
            .context("Failed to parse card_number_and_gen_codriver_slot")
            .ok();
        let gnss_place_auth_record =
            GNSSPlaceAuthRecord::parse(cursor).context("Failed to parse gnss_place_auth_record")?;
        let vehicle_odometer_value =
            OdometerShort::parse(cursor).context("Failed to parse vehicle_odometer_value")?;

        Ok(VuGNSSADRecordGen2V2 {
            timestamp,
            card_number_and_gen_driver_slot,
            card_number_and_gen_codriver_slot,
            gnss_place_auth_record,
            vehicle_odometer_value,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
#[serde(rename_all = "camelCase")]
/// [VuBorderCrossingRecord: appendix 2.203a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27413)

pub struct VuBorderCrossingRecord {
    pub card_number_and_gen_driver_slot: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot: Option<FullCardNumberAndGenerationGen2>,
    pub country_left: external::NationNumeric,
    pub country_entered: external::NationNumeric,
    pub gnss_place_auth_record: GNSSPlaceAuthRecord,
    pub vehicle_odometer_value: OdometerShort,
}
impl VuBorderCrossingRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let card_number_and_gen_driver_slot = FullCardNumberAndGenerationGen2::parse(cursor)
            .context("Failed to parse card_number_and_gen_driver_slot")
            .ok();
        let card_number_and_gen_codriver_slot = FullCardNumberAndGenerationGen2::parse(cursor)
            .context("Failed to parse card_number_and_gen_codriver_slot")
            .ok();
        let country_left =
            external::NationNumeric::parse(cursor).context("Failed to parse country_left")?;
        let country_entered =
            external::NationNumeric::parse(cursor).context("Failed to parse country_entered")?;
        let gnss_place_auth_record =
            GNSSPlaceAuthRecord::parse(cursor).context("Failed to parse gnss_place_auth_record")?;
        let vehicle_odometer_value =
            OdometerShort::parse(cursor).context("Failed to parse vehicle_odometer_value")?;

        Ok(VuBorderCrossingRecord {
            card_number_and_gen_driver_slot,
            card_number_and_gen_codriver_slot,
            country_left,
            country_entered,
            gnss_place_auth_record,
            vehicle_odometer_value,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
#[serde(rename_all = "camelCase")]
/// [VuLoadUnloadRecord: appendix 2.208a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27834)
pub struct VuLoadUnloadRecord {
    pub time_stamp: TimeReal,
    pub operation_type: OperationType,
    pub card_number_and_gen_driver_slot: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot: Option<FullCardNumberAndGenerationGen2>,
    pub gnss_place_auth_record: GNSSPlaceAuthRecord,
    pub vehicle_odometer_value: OdometerShort,
}
impl VuLoadUnloadRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let time_stamp = TimeReal::parse(cursor).context("Failed to parse time_stamp")?;
        let operation_type =
            OperationType::parse(cursor).context("Failed to parse operation_type")?;
        let card_number_and_gen_driver_slot = FullCardNumberAndGenerationGen2::parse(cursor)
            .context("Failed to parse card_number_and_gen_driver_slot")
            .ok();
        let card_number_and_gen_codriver_slot = FullCardNumberAndGenerationGen2::parse(cursor)
            .context("Failed to parse card_number_and_gen_codriver_slot")
            .ok();
        let gnss_place_auth_record =
            GNSSPlaceAuthRecord::parse(cursor).context("Failed to parse gnss_place_auth_record")?;
        let vehicle_odometer_value =
            OdometerShort::parse(cursor).context("Failed to parse vehicle_odometer_value")?;

        Ok(VuLoadUnloadRecord {
            time_stamp,
            operation_type,
            card_number_and_gen_driver_slot,
            card_number_and_gen_codriver_slot,
            gnss_place_auth_record,
            vehicle_odometer_value,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
/// [VuDigitalMapVersion: appendix 2.192a.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e26652)
pub struct VuDigitalMapVersion(pub IA5String);

impl VuDigitalMapVersion {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let value =
            IA5String::parse_dyn_size(cursor, 12).context("Failed to parse VuDigitalMapVersion")?;
        Ok(VuDigitalMapVersion(value))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]

pub struct VuIdentificationGen2V2 {
    pub vu_manufacturer_name: VuManufacturerName,
    pub vu_manufacturer_address: VuManufacturerAddress,
    pub vu_part_number: VuPartNumber,
    pub vu_serial_number: gen2::VuSerialNumberGen2,
    pub vu_software_identification: VuSoftwareIdentification,
    pub vu_manufacturing_date: VuManufacturingDate,
    pub vu_approval_number: gen2::VuApprovalNumberGen2,
    pub vu_generation: gen2::GenerationGen2,
    pub vu_ability: gen2::VuAbilityGen2,
    pub vu_digital_map_version: VuDigitalMapVersion, // Only in Gen2V2, but for some reason it's categorized as "Generation 2" unlike other types, EU please.
}
/// [VuIdentification: appendix 2.205.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e27574)
impl VuIdentificationGen2V2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let vu_manufacturer_name =
            VuManufacturerName::parse(cursor).context("Failed to parse vu_manufacturer_name")?;
        let vu_manufacturer_address = VuManufacturerAddress::parse(cursor)
            .context("Failed to parse vu_manufacturer_address")?;
        let vu_part_number =
            VuPartNumber::parse(cursor).context("Failed to parse vu_part_number")?;
        let vu_serial_number =
            gen2::VuSerialNumberGen2::parse(cursor).context("Failed to parse vu_serial_number")?;
        let vu_software_identification = VuSoftwareIdentification::parse(cursor)
            .context("Failed to parse vu_software_identification")?;
        let vu_manufacturing_date =
            VuManufacturingDate::parse(cursor).context("Failed to parse vu_manufacturing_date")?;
        let vu_approval_number = gen2::VuApprovalNumberGen2::parse(cursor)
            .context("Failed to parse vu_approval_number")?;
        let vu_generation =
            gen2::GenerationGen2::parse(cursor).context("Failed to parse vu_generation")?;
        let vu_ability =
            gen2::VuAbilityGen2::parse(cursor).context("Failed to parse vu_ability")?;
        let vu_digital_map_version =
            VuDigitalMapVersion::parse(cursor).context("Failed to parse vu_digital_map_version")?;

        Ok(VuIdentificationGen2V2 {
            vu_manufacturer_name,
            vu_manufacturer_address,
            vu_part_number,
            vu_serial_number,
            vu_software_identification,
            vu_manufacturing_date,
            vu_approval_number,
            vu_generation,
            vu_ability,
            vu_digital_map_version,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// TREP 0x35 page 349
pub struct VuCompanyLocksGen2V2 {
    /// All VU identification data stored in the VU
    pub vu_identification_record_array: Vec<VuIdentificationGen2V2>,
    /// All MS pairings stored in the VU
    pub vu_sensor_paired_record_array: Vec<gen2::SensorPairedRecordGen2>,
    /// All external GNSS facility couplings stored in the VU
    pub vu_sensor_external_gnss_coupled_record_array:
        Vec<gen2::SensorExternalGNSSCoupledRecordGen2>,
    /// All calibration records stored in the VU
    pub vu_calibration_record_array: Vec<VuCalibrationRecordGen2V2>,
    /// All card insertion data stored in the VU
    pub vu_card_record_array: Vec<gen2::VuCardRecordGen2>,
    /// All ITS consent records stored in the VU
    pub vu_its_consent_record_array: Vec<gen2::VuITSConsentRecordGen2>,
    /// All power supply interruption records stored in the VU
    pub vu_power_supply_interruption_record_array:
        Vec<gen2v2::VuPowerSupplyInterruptionRecordGen2V2>,
    // /// ECC signature of all preceding data
    pub signature_record_array: Vec<gen2::SignatureGen2>,
}
impl VuCompanyLocksGen2V2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let vu_identification_record_array =
            RecordArray::parse(cursor, VuIdentificationGen2V2::parse)
                .context("Failed to parse vu_identification_record_array")?
                .into_inner();

        let vu_sensor_paired_record_array =
            RecordArray::parse(cursor, gen2::SensorPairedRecordGen2::parse)
                .context("Failed to parse vu_sensor_paired_record_array")?
                .into_inner();

        let vu_sensor_external_gnss_coupled_record_array =
            RecordArray::parse(cursor, gen2::SensorExternalGNSSCoupledRecordGen2::parse)
                .context("Failed to parse vu_sensor_external_gnss_coupled_record_array")?
                .into_inner();

        let vu_calibration_record_array =
            RecordArray::parse(cursor, VuCalibrationRecordGen2V2::parse)
                .context("Failed to parse vu_calibration_record_array")?
                .into_inner();

        let vu_card_record_array = RecordArray::parse(cursor, gen2::VuCardRecordGen2::parse)
            .context("Failed to parse vu_card_record_array")?
            .into_inner();

        let vu_its_consent_record_array =
            RecordArray::parse(cursor, gen2::VuITSConsentRecordGen2::parse)
                .context("Failed to parse vu_its_consent_record_array")?
                .into_inner();

        let vu_power_supply_interruption_record_array =
            RecordArray::parse(cursor, gen2v2::VuPowerSupplyInterruptionRecordGen2V2::parse)
                .context("Failed to parse vu_power_supply_interruption_record_array")?
                .into_inner();

        let signature_record_array =
            RecordArray::parse_dyn_size(cursor, gen2::SignatureGen2::parse_dyn_size)
                .context("Failed to parse signature_record_array")?
                .into_inner();

        Ok(VuCompanyLocksGen2V2 {
            vu_identification_record_array,
            vu_sensor_paired_record_array,
            vu_sensor_external_gnss_coupled_record_array,
            vu_calibration_record_array,
            vu_card_record_array,
            vu_its_consent_record_array,
            vu_power_supply_interruption_record_array,
            signature_record_array,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
/// [VuCalibrationRecord: appendix 2.174.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e25506)
pub struct VuCalibrationRecordGen2V2 {
    pub calibration_purpose: gen2::CalibrationPurposeGen2,
    pub workshop_name: Name,
    pub workshop_address: Address,
    pub workshop_card_number: gen2::FullCardNumberGen2,
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
    pub seal_data_vu: gen2::SealDataVuGen2,
    pub by_default_load_type: LoadType,
    pub calibration_country: external::NationNumeric,
    pub calibration_country_timestamp: TimeReal,
}

impl VuCalibrationRecordGen2V2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let calibration_purpose = gen2::CalibrationPurposeGen2::parse(cursor)
            .context("Failed to parse calibration_purpose")?;
        let workshop_name = Name::parse(cursor).context("Failed to parse workshop_name")?;
        let workshop_address =
            Address::parse(cursor).context("Failed to parse workshop_address")?;
        let workshop_card_number = gen2::FullCardNumberGen2::parse(cursor)
            .context("Failed to parse workshop_card_number")?;
        let workshop_card_expiry_date = TimeReal::parse(cursor).ok();
        let vehicle_identification_number = VehicleIdentificationNumber::parse(cursor).ok();
        let vehicle_registration_identification =
            VehicleRegistrationIdentification::parse(cursor).ok();
        let w_vehicle_characteristic_constant = WVehicleCharacteristicConstant::parse(cursor)
            .context("Failed to parse w_vehicle_characteristic_constant")?;
        let k_constant_of_recording_equipment = KConstantOfRecordingEquipment::parse(cursor)
            .context("Failed to parse k_constant_of_recording_equipment")?;
        let l_tyre_circumference =
            LTyreCircumference::parse(cursor).context("Failed to parse l_tyre_circumference")?;
        let tyre_size = TyreSize::parse(cursor).context("Failed to parse tyre_size")?;
        let authorised_speed =
            SpeedAuthorised::parse(cursor).context("Failed to parse authorised_speed")?;
        let old_odometer_value =
            OdometerShort::parse(cursor).context("Failed to parse old_odometer_value")?;
        let new_odometer_value =
            OdometerShort::parse(cursor).context("Failed to parse new_odometer_value")?;
        let old_time_value = TimeReal::parse(cursor).ok();
        let new_time_value = TimeReal::parse(cursor).ok();
        let next_calibration_date = TimeReal::parse(cursor).ok();
        let seal_data_vu =
            gen2::SealDataVuGen2::parse(cursor).context("Failed to parse seal_data_vu")?;
        cursor.consume(24);
        let by_default_load_type =
            LoadType::parse(cursor).context("Failed to parse by_default_load_type")?;
        let calibration_country = external::NationNumeric::parse(cursor)
            .context("Failed to parse calibration_country")?;
        let calibration_country_timestamp =
            TimeReal::parse(cursor).context("Failed to parse calibration_country_timestamp")?;

        Ok(VuCalibrationRecordGen2V2 {
            calibration_purpose,
            workshop_name,
            workshop_address,
            workshop_card_number,
            workshop_card_expiry_date,
            vehicle_identification_number,
            vehicle_registration_identification,
            w_vehicle_characteristic_constant,
            k_constant_of_recording_equipment,
            l_tyre_circumference,
            tyre_size,
            authorised_speed,
            old_odometer_value,
            new_odometer_value,
            old_time_value,
            new_time_value,
            next_calibration_date,
            seal_data_vu,
            by_default_load_type,
            calibration_country,
            calibration_country_timestamp,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS))]
/// [EventFaultType: appendix 2.70.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e20338)
pub enum EventFaultTypeGen2V2 {
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
    GNSSAnomaly,
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
    InconsistencyBetweenMotionDataAndStoredDriverActivityData,
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
    InternalSensorFault,
    CardFaultNoFurtherDetails,
    RFU,
    ManufacturerSpecific,
}

impl EventFaultTypeGen2V2 {
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
            0x0F => Ok(Self::GNSSAnomaly),

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
            0x1C => Ok(Self::InconsistencyBetweenMotionDataAndStoredDriverActivityData),
            0x1D..=0x1F => Ok(Self::RFU),

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
            0x3A => Ok(Self::InternalSensorFault),
            0x3B..=0x3F => Ok(Self::RFU),

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
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]

/// [VuPowerSupplyInterruptionRecord: appendix 2.240.](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:02016R0799-20230821#cons_toc_d1e29420)
pub struct VuPowerSupplyInterruptionRecordGen2V2 {
    pub event_type: EventFaultTypeGen2V2,
    pub event_record_purpose: EventFaultRecordPurpose,
    pub event_begin_time: TimeReal,
    pub event_end_time: TimeReal,
    pub card_number_and_gen_driver_slot_begin: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_driver_slot_end: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot_begin: Option<FullCardNumberAndGenerationGen2>,
    pub card_number_and_gen_codriver_slot_end: Option<FullCardNumberAndGenerationGen2>,
    pub similar_events_number: SimilarEventsNumber,
}

impl VuPowerSupplyInterruptionRecordGen2V2 {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(VuPowerSupplyInterruptionRecordGen2V2 {
            event_type: EventFaultTypeGen2V2::parse(cursor)
                .context("Failed to parse event_type")?,
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
