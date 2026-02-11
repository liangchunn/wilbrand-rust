use lib::payload::build_payload;
use lib::types::MacAddress;

use std::fs;
use std::str::FromStr;
use std::time::Instant;

use chrono::NaiveDate;

pub fn main() -> anyhow::Result<()> {
    // println!("{:#?}", SYSMENU_KEYS);
    // inputs
    let mac_address = MacAddress::from_str("00-00-00-00-00-00")?;
    let date = NaiveDate::from_ymd_opt(2026, 2, 10).ok_or(anyhow::anyhow!("invalid date"))?;
    let sys_ver = "4.3e";

    let now = Instant::now();
    let payload = build_payload(&mac_address, &date, sys_ver)?;
    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
    fs::write(&payload.file_name, payload.bin)?;

    println!("{}", payload.path);
    println!("wrote {} bytes to {}", payload.bin.len(), payload.file_name);

    Ok(())
}
