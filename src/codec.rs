use tokio::codec::{Encoder, Decoder};
use std::io;
use bytes::{Bytes, BytesMut, BufMut};
use crate::varint::read_varint;

pub(crate) struct PacketCodec;

impl Encoder for PacketCodec {
  type Item = Bytes;
  type Error = io::Error;

  fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
    dst.put(item);
    Ok(())
  }
}

impl Decoder for PacketCodec {
  type Item = Bytes;
  type Error = io::Error;

  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    let (size, read) = match read_varint(&src) {
      Some(x) => x,
      None => return Ok(None)
    };
    if size as usize + read > src.len() {
      return Ok(None)
    }

    let mut packet_bytes = src.split_to(read + size as usize);
//    let packet_bytes = &packet_bytes[read..];
    Ok(Some(packet_bytes.freeze()))
  }
}