use std::io;
use std::io::Read;
use std::io::Result as IoResult;

pub mod nbt;

pub trait ReadSkip: Read {
    fn skip_all(&mut self, size: usize) -> IoResult<()>;
}
impl<R: ReadSkip> ReadSkip for &mut R {
    fn skip_all(&mut self, size: usize) -> IoResult<()> {
        (**self).skip_all(size)
    }
}
impl ReadSkip for &[u8] {
    fn skip_all(&mut self, size: usize) -> IoResult<()> {
        if size > self.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "failed to fill whole buffer",
            ));
        }
        *self = &self[size..];
        Ok(())
    }
}
