#![allow(warnings)]
use napi::bindgen_prelude::Buffer;
use tachograph_parser::card_parser::CardData;
use tachograph_parser::detector::TachoFileType;
use tachograph_parser::vu_parser::VuData;
use ts_rs::TS;

#[macro_use]
extern crate napi_derive;

#[derive(TS)]
#[ts(export)]
struct NoopStruct {
    card_data: CardData,
    vu_data: VuData,
    tacho_file_type: TachoFileType,
}

#[napi(ts_return_type = "VuData")]
pub fn parse_vu(bytes: Buffer) -> Result<String, napi::Error> {
    std::panic::catch_unwind(|| {
        tachograph_parser::parse_vu_from_bytes_to_json(&bytes)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    })
    .unwrap_or_else(|panic| {
        let panic_msg = panic
            .downcast_ref::<String>()
            .map(|s| s.as_str())
            .or_else(|| panic.downcast_ref::<&str>().copied())
            .unwrap_or("Unknown panic");
        Err(napi::Error::from_reason(format!(
            "Parser panicked: {}",
            panic_msg
        )))
    })
}

#[napi(ts_return_type = "CardData")]
pub fn parse_card(bytes: Buffer) -> Result<String, napi::Error> {
    std::panic::catch_unwind(|| {
        tachograph_parser::parse_card_from_bytes_to_json(&bytes)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    })
    .unwrap_or_else(|panic| {
        let panic_msg = panic
            .downcast_ref::<String>()
            .map(|s| s.as_str())
            .or_else(|| panic.downcast_ref::<&str>().copied())
            .unwrap_or("Unknown panic");
        Err(napi::Error::from_reason(format!(
            "Parser panicked: {}",
            panic_msg
        )))
    })
}

#[napi(ts_return_type = "TachoFileType")]
pub fn detect_tacho_file_type(bytes: Buffer) -> Result<String, napi::Error> {
    std::panic::catch_unwind(|| {
        let value = tachograph_parser::detector::detect_from_bytes(&bytes)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(value.to_string())
    })
    .unwrap_or_else(|panic| {
        let panic_msg = panic
            .downcast_ref::<String>()
            .map(|s| s.as_str())
            .or_else(|| panic.downcast_ref::<&str>().copied())
            .unwrap_or("Unknown panic");
        Err(napi::Error::from_reason(format!(
            "Parser panicked: {}",
            panic_msg
        )))
    })
}
