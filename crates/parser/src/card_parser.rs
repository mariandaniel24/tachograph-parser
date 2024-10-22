use crate::dt::gen1;
use crate::dt::gen2;
use crate::dt::{self};
use anyhow::{Context, Result};
use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Cursor, Read};
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct CardGen1Blocks {
    pub card_icc_identification: gen1::CardIccIdentification,
    pub card_chip_identification: dt::CardChipIdentification,
    pub application_identification: gen1::DriverCardApplicationIdentification,
    pub application_identification_signature: gen1::Signature,
    pub card_certificate: gen1::Certificate,
    pub member_state_certificate: gen1::Certificate,
    pub identification: dt::Identification,
    pub identification_signature: gen1::Signature,
    pub card_download: Option<dt::CardDownload>,
    pub card_download_signature: Option<gen1::Signature>,
    pub driver_licence_info: Option<dt::CardDrivingLicenceInformation>,
    pub driver_licence_info_signature: Option<gen1::Signature>,
    pub events_data: gen1::CardEventData,
    pub events_data_signature: gen1::Signature,
    pub faults_data: gen1::CardFaultData,
    pub faults_data_signature: gen1::Signature,
    pub driver_activity_data: dt::DriverActivityData,
    pub driver_activity_data_signature: gen1::Signature,
    pub vehicles_used: gen1::CardVehiclesUsed,
    pub vehicles_used_signature: gen1::Signature,
    pub places: gen1::CardPlaceDailyWorkPeriod,
    pub places_signature: gen1::Signature,
    pub current_usage: Option<dt::CurrentUsage>,
    pub current_usage_signature: Option<gen1::Signature>,
    pub control_activity_data: gen1::CardControlActivityDataRecord,
    pub control_activity_data_signature: gen1::Signature,
    pub specific_conditions: gen1::SpecificConditions,
    pub specific_conditions_signature: gen1::Signature,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct CardGen2Blocks {
    pub card_icc_identification: gen2::CardIccIdentificationGen2,
    pub card_chip_identification: dt::CardChipIdentification,
    pub application_identification: gen2::DriverCardApplicationIdentificationGen2,
    pub application_identification_signature: gen2::SignatureGen2,
    pub card_sign_certificate: gen2::CertificateGen2,
    pub ca_certificate: gen2::CertificateGen2,
    pub link_certificate: gen2::CertificateGen2,
    pub identification: dt::Identification,
    pub identification_signature: gen2::SignatureGen2,
    pub card_download: Option<dt::CardDownload>,
    pub card_download_signature: Option<gen2::SignatureGen2>,
    pub driver_licence_info: Option<dt::CardDrivingLicenceInformation>,
    pub driver_licence_info_signature: Option<gen2::SignatureGen2>,
    pub events_data: gen2::CardEventDataGen2,
    pub events_data_signature: gen2::SignatureGen2,
    pub faults_data: gen2::CardFaultDataGen2,
    pub faults_data_signature: gen2::SignatureGen2,
    pub driver_activity_data: dt::DriverActivityData,
    pub driver_activity_data_signature: gen2::SignatureGen2,
    pub vehicles_used: gen2::CardVehiclesUsedGen2,
    pub vehicles_used_signature: gen2::SignatureGen2,
    pub places: gen2::CardPlaceDailyWorkPeriodGen2,
    pub places_signature: gen2::SignatureGen2,
    pub current_usage: Option<dt::CurrentUsage>,
    pub current_usage_signature: Option<gen2::SignatureGen2>,
    pub control_activity_data: gen2::CardControlActivityDataRecordGen2,
    pub control_activity_data_signature: gen2::SignatureGen2,
    pub specific_conditions: gen2::SpecificConditionsGen2,
    pub specific_conditions_signature: gen2::SignatureGen2,
    pub vehicle_units_used: gen2::CardVehicleUnitsUsedGen2,
    pub vehicle_units_used_signature: gen2::SignatureGen2,
    pub gnss_accumulated_driving: gen2::GnssAccumulatedDrivingGen2,
    pub gnss_places_signature: gen2::SignatureGen2,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct CardGen2V2Blocks {
    pub card_icc_identification: gen2::CardIccIdentificationGen2,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "generation")]
#[cfg_attr(feature = "ts", derive(TS))]
pub enum CardData {
    #[serde(rename_all = "camelCase")]
    Gen1 { gen1_blocks: CardGen1Blocks },
    #[serde(rename_all = "camelCase")]
    Gen2 {
        gen1_blocks: CardGen1Blocks,
        gen2_blocks: CardGen2Blocks,
    },
    #[serde(rename_all = "camelCase")]
    Gen2V2 {
        gen1_blocks: CardGen1Blocks,
        gen2_blocks: CardGen2Blocks,
        gen2v2_blocks: CardGen2V2Blocks,
    },
}

fn panic_on_duplicate_block_type(block_type: &str) {
    panic!("{}: duplicate block type detected. This suggests an unexpected structure in the Card files, where multiple instances of the same block type are present within a single generation. This indicates a bug in the parser.", block_type);
}

pub struct CardParser {
    input: Vec<u8>,
}
impl CardParser {
    pub fn new_from_file(file_path: &str) -> Result<Self> {
        let input = std::fs::read(file_path).expect("Failed to read file");
        Ok(CardParser { input })
    }
    pub fn new_from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(CardParser {
            input: bytes.to_vec(),
        })
    }

    pub fn parse(&self) -> Result<CardData> {
        let mut cursor = Cursor::new(&self.input[..]);
        let mut card_icc_identification: Option<gen1::CardIccIdentification> = None;
        let mut card_chip_identification: Option<dt::CardChipIdentification> = None;
        let mut application_identification: Option<gen1::DriverCardApplicationIdentification> =
            None;
        let mut application_identification_signature: Option<gen1::Signature> = None;
        let mut card_certificate: Option<gen1::Certificate> = None;
        let mut member_state_certificate: Option<gen1::Certificate> = None;
        let mut identification: Option<dt::Identification> = None;
        let mut identification_signature: Option<gen1::Signature> = None;
        let mut last_card_download: Option<dt::CardDownload> = None;
        let mut last_card_download_signature: Option<gen1::Signature> = None;
        let mut driver_licence_information: Option<dt::CardDrivingLicenceInformation> = None;
        let mut driver_licence_info_signature: Option<gen1::Signature> = None;
        let mut events_data: Option<gen1::CardEventData> = None;
        let mut events_data_signature: Option<gen1::Signature> = None;
        let mut faults_data: Option<gen1::CardFaultData> = None;
        let mut faults_data_signature: Option<gen1::Signature> = None;
        let mut driver_activity_data: Option<dt::DriverActivityData> = None;
        let mut driver_activity_data_signature: Option<gen1::Signature> = None;
        let mut vehicles_used: Option<gen1::CardVehiclesUsed> = None;
        let mut vehicles_used_signature: Option<gen1::Signature> = None;
        let mut places: Option<gen1::CardPlaceDailyWorkPeriod> = None;
        let mut places_signature: Option<gen1::Signature> = None;
        let mut current_usage: Option<dt::CurrentUsage> = None;
        let mut current_usage_signature: Option<gen1::Signature> = None;
        let mut control_activity_data: Option<gen1::CardControlActivityDataRecord> = None;
        let mut control_activity_data_signature: Option<gen1::Signature> = None;
        let mut specific_conditions: Option<gen1::SpecificConditions> = None;
        let mut specific_conditions_signature: Option<gen1::Signature> = None;

        // GEN2
        let mut card_icc_identification_gen2: Option<gen2::CardIccIdentificationGen2> = None;
        let mut card_chip_identification_gen2: Option<dt::CardChipIdentification> = None;
        let mut application_identification_gen2: Option<
            gen2::DriverCardApplicationIdentificationGen2,
        > = None;
        let mut application_identification_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut card_sign_certificate_gen2: Option<gen2::CertificateGen2> = None;
        let mut ca_certificate_gen2: Option<gen2::CertificateGen2> = None;
        let mut link_certificate_gen2: Option<gen2::CertificateGen2> = None;
        let mut identification_gen2: Option<dt::Identification> = None;
        let mut identification_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut card_download_gen2: Option<dt::CardDownload> = None;
        let mut card_download_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut driver_licence_info_gen2: Option<dt::CardDrivingLicenceInformation> = None;
        let mut driver_licence_info_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut events_data_gen2: Option<gen2::CardEventDataGen2> = None;
        let mut events_data_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut faults_data_gen2: Option<gen2::CardFaultDataGen2> = None;
        let mut faults_data_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut driver_activity_data_gen2: Option<dt::DriverActivityData> = None;
        let mut driver_activity_data_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut vehicles_used_gen2: Option<gen2::CardVehiclesUsedGen2> = None;
        let mut vehicles_used_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut places_gen2: Option<gen2::CardPlaceDailyWorkPeriodGen2> = None;
        let mut places_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut current_usage_gen2: Option<dt::CurrentUsage> = None;
        let mut current_usage_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut control_activity_data_gen2: Option<gen2::CardControlActivityDataRecordGen2> = None;
        let mut control_activity_data_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut specific_conditions_gen2: Option<gen2::SpecificConditionsGen2> = None;
        let mut specific_conditions_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut vehicle_units_used_gen2: Option<gen2::CardVehicleUnitsUsedGen2> = None;
        let mut vehicle_units_used_signature_gen2: Option<gen2::SignatureGen2> = None;
        let mut gnss_places_gen2: Option<gen2::GnssAccumulatedDrivingGen2> = None;
        let mut gnss_places_signature_gen2: Option<gen2::SignatureGen2> = None;

        // all data blocks for card files follow the structure
        // file_id (2 bytes), sfid (1 byte), size (2 bytes)
        while !cursor.fill_buf()?.is_empty() {
            let sfid = cursor
                .read_u16::<BigEndian>()
                .expect("Failed to read file_id");
            let file_id = cursor.read_u8().expect("Failed to read sfid");

            log::debug!(
                "Parsing card data with sfid: {:04X} and file_id: {:02X}",
                sfid,
                file_id
            );
            // Page 283
            match (sfid, file_id) {
                // CardIccIdentification Gen1
                (0x0002, 0) => {
                    if card_icc_identification.is_some() {
                        panic_on_duplicate_block_type("card_icc_identification");
                    }
                    card_icc_identification = Some(
                        CardBlock::parse(&mut cursor, gen1::CardIccIdentification::parse)?
                            .into_inner(),
                    );
                }
                // CardChipIdentification Gen1
                (0x0005, 0) => {
                    if card_chip_identification.is_some() {
                        panic_on_duplicate_block_type("card_chip_identification");
                    }
                    card_chip_identification = Some(
                        CardBlock::parse(&mut cursor, dt::CardChipIdentification::parse)?
                            .into_inner(),
                    );
                }
                // ApplicationIdentification Gen1
                (0x0501, 0) => {
                    if application_identification.is_some() {
                        panic_on_duplicate_block_type("application_identification");
                    }
                    application_identification = Some(
                        CardBlock::parse(
                            &mut cursor,
                            gen1::DriverCardApplicationIdentification::parse,
                        )?
                        .into_inner(),
                    );
                }
                // ApplicationIdentification Signature Gen1
                (0x0501, 1) => {
                    application_identification_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // CardCertificate Gen1
                (0xC100, 0) => {
                    card_certificate =
                        Some(CardBlock::parse(&mut cursor, gen1::Certificate::parse)?.into_inner());
                }
                // MemberStateCertificate Gen1
                (0xC108, 0) => {
                    member_state_certificate =
                        Some(CardBlock::parse(&mut cursor, gen1::Certificate::parse)?.into_inner());
                }
                // Identification Gen1
                (0x0520, 0) => {
                    if identification.is_some() {
                        panic_on_duplicate_block_type("identification");
                    }
                    identification = Some(
                        CardBlock::parse(&mut cursor, dt::Identification::parse)?.into_inner(),
                    );
                }
                // Identification Signature Gen1
                (0x0520, 1) => {
                    identification_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // CardDownload Gen1
                (0x050E, 0) => {
                    last_card_download =
                        Some(CardBlock::parse(&mut cursor, dt::CardDownload::parse)?.into_inner());
                }
                // CardDownload Signature Gen1
                (0x050E, 1) => {
                    last_card_download_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // DrivingLicenseInfo Gen1
                (0x0521, 0) => {
                    if driver_licence_information.is_some() {
                        panic_on_duplicate_block_type("driver_licence_information");
                    }
                    driver_licence_information = Some(
                        CardBlock::parse(&mut cursor, dt::CardDrivingLicenceInformation::parse)?
                            .into_inner(),
                    );
                }
                // DrivingLicenseInfo Signature Gen1
                (0x0521, 1) => {
                    driver_licence_info_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // EventsData Gen1
                (0x0502, 0) => {
                    if events_data.is_some() {
                        panic_on_duplicate_block_type("events_data");
                    }
                    events_data = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen1::CardEventData::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                // EventsData Signature Gen1
                (0x0502, 1) => {
                    events_data_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // FaultsData Gen1
                (0x0503, 0) => {
                    if faults_data.is_some() {
                        panic_on_duplicate_block_type("faults_data");
                    }
                    faults_data = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen1::CardFaultData::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                // FaultsData Signature Gen1
                (0x0503, 1) => {
                    faults_data_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // DriverActivityData Gen1
                (0x0504, 0) => {
                    if driver_activity_data.is_some() {
                        panic_on_duplicate_block_type("driver_activity_data");
                    }
                    driver_activity_data = Some(
                        CardBlock::parse(&mut cursor, dt::DriverActivityData::parse)?.into_inner(),
                    );
                }
                // DriverActivityData Signature Gen1
                (0x0504, 1) => {
                    driver_activity_data_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // VehiclesUsed Gen1
                (0x0505, 0) => {
                    if vehicles_used.is_some() {
                        panic_on_duplicate_block_type("vehicles_used");
                    }
                    vehicles_used = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen1::CardVehiclesUsed::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                // VehiclesUsed Signature Gen1
                (0x0505, 1) => {
                    vehicles_used_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // Places Gen1
                (0x0506, 0) => {
                    if places.is_some() {
                        panic_on_duplicate_block_type("places");
                    }
                    places = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen1::CardPlaceDailyWorkPeriod::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                // Places Signature Gen1
                (0x0506, 1) => {
                    places_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // CurrentUsage Gen1
                (0x0507, 0) => {
                    current_usage =
                        Some(CardBlock::parse(&mut cursor, dt::CurrentUsage::parse)?.into_inner());
                }
                // CurrentUsage Signature Gen1
                (0x0507, 1) => {
                    current_usage_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // ControlActivityData Gen1
                (0x0508, 0) => {
                    if control_activity_data.is_some() {
                        panic_on_duplicate_block_type("control_activity_data");
                    }
                    control_activity_data = Some(
                        CardBlock::parse(&mut cursor, gen1::CardControlActivityDataRecord::parse)?
                            .into_inner(),
                    );
                }
                // ControlActivityData Signature Gen1
                (0x0508, 1) => {
                    control_activity_data_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // SpecificConditions Gen1
                (0x0522, 0) => {
                    if specific_conditions.is_some() {
                        panic_on_duplicate_block_type("specific_conditions");
                    }
                    specific_conditions = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen1::SpecificConditions::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                // SpecificConditions Signature Gen1
                (0x0522, 1) => {
                    specific_conditions_signature =
                        Some(CardBlock::parse(&mut cursor, gen1::Signature::parse)?.into_inner());
                }
                // IMPL GEN2
                // CardIccIdentification Gen2
                (0x0002, 2) => {
                    if card_icc_identification_gen2.is_some() {
                        panic_on_duplicate_block_type("card_icc_identification_gen2");
                    }
                    card_icc_identification_gen2 = Some(
                        CardBlock::parse(&mut cursor, gen2::CardIccIdentificationGen2::parse)?
                            .into_inner(),
                    );
                }
                // CardChipIdentification Gen2
                (0x0005, 2) => {
                    if card_chip_identification_gen2.is_some() {
                        panic_on_duplicate_block_type("card_chip_identification_gen2");
                    }
                    card_chip_identification_gen2 = Some(
                        CardBlock::parse(&mut cursor, dt::CardChipIdentification::parse)?
                            .into_inner(),
                    );
                }
                // ApplicationIdentification Gen2
                (0x0501, 2) => {
                    if application_identification_gen2.is_some() {
                        panic_on_duplicate_block_type("application_identification_gen2");
                    }
                    application_identification_gen2 = Some(
                        CardBlock::parse(
                            &mut cursor,
                            gen2::DriverCardApplicationIdentificationGen2::parse,
                        )?
                        .into_inner(),
                    );
                }
                // ApplicationIdentification Signature Gen2
                (0x0501, 3) => {
                    if application_identification_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("application_identification_signature_gen2");
                    }
                    application_identification_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                // CardSignCertificate Gen2
                (0xC101, 2) => {
                    if card_sign_certificate_gen2.is_some() {
                        panic_on_duplicate_block_type("card_sign_certificate_gen2");
                    }
                    card_sign_certificate_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::CertificateGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                // MemberStateCertificate Gen2
                (0xC108, 2) => {
                    if ca_certificate_gen2.is_some() {
                        panic_on_duplicate_block_type("ca_certificate_gen2");
                    }
                    ca_certificate_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::CertificateGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                // LinkCertificate Gen2
                (0xC109, 2) => {
                    if link_certificate_gen2.is_some() {
                        panic_on_duplicate_block_type("link_certificate_gen2");
                    }
                    link_certificate_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::CertificateGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                // Identification Gen2
                (0x0520, 2) => {
                    if identification_gen2.is_some() {
                        panic_on_duplicate_block_type("identification_gen2");
                    }
                    identification_gen2 = Some(
                        CardBlock::parse(&mut cursor, dt::Identification::parse)?.into_inner(),
                    );
                }
                // Identification Signature Gen2
                (0x0520, 3) => {
                    if identification_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("identification_signature_gen2");
                    }
                    identification_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                // CardDownload Gen2
                (0x050E, 2) => {
                    card_download_gen2 =
                        Some(CardBlock::parse(&mut cursor, dt::CardDownload::parse)?.into_inner());
                }
                // CardDownload Signature Gen2
                (0x050E, 3) => {
                    if card_download_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("card_download_signature_gen2");
                    }
                    card_download_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                // DrivingLicenseInfo Gen2
                (0x0521, 2) => {
                    if driver_licence_info_gen2.is_some() {
                        panic_on_duplicate_block_type("driver_licence_info_gen2");
                    }
                    driver_licence_info_gen2 = Some(
                        CardBlock::parse(&mut cursor, dt::CardDrivingLicenceInformation::parse)?
                            .into_inner(),
                    );
                }
                (0x0521, 3) => {
                    if driver_licence_info_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("driver_licence_info_signature_gen2");
                    }
                    driver_licence_info_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0502, 2) => {
                    if events_data_gen2.is_some() {
                        panic_on_duplicate_block_type("events_data_gen2");
                    }
                    events_data_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::CardEventDataGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0502, 3) => {
                    if events_data_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("events_data_signature_gen2");
                    }
                    events_data_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0503, 2) => {
                    if faults_data_gen2.is_some() {
                        panic_on_duplicate_block_type("faults_data_gen2");
                    }
                    faults_data_gen2 = Some(
                        CardBlock::parse(&mut cursor, gen2::CardFaultDataGen2::parse)?.into_inner(),
                    );
                }
                (0x0503, 3) => {
                    if faults_data_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("faults_data_signature_gen2");
                    }
                    faults_data_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0504, 2) => {
                    if driver_activity_data_gen2.is_some() {
                        panic_on_duplicate_block_type("driver_activity_data_gen2");
                    }
                    driver_activity_data_gen2 = Some(
                        CardBlock::parse(&mut cursor, dt::DriverActivityData::parse)?.into_inner(),
                    );
                }
                (0x0504, 3) => {
                    if driver_activity_data_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("driver_activity_data_signature_gen2");
                    }
                    driver_activity_data_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0505, 2) => {
                    if vehicles_used_gen2.is_some() {
                        panic_on_duplicate_block_type("vehicles_used_gen2");
                    }
                    vehicles_used_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::CardVehiclesUsedGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0505, 3) => {
                    if vehicles_used_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("vehicles_used_signature_gen2");
                    }
                    vehicles_used_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0506, 2) => {
                    if places_gen2.is_some() {
                        panic_on_duplicate_block_type("places_gen2");
                    }
                    places_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::CardPlaceDailyWorkPeriodGen2::parse,
                        )?
                        .into_inner(),
                    );
                }
                (0x0506, 3) => {
                    if places_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("places_signature_gen2");
                    }
                    places_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0507, 2) => {
                    current_usage_gen2 =
                        Some(CardBlock::parse(&mut cursor, dt::CurrentUsage::parse)?.into_inner());
                }
                (0x0507, 3) => {
                    if current_usage_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("current_usage_signature_gen2");
                    }
                    current_usage_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0508, 2) => {
                    if control_activity_data_gen2.is_some() {
                        panic_on_duplicate_block_type("control_activity_data_gen2");
                    }
                    control_activity_data_gen2 = Some(
                        CardBlock::parse(
                            &mut cursor,
                            gen2::CardControlActivityDataRecordGen2::parse,
                        )?
                        .into_inner(),
                    );
                }
                (0x0508, 3) => {
                    if control_activity_data_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("control_activity_data_signature_gen2");
                    }
                    control_activity_data_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0522, 2) => {
                    if specific_conditions_gen2.is_some() {
                        panic_on_duplicate_block_type("specific_conditions_gen2");
                    }
                    specific_conditions_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SpecificConditionsGen2::parse,
                        )?
                        .into_inner(),
                    );
                }
                (0x0522, 3) => {
                    if specific_conditions_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("specific_conditions_signature_gen2");
                    }
                    specific_conditions_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0523, 2) => {
                    if vehicle_units_used_gen2.is_some() {
                        panic_on_duplicate_block_type("vehicle_units_used_gen2");
                    }
                    vehicle_units_used_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::CardVehicleUnitsUsedGen2::parse,
                        )?
                        .into_inner(),
                    );
                }
                (0x0523, 3) => {
                    if vehicle_units_used_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("vehicle_units_used_signature_gen2");
                    }
                    vehicle_units_used_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                (0x0524, 2) => {
                    if gnss_places_gen2.is_some() {
                        panic_on_duplicate_block_type("gnss_places_gen2");
                    }
                    gnss_places_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::GnssAccumulatedDrivingGen2::parse,
                        )?
                        .into_inner(),
                    );
                }
                (0x0524, 3) => {
                    if gnss_places_signature_gen2.is_some() {
                        panic_on_duplicate_block_type("gnss_places_signature_gen2");
                    }
                    gnss_places_signature_gen2 = Some(
                        CardBlock::parse_dyn_size(
                            &mut cursor,
                            gen2::SignatureGen2::parse_dyn_size,
                        )?
                        .into_inner(),
                    );
                }
                _ => {
                    log::debug!(
                        "Found unknown block with sfid: {:#04x}, file_id: {:#04x}",
                        sfid,
                        file_id
                    );
                    break;
                }
            }
        }

        let gen1_blocks = CardGen1Blocks {
            card_icc_identification: card_icc_identification
                .context("unable to find card_icc_identification gen1 after parsing file")?,
            card_chip_identification: card_chip_identification
                .context("unable to find card_chip_identification gen1 after parsing file")?,
            application_identification: application_identification
                .context("unable to find application_identification gen1 after parsing file")?,
            application_identification_signature: application_identification_signature.context(
                "unable to find application_identification_signature gen1 after parsing file",
            )?,
            card_certificate: card_certificate
                .context("unable to find card_certificate gen1 after parsing file")?,
            member_state_certificate: member_state_certificate
                .context("unable to find member_state_certificate gen1 after parsing file")?,
            identification: identification
                .context("unable to find identification gen1 after parsing file")?,
            identification_signature: identification_signature
                .context("unable to find identification_signature gen1 after parsing file")?,
            card_download: last_card_download,
            card_download_signature: last_card_download_signature,
            driver_licence_info: driver_licence_information,
            driver_licence_info_signature: driver_licence_info_signature,
            events_data: events_data
                .context("unable to find events_data gen1 after parsing file")?,
            events_data_signature: events_data_signature
                .context("unable to find events_data_signature gen1 after parsing file")?,
            faults_data: faults_data
                .context("unable to find faults_data gen1 after parsing file")?,
            faults_data_signature: faults_data_signature
                .context("unable to find faults_data_signature gen1 after parsing file")?,
            driver_activity_data: driver_activity_data
                .context("unable to find driver_activity_data gen1 after parsing file")?,
            driver_activity_data_signature: driver_activity_data_signature
                .context("unable to find driver_activity_data_signature gen1 after parsing file")?,
            vehicles_used: vehicles_used
                .context("unable to find vehicles_used gen1 after parsing file")?,
            vehicles_used_signature: vehicles_used_signature
                .context("unable to find vehicles_used_signature gen1 after parsing file")?,
            places: places.context("unable to find places gen1 after parsing file")?,
            places_signature: places_signature
                .context("unable to find places_signature gen1 after parsing file")?,
            current_usage: current_usage,
            current_usage_signature: current_usage_signature,
            control_activity_data: control_activity_data
                .context("unable to find control_activity_data gen1 after parsing file")?,
            control_activity_data_signature: control_activity_data_signature.context(
                "unable to find control_activity_data_signature gen1 after parsing file",
            )?,
            specific_conditions: specific_conditions
                .context("unable to find specific_conditions gen1 after parsing file")?,
            specific_conditions_signature: specific_conditions_signature
                .context("unable to find specific_conditions_signature gen1 after parsing file")?,
        };

        let mut gen2_blocks: Option<CardGen2Blocks> = None;

        if card_icc_identification_gen2.is_some() {
            let blocks = CardGen2Blocks {
                card_icc_identification: card_icc_identification_gen2
                    .context("unable to find card_icc_identification gen2 after parsing file")?,
                card_chip_identification: card_chip_identification_gen2
                    .context("unable to find card_chip_identification gen2 after parsing file")?,
                application_identification: application_identification_gen2
                    .context("unable to find application_identification gen2 after parsing file")?,
                application_identification_signature: application_identification_signature_gen2
                    .context(
                    "unable to find application_identification_signature gen2 after parsing file",
                )?,
                card_sign_certificate: card_sign_certificate_gen2
                    .context("unable to find card_sign_certificate gen2 after parsing file")?,
                ca_certificate: ca_certificate_gen2
                    .context("unable to find ca_certificate gen2 after parsing file")?,
                link_certificate: link_certificate_gen2
                    .context("unable to find link_certificate gen2 after parsing file")?,
                identification: identification_gen2
                    .context("unable to find identification gen2 after parsing file")?,
                identification_signature: identification_signature_gen2
                    .context("unable to find identification_signature gen2 after parsing file")?,
                card_download: card_download_gen2,
                card_download_signature: card_download_signature_gen2,
                driver_licence_info: driver_licence_info_gen2,
                driver_licence_info_signature: driver_licence_info_signature_gen2,
                events_data: events_data_gen2
                    .context("unable to find events_data gen2 after parsing file")?,
                events_data_signature: events_data_signature_gen2
                    .context("unable to find events_data_signature gen2 after parsing file")?,
                faults_data: faults_data_gen2
                    .context("unable to find faults_data gen2 after parsing file")?,
                faults_data_signature: faults_data_signature_gen2
                    .context("unable to find faults_data_signature gen2 after parsing file")?,
                driver_activity_data: driver_activity_data_gen2
                    .context("unable to find driver_activity_data gen2 after parsing file")?,
                driver_activity_data_signature: driver_activity_data_signature_gen2.context(
                    "unable to find driver_activity_data_signature gen2 after parsing file",
                )?,
                vehicles_used: vehicles_used_gen2
                    .context("unable to find vehicles_used gen2 after parsing file")?,
                vehicles_used_signature: vehicles_used_signature_gen2
                    .context("unable to find vehicles_used_signature gen2 after parsing file")?,
                places: places_gen2.context("unable to find places gen2 after parsing file")?,
                places_signature: places_signature_gen2
                    .context("unable to find places_signature gen2 after parsing file")?,
                current_usage: current_usage_gen2,
                current_usage_signature: current_usage_signature_gen2,
                control_activity_data: control_activity_data_gen2
                    .context("unable to find control_activity_data gen2 after parsing file")?,
                control_activity_data_signature: control_activity_data_signature_gen2.context(
                    "unable to find control_activity_data_signature gen2 after parsing file",
                )?,
                specific_conditions: specific_conditions_gen2
                    .context("unable to find specific_conditions gen2 after parsing file")?,
                specific_conditions_signature: specific_conditions_signature_gen2.context(
                    "unable to find specific_conditions_signature gen2 after parsing file",
                )?,
                vehicle_units_used: vehicle_units_used_gen2
                    .context("unable to find vehicle_units_used gen2 after parsing file")?,
                vehicle_units_used_signature: vehicle_units_used_signature_gen2.context(
                    "unable to find vehicle_units_used_signature gen2 after parsing file",
                )?,
                gnss_accumulated_driving: gnss_places_gen2
                    .context("unable to find gnss_accumulated_driving gen2 after parsing file")?,
                gnss_places_signature: gnss_places_signature_gen2
                    .context("unable to find gnss_places_signature gen2 after parsing file")?,
            };
            gen2_blocks = Some(blocks);
        }

        Ok(match (gen1_blocks, gen2_blocks) {
            (gen1, None) => CardData::Gen1 { gen1_blocks: gen1 },
            (gen1, Some(gen2)) => CardData::Gen2 {
                gen1_blocks: gen1,
                gen2_blocks: gen2,
            },
        })
    }

    pub fn parse_to_json(&self) -> Result<String> {
        let card_data = self.parse().context("Failed to parse vehicle data")?;
        let json = serde_json::to_string(&card_data)
            .context("Failed to convert serde value to JSON string")?;
        Ok(json)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "ts", derive(TS))]
pub struct CardBlock<T> {
    pub size: u16,
    pub data: T,
}

impl<T> CardBlock<T> {
    pub fn parse<F>(cursor: &mut Cursor<&[u8]>, parse_block: F) -> Result<Self>
    where
        F: Fn(&mut Cursor<&[u8]>) -> Result<T>,
    {
        let size = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read size in CardBlock")?;

        let mut buf = vec![0u8; size as usize];
        cursor.read_exact(&mut buf).context(format!(
            "Failed to read data in CardBlock of size {} for type {}",
            size,
            std::any::type_name::<T>()
        ))?;
        let mut inner_cursor = Cursor::new(buf.as_slice());

        let data = parse_block(&mut inner_cursor).context(format!(
            "Failed to parse data in CardBlock of size {} for type {}",
            size,
            std::any::type_name::<T>()
        ))?;

        let consumed = inner_cursor.position();
        if consumed < size as u64 {
            let unused_bytes = size as u64 - consumed;
            log::warn!(
                "CardBlock of type {} did not consume all bytes. Expected to consume {} bytes, but only consumed {}. {} bytes were unused.",
                std::any::type_name::<T>(),
                size,
                consumed,
                unused_bytes
            );
        }

        Ok(CardBlock { size, data })
    }

    pub fn parse_dyn_size<F>(cursor: &mut Cursor<&[u8]>, parse_block: F) -> Result<Self>
    where
        F: Fn(&mut Cursor<&[u8]>, usize) -> Result<T>,
    {
        let size = cursor
            .read_u16::<BigEndian>()
            .context("Failed to read size in CardBlock")?;

        let mut buf = vec![0u8; size as usize];
        cursor.read_exact(&mut buf).context(format!(
            "Failed to read data in CardBlock of size {} for type {}",
            size,
            std::any::type_name::<T>()
        ))?;

        let mut inner_cursor = Cursor::new(buf.as_slice());
        let data = parse_block(&mut inner_cursor, size as usize).context(format!(
            "Failed to parse data with dyn size in CardBlock of size {}",
            size
        ))?;

        let consumed = inner_cursor.position();
        if consumed < size as u64 {
            let unused_bytes = size as u64 - consumed;
            log::warn!(
                "CardBlock of type {} with dynamic size did not consume all bytes. Expected to consume {} bytes, but only consumed {}. {} bytes were unused.",
                std::any::type_name::<T>(),
                size,
                consumed,
                unused_bytes
            );
        }

        Ok(CardBlock { size, data })
    }
    pub fn into_inner(self) -> T {
        self.data
    }
}
