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
pub struct Args {
    /// MAC address in the format 'AA-BB-CC-DD-EE-FF'
    #[arg(value_parser = validate_mac)]
    pub mac_address: MacAddress,

    /// Date in the format 'dd-MM-yyy; (e.g: 13-02-2026)
    #[arg(value_parser = validate_date)]
    pub date: NaiveDate,

    /// System Menu Version (e.g: 4.3u)
    #[arg(value_parser = PossibleValuesParser::new(SYSMENU_KEYS))]
    pub sys_version: String,

    /// Output directory
    pub out_dir: String,
}

pub fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    let args = Args::parse();
    run(args)
}

pub fn run(args: Args) -> anyhow::Result<()> {
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

#[cfg(test)]
mod tests {
    use std::{fs, path::Path, str::FromStr};

    use chrono::NaiveDate;
    use lib::types::MacAddress;

    use crate::{Args, run};

    #[test]
    fn test_cli_write() {
        let dir_path = tempdir::TempDir::new("test_dir").unwrap();
        let args = Args {
            date: NaiveDate::from_ymd_opt(2022, 5, 2).unwrap(),
            mac_address: MacAddress::from_str("aa-bb-cc-dd-ee-ff").unwrap(),
            out_dir: dir_path.path().to_str().unwrap().to_owned(),
            sys_version: "4.3e".into(),
        };
        let path_segments = [
            "private",
            "wii",
            "title",
            "HAEA",
            "2669460f",
            "ada5ed77",
            "2022",
            "04",
            "02",
            "23",
            "59",
            "PUNE_69",
            "log",
            "2a032cc4.000",
        ];
        let mut file_path = dir_path.into_path();
        for segment in path_segments {
            file_path = file_path.join(segment)
        }
        let bin_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("lib")
            .join("tests")
            .join("bins")
            .join("4.3e.bin")
            .canonicalize()
            .unwrap();
        let bin_contents = fs::read(bin_path).unwrap();

        run(args).unwrap();

        let contents = fs::read(file_path).unwrap();

        assert_eq!(bin_contents, contents)
    }
}
