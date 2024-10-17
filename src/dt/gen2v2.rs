use crate::dt::gen2;
use crate::dt::gen2::RecordArray;
use crate::dt::*;

// TOOD: determine if we need to reimplement gen2v2 RecordArray helper
// pub struct RecordArray<T> {}

pub type MemberStateCertificateRecordArray = RecordArray<gen2::MemberStateCertificate>;
pub type VuCertificateRecordArray = RecordArray<gen2::VuCertificate>;
pub type VehicleIdentificationNumberRecordArray = RecordArray<VehicleIdentificationNumber>;
pub type VehicleRegistrationNumberRecordArray = RecordArray<VehicleRegistrationNumber>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VuOverviewBlock {
    pub member_state_certificate_record_array: MemberStateCertificateRecordArray,
    pub vu_certificate_record_array: VuCertificateRecordArray,
    pub vehicle_identification_number_record_array: VehicleIdentificationNumberRecordArray,
    pub vehicle_registration_number_record_array: VehicleRegistrationNumberRecordArray,
}

impl VuOverviewBlock {
    pub fn parse(reader: &mut dyn Read) -> Result<Self> {
        let member_state_certificate_record_array =
            MemberStateCertificateRecordArray::parse_dyn_size(
                reader,
                gen2::MemberStateCertificate::parse_dyn_size,
            )
            .context("Failed to parse member_state_certificate_record_array")?;

        let vu_certificate_record_array =
            VuCertificateRecordArray::parse_dyn_size(reader, gen2::VuCertificate::parse_dyn_size)
                .context("Failed to parse vu_certificate_record_array")?;

        let vehicle_identification_number_record_array =
            VehicleIdentificationNumberRecordArray::parse(
                reader,
                VehicleIdentificationNumber::parse,
            )
            .context("Failed to parse vehicle_identification_number_record_array")?;

        let vehicle_registration_number_record_array =
            VehicleRegistrationNumberRecordArray::parse(reader, VehicleRegistrationNumber::parse)
                .context("Failed to parse vehicle_registration_number_record_array")?;

        Ok(VuOverviewBlock {
            member_state_certificate_record_array,
            vu_certificate_record_array,
            vehicle_identification_number_record_array,
            vehicle_registration_number_record_array,
        })
    }
}
