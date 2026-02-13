use clap::Parser;
use clap::builder::PossibleValuesParser;
use lib::consts::SYSMENU_KEYS;
use lib::payload::build_payload;
use lib::types::MacAddress;
use log::info;

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::NaiveDate;

/// A program used to build the mailbox bomb exploit for the Wii system menu
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, version = concat!(env!("CARGO_PKG_VERSION"), " (", env!("GIT_HASH"), ", ", env!("BUILD_DATE"), ")"))]
struct Args {
    /// MAC address in the format 'AA-BB-CC-DD-EE-FF'
    #[arg(value_parser = validate_mac)]
    mac_address: MacAddress,

    /// Date in the format 'dd-MM-yyy; (e.g: 13-02-2026)
    #[arg(value_parser = validate_date)]
    date: NaiveDate,

    /// System Menu Version (e.g: 4.3u)
    #[arg(value_parser = PossibleValuesParser::new(SYSMENU_KEYS))]
    sys_version: String,

    /// Output directory
    out_dir: String,
}

pub fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    let args = Args::parse();

    let payload = build_payload(&args.mac_address, &args.date, &args.sys_version)?;

    let folder_path = create_dirs(&PathBuf::from(args.out_dir), &payload.path)?;
    let file_path = Path::new(&folder_path).join(&payload.file_name);
    create_file(&file_path, &payload.bin)?;

    info!("wrote {} bytes to {:?}", payload.bin.len(), file_path);

    Ok(())
}

fn validate_mac(s: &str) -> Result<MacAddress, String> {
    MacAddress::from_str(s).map_err(|e| e.to_string())
}

fn validate_date(date: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date, "%d-%m-%Y")
        .map_err(|e| format!("invalid date, must be in dd-MM-yyyy format: {}", e))
}

fn create_dirs(base_dir: &PathBuf, path: &str) -> std::io::Result<PathBuf> {
    let mut dir_path: PathBuf = PathBuf::new().join(base_dir);

    for segment in path.split('/') {
        dir_path = dir_path.join(segment)
    }

    info!("creating directories {:?}", dir_path);
    fs::create_dir_all(&dir_path)?;

    Ok(dir_path)
}

fn create_file(file_path: &PathBuf, contents: &[u8]) -> std::io::Result<()> {
    info!("writing file {:?}", file_path);
    let mut file = File::create(file_path)?;
    file.write_all(contents)?;

    Ok(())
}
