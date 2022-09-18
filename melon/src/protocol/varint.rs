use anyhow::Result;
use byteorder::ReadBytesExt;
use std::io::Read;

pub fn read_varint_with_size<R: Read>(mut reader: R) -> Result<(i32, usize)> {
    let mut result = 0;
    let mut bytes_read = 0;
    loop {
        let read = reader.read_u8()?;
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

pub fn read_varint<R: Read>(reader: R) -> Result<i32> {
    let (value, _) = read_varint_with_size(reader)?;
    Ok(value)
}

pub fn write_varint(mut value: u32) -> ([u8; 10], usize) {
    let mut result = [0; 10];
    let mut index = 0;
    loop {
        let mut temp = (value & 0b01111111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b10000000;
        }
        result[index] = temp;
        index += 1;
        if value == 0 {
            break;
        }
    }

    (result, index)
}
