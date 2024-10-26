use crate::dt::gen2;
use crate::dt::gen2::RecordArray;
use crate::dt::*;
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct VuOverviewBlock {
    pub member_state_certificate_record_array: Vec<gen2::MemberStateCertificateGen2>,
    pub vu_certificate_record_array: Vec<gen2::VuCertificateGen2>,
    pub vehicle_identification_number_record_array: Vec<VehicleIdentificationNumber>,
    pub vehicle_registration_number_record_array: Vec<VehicleRegistrationNumber>,
}

impl VuOverviewBlock {
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
            RecordArray::parse(cursor, VehicleRegistrationNumber::parse)
                .context("Failed to parse vehicle_registration_number_record_array")?
                .into_inner();

        Ok(VuOverviewBlock {
            member_state_certificate_record_array,
            vu_certificate_record_array,
            vehicle_identification_number_record_array,
            vehicle_registration_number_record_array,
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
        let inner_cursor = &mut cursor.take_exact(Self::SIZE);

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
        let cursor = &mut cursor.take_exact(size);

        let place_auth_pointer_newest_record = cursor
            .read_u16::<BigEndian>()
            .context("Failed to parse place_auth_pointer_newest_record")?;

        let mut place_auth_status_records = Vec::new();
        let amount_of_records = size / PlaceAuthStatusRecord::SIZE;
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
