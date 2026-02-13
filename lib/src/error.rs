use std::array::TryFromSliceError;

use aes::cipher::{InvalidLength, inout::PadError};
use thiserror::Error;

use crate::consts::SYSMENU_KEYS;

#[derive(Error, Debug)]
pub enum MacAddressError {
    #[error("invalid mac address, must be in 'AA-BB-CC-DD-EE-FF' format")]
    InvalidMacAddress(#[from] TryFromSliceError),
    #[error("invalid hex literal: `{0}`")]
    InvalidHex(#[from] hex::FromHexError),
}

#[derive(Error, Debug)]
pub enum BuildPayloadError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid u32 slice from BigEndian slide: {0}")]
    InvalidU32SliceFromBe(#[from] TryFromSliceError),
    #[error("invalid length for new_from_slice")]
    CryptoInvalidLength(#[from] InvalidLength),
    #[error("encrypt pad error")]
    CryptoPadError,
    #[error("invalid date")]
    InvalidDate,
    #[error("invalid timestamp")]
    InvalidTimestamp,
    #[error("invalid or unsupported system version; supported versions: {}", SYSMENU_KEYS.join(", "))]
    InvalidSystemVersion,
    #[error("year must be between 2000 and 2035")]
    InvalidYear,
}

// TODO: hacks lol
impl From<PadError> for BuildPayloadError {
    fn from(_: PadError) -> Self {
        BuildPayloadError::CryptoPadError
    }
}
