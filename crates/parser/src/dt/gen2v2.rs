use crate::dt::gen2;
use crate::dt::gen2::RecordArray;
use crate::dt::*;
#[cfg(feature = "napi")]
use napi_derive::napi;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(feature = "napi", napi(object))]
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
