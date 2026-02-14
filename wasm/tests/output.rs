use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use wasm::create_payload;

wasm_bindgen_test_configure!(run_in_browser);

const BIN: &[u8] = include_bytes!("../../lib/tests/bins/4.3e.bin");

#[wasm_bindgen_test]
fn test_output() {
    let payload = create_payload("aa-bb-cc-dd-ee-ff", "02-05-2022", "4.3e").unwrap();
    assert_eq!(BIN, payload.bin)
}
