use crate::dt::gen2;
use crate::dt::gen2::RecordArray;
use crate::dt::*;
#[cfg(feature = "napi")]
use napi_derive::napi;

pub type MemberStateCertificateRecordArray = Vec<gen2::MemberStateCertificate>;
pub type VuCertificateRecordArray = Vec<gen2::VuCertificate>;
pub type VehicleIdentificationNumberRecordArray = Vec<VehicleIdentificationNumber>;
pub type VehicleRegistrationNumberRecordArray = Vec<VehicleRegistrationNumber>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
pub struct VuOverviewBlock {
    pub member_state_certificate_record_array: MemberStateCertificateRecordArray,
    pub vu_certificate_record_array: VuCertificateRecordArray,
    pub vehicle_identification_number_record_array: VehicleIdentificationNumberRecordArray,
    pub vehicle_registration_number_record_array: VehicleRegistrationNumberRecordArray,
}

impl VuOverviewBlock {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let member_state_certificate_record_array =
            RecordArray::parse_dyn_size(cursor, gen2::MemberStateCertificate::parse_dyn_size)
                .context("Failed to parse member_state_certificate_record_array")?
                .into_inner();

        let vu_certificate_record_array =
            RecordArray::parse_dyn_size(cursor, gen2::VuCertificate::parse_dyn_size)
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
