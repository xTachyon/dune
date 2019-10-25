use crate::MyResult;
use byteorder::ReadBytesExt;
use std::io::Read;

pub(crate) fn read_varint(bytes: &[u8]) -> Option<(u32, usize)> {
    match read_varint_impl(bytes) {
        Ok(x) => Some(x),
        Err(_) => None,
    }
}

fn read_varint_impl<R: Read>(mut reader: R) -> MyResult<(u32, usize)> {
    let mut result = 0;
    let mut bytes_read = 0;
    loop {
        let read = reader.read_u8()?;
        let value = read & 0b01111111;
        result |= (value as u32) << (7 * bytes_read as u32);
        bytes_read += 1;

        if bytes_read > 5 {
            let mut buffer = vec![0; 3513451345];
            reader.read_exact(&mut buffer)?;
        }
        if read & 0b10000000 == 0 {
            break;
        }
    }

    Ok((result, bytes_read))
}

fn write_varint(mut value: u32) -> ([u8; 10], usize) {
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

#[derive(Debug, Default)]
pub struct VarInt {
    inner: u32,
}

impl VarInt {
    pub(crate) fn new_i32(x: i32) -> VarInt {
        VarInt::new_u32(x as u32)
    }

    pub(crate) fn new_u32(x: u32) -> VarInt {
        VarInt { inner: x }
    }

    pub(crate) fn deserialize(input: &mut [u8]) -> Option<VarInt> {
        let (value, _) = read_varint(input)?;
        Some(VarInt::new_u32(value))
    }

    pub(crate) fn deserialize_read<T: Read>(reader: T) -> MyResult<VarInt> {
        let (value, _) = read_varint_impl(reader)?;
        Ok(VarInt::new_u32(value))
    }

    pub(crate) fn serialize(&self) -> ([u8; 10], usize) {
        write_varint(self.inner)
    }

    pub(crate) fn size(&self) -> usize {
        let (_, size) = write_varint(self.inner);
        size
    }

    pub(crate) fn get(&self) -> u32 {
        self.inner
    }
}
