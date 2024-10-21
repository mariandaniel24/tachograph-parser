use napi::bindgen_prelude::Buffer;
use tachograph_parser;
#[macro_use]
extern crate napi_derive;

#[napi]
pub fn parse_vu(bytes: Buffer) -> Result<tachograph_parser::vu_parser::VuData, napi::Error> {
    tachograph_parser::parse_vu_from_bytes(&bytes)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn parse_card(bytes: Buffer) -> Result<tachograph_parser::card_parser::CardData, napi::Error> {
    tachograph_parser::parse_card_from_bytes(&bytes)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn detect_tacho_file_type(
    bytes: Buffer,
) -> Result<tachograph_parser::detector::TachoFileType, napi::Error> {
    tachograph_parser::detector::detect_from_bytes(&bytes)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn parse_tacho_file(bytes: Buffer) -> Result<tachograph_parser::TachoData, napi::Error> {
    tachograph_parser::parse_from_bytes(&bytes).map_err(|e| napi::Error::from_reason(e.to_string()))
}
