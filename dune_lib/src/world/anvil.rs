use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;

use anyhow::bail;
use anyhow::Result;
use bitvec::vec::BitVec;
use flate2::write::ZlibDecoder;

const SECTOR_SIZE: usize = 4096;
pub const CHUNKS_PER_REGION: usize = 1024;

pub struct Region {
    file: BufReader<File>,
    header: [u8; SECTOR_SIZE],
    sectors: BitVec,
}

impl Region {
    pub fn load(path: &Path, sectors_map: bool) -> Result<Region> {
        let extension = path.extension().unwrap_or_default();
        if extension != "mca" {
            bail!(
                "invalid file extension: {:?}, for file {}",
                extension,
                path.display()
            );
        }
        let mut file = BufReader::with_capacity(128 * 1024, File::open(path)?);

        let file_size: usize = file.get_ref().metadata()?.len().try_into()?;
        if file_size < SECTOR_SIZE * 2 || !file_size.is_multiple_of(SECTOR_SIZE) {
            bail!(
                "invalid file size: {}, for file {}",
                file_size,
                path.display()
            );
        }
        let number_of_sectors = file_size / SECTOR_SIZE;

        let mut header = [0; SECTOR_SIZE];
        file.read_exact(&mut header)?;

        let mut region = Region {
            file,
            header,
            sectors: BitVec::new(),
        };
        if !sectors_map {
            return Ok(region);
        }
        region.sectors.resize(number_of_sectors, false);
        region.sectors.set(0, true); // locations
        region.sectors.set(1, true); // timestamps

        for chunk_index in 0..CHUNKS_PER_REGION {
            let (loc, size) = region.sector_loc_and_size(chunk_index);
            if loc + size > number_of_sectors {
                bail!(
                    "expected sector in the range 0..{}, found {}",
                    number_of_sectors,
                    loc + size
                );
            }

            for i in 0..size {
                let off = loc + i;
                if region.sectors[off] {
                    bail!("sector used by 2 different chunks: {}", off);
                }
                region.sectors.set(off, true);
            }
        }

        Ok(region)
    }

    fn sector_loc_and_size(&self, chunk_index: usize) -> (usize, usize) {
        let off = chunk_index * 4;
        let size = self.header[off + 3] as usize;
        let loc = [
            0,
            self.header[off],
            self.header[off + 1],
            self.header[off + 2],
        ]; // 3 bytes big endian :squint:
        let loc = u32::from_be_bytes(loc) as usize;

        (loc, size)
    }
    fn sector_loc_and_size_real(&self, chunk_index: usize) -> (usize, usize) {
        let (loc, size) = self.sector_loc_and_size(chunk_index);
        (loc * SECTOR_SIZE, size * SECTOR_SIZE)
    }

    pub fn get_chunk<'x>(&mut self, vec: &'x mut Vec<u8>, chunk_index: usize) -> Result<&'x [u8]> {
        const CHUNKS_HEADER_SIZE: usize = 5;

        assert!(chunk_index < CHUNKS_PER_REGION);
        let (loc, size) = self.sector_loc_and_size_real(chunk_index);
        if size == 0 {
            return Ok(&[]);
        }
        vec.clear();
        vec.reserve(size);

        let mut buffer = [0; SECTOR_SIZE];
        self.file.seek(SeekFrom::Start(loc as u64))?;
        self.file.read_exact(&mut buffer)?;

        let size = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize - 1;
        // -1 because the compression byte is included here
        let compression_type = buffer[4];
        if compression_type != 2 {
            bail!(
                "unknown compression type: {}. only zlib (2) is supported",
                compression_type
            )
        }
        let mut decoder = ZlibDecoder::new(vec);
        let first_batch = size.min(SECTOR_SIZE - CHUNKS_HEADER_SIZE);
        decoder.write_all(&buffer[CHUNKS_HEADER_SIZE..CHUNKS_HEADER_SIZE + first_batch])?;

        let mut remaining_size = size - first_batch;
        while remaining_size > 0 {
            let to_read = remaining_size.min(buffer.len());
            let read = self.file.read(&mut buffer[..to_read])?;
            decoder.write_all(&buffer[..read])?;
            remaining_size -= read;
        }
        Ok(decoder.finish()?)
    }
}

// Upgrade chunks:
// java -jar .\server.jar --nogui --forceUpgrade
