use super::de::read_u8;
use anyhow::Result;
use std::io::{Read, Result as IoResult, Write};

pub(crate) fn read_varint_with_size<R: Read>(reader: &mut R) -> Result<(i32, usize)> {
    let mut result = 0;
    let mut bytes_read = 0usize;
    loop {
        let read = read_u8(&mut *reader)?;
        let value = read & 0b01111111;
        result |= (value as u32) << (7 * bytes_read as u32);
        bytes_read += 1;

        if bytes_read > 5 {
            return Err(anyhow::anyhow!("varint can't be bigger than 5 bytes"));
        }
        if read & 0b1000_0000 == 0 {
            break;
        }
    }

    Ok((result as i32, bytes_read))
}

pub(crate) fn read_varint<R: Read>(reader: &mut R) -> Result<i32> {
    let (value, _) = read_varint_with_size(reader)?;
    Ok(value)
}

pub(crate) fn read_varlong<R: Read>(reader: &mut R) -> Result<i64> {
    let mut result = 0;
    let mut bytes_read = 0usize;
    loop {
        let read = read_u8(&mut *reader)?;
        let value = read & 0b01111111;
        result |= (value as u64) << (7 * bytes_read as u64);
        bytes_read += 1;

        if bytes_read > 10 {
            return Err(anyhow::anyhow!("varlong can't be bigger than 10 bytes"));
        }
        if read & 0b1000_0000 == 0 {
            break;
        }
    }

    Ok(result as i64)
}

#[derive(Default)]
pub struct VarintSerialized {
    pub buffer: [u8; 5],
    pub size: usize,
}

pub fn write_varint_serialize(mut value: u32) -> VarintSerialized {
    let mut buffer = [0; 5];
    let mut size = 0;
    loop {
        let mut temp = (value & 0b01111111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b10000000;
        }
        buffer[size] = temp;
        size += 1;

        if value == 0 {
            break;
        }
    }

    VarintSerialized { buffer, size }
}

pub fn write_varint<W: Write>(mut writer: W, mut value: u32) -> IoResult<u32> {
    let mut count = 0;
    loop {
        let mut temp = (value & 0b01111111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b10000000;
        }
        writer.write_all(&[temp])?;
        count += 1;

        if value == 0 {
            break;
        }
    }

    Ok(count)
}

pub(crate) fn write_varlong<W: Write>(mut writer: W, mut value: u64) -> IoResult<()> {
    loop {
        let mut temp = (value & 0b01111111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b10000000;
        }
        writer.write_all(&[temp])?;

        if value == 0 {
            break;
        }
    }

    Ok(())
}
