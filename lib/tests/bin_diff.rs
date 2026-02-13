use std::{fs, path::Path, str::FromStr};

use chrono::NaiveDate;
use lib::{consts::SYSMENU_KEYS, payload::build_payload, types::MacAddress};

#[test]
fn diff_original_binary() {
    let mac = MacAddress::from_str("aa-bb-cc-dd-ee-ff").unwrap();
    let date = NaiveDate::from_ymd_opt(2022, 5, 2).unwrap();

    for version in SYSMENU_KEYS {
        let payload = build_payload(&mac, &date, version).unwrap();
        let bytes = payload.bin;

        let bin_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("bins")
            .join(format!("{}.bin", version));
        let expected = fs::read(bin_path).unwrap();

        assert_eq!(expected, bytes);
    }
}
