use std::fmt::Debug;
use std::io::{Cursor, Write};
use std::{fmt::Display, str::FromStr};

use byteorder::{BigEndian, WriteBytesExt};
use hex::FromHex;
use sha1::{Digest, Sha1};

use crate::error::MacAddressError;

pub struct MacAddress([u8; 6]);

impl FromStr for MacAddress {
    type Err = MacAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.replace('-', "");
        let bytes = Vec::from_hex(&stripped)?;
        let slice = bytes.as_slice();
        Ok(Self(slice.try_into()?))
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.0 {
            write!(f, "{:02X}", chunk)?
        }
        Ok(())
    }
}

pub struct WiiId {
    bytes: Vec<u8>,
    pub upper: u32,
    pub lower: u32,
}

impl Debug for WiiId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WiiId")
            .field("bytes", &format_args!("{:02x?}", &self.bytes))
            .field("upper", &format_args!("0x{:08x}", &self.upper))
            .field("lower", &format_args!("0x{:08x}", &self.lower))
            .finish()
    }
}

impl<'a> From<&'a MacAddress> for WiiId {
    fn from(mac: &'a MacAddress) -> Self {
        let mut mac = mac.0.to_vec();
        mac.extend([0x75, 0x79, 0x79]);
        let mut hasher = Sha1::new();
        hasher.update(mac);
        let r = hasher.finalize();
        Self::new(r[..].to_vec())
    }
}

impl WiiId {
    fn new(bytes: Vec<u8>) -> Self {
        let mut chunks = bytes.chunks_exact(4);
        let upper = chunks.next().expect("missing first u32 chunk");
        let lower = chunks.next().expect("missing second u32 chunk");
        let upper = u32::from_be_bytes(upper.try_into().unwrap());
        let lower = u32::from_be_bytes(lower.try_into().unwrap());

        Self {
            bytes,
            upper,
            lower,
        }
    }

    pub fn split_upper_lower(bytes: &[u8]) -> Option<(u32, u32)> {
        let mut chunks = bytes.chunks_exact(4);
        let upper = chunks.next()?;
        let lower = chunks.next()?;

        Some((
            u32::from_be_bytes(upper.try_into().unwrap()),
            u32::from_be_bytes(lower.try_into().unwrap()),
        ))
    }

    pub fn hmac_key(&self) -> &[u8] {
        &self.bytes[8..]
    }
}

pub(crate) struct PayloadWriter<'a> {
    cur: Cursor<&'a mut [u8]>,
    base: u64,
    offset: u64,
    xor_key: u32,
}

impl<'a> PayloadWriter<'a> {
    pub(crate) fn new(buf: &'a mut [u8], base: u64, xor_key: u32) -> Self {
        Self {
            cur: Cursor::new(&mut *buf),
            base,
            offset: 0,
            xor_key,
        }
    }

    pub(crate) fn increment_offset(&mut self, offset: u64) {
        self.offset += offset;
    }

    pub(crate) fn write_raw(&mut self, val: &[u8]) -> std::io::Result<()> {
        assert_eq!(val.len(), 4);
        self.cur.set_position(self.base + self.offset);
        self.cur.write_all(val)?;
        self.offset += 4;
        Ok(())
    }

    pub(crate) fn l(&mut self, val: u32) -> std::io::Result<()> {
        self.cur.set_position(self.base + self.offset);
        self.cur.write_u32::<BigEndian>(val)?;
        self.offset += 4;
        Ok(())
    }

    pub(crate) fn x(&mut self, val: u32) -> std::io::Result<()> {
        self.l(val ^ self.xor_key)?;
        Ok(())
    }
}
