pub mod logger;

use chrono::NaiveDate;
use lib::{consts::SYSMENU_KEYS, payload::build_payload, types::MacAddress};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_supported_versions() -> Vec<String> {
    SYSMENU_KEYS
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
}

#[wasm_bindgen(getter_with_clone)]
pub struct WasmPayload {
    pub bin: Vec<u8>,
    pub path: String,
    pub file_name: String,
}

#[wasm_bindgen]
pub fn create_payload(
    mac_address: &str,
    date: &str,
    sys_version: &str,
) -> Result<WasmPayload, JsValue> {
    let mac_address =
        MacAddress::from_str(mac_address).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let date = NaiveDate::parse_from_str(date, "%d-%m-%Y")
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let payload = build_payload(&mac_address, &date, sys_version)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(WasmPayload {
        bin: payload.bin.to_vec(),
        path: payload.path,
        file_name: payload.file_name,
    })
}
