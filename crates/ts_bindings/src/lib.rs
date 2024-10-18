use napi::bindgen_prelude::Buffer;

#[macro_use]
extern crate napi_derive;

#[napi(string_enum)]
pub enum FileType {
  VehicleUnitGen1,
  VehicleUnitGen2,
  VehicleUnitGen2V2,
  DriverCardGen1,
  DriverCardGen2,
  DriverCardGen2V2,
}

// pub type SomeNation = FileType;

// #[napi]
// struct Driver {
//   first_name: String,
//   last_name: String,
//   drivingLicenceNumber: String,
//   drivingLicenceIssuingNation: String,
// }

// #[napi]
// struct DriverCardFile {
//   driver: Driver,
// }

// #[napi]
// pub fn parse_driver_card(data: Buffer) -> DriverCardFile {
//   let data = data.to_vec();
//   let card = tachograph_parser::process_card_bytes(&data).unwrap();

// card.gen2_blocks.

// }

#[napi]
pub fn detect_file_type(data: Buffer) -> FileType {
  let data = data.to_vec();
  tachograph_parser::detector::detect_from_bytes(&data)
    .unwrap()
    .into()
}
