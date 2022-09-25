use anyhow::{anyhow, Result};
use bumpalo::collections::Vec;
use bumpalo::Bump;
use byteorder::{ReadBytesExt, BE};
use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::io::Read;
use std::str;

#[derive(Debug)]
pub enum Tag<'n> {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(&'n [u8]),
    String(&'n str),
    List(Vec<'n, Tag<'n>>),
    Compound(HashMap<&'n str, Tag<'n>>),
    // TODO: use HashMap with bumpalo?
    IntArray(Vec<'n, i32>),
    LongArray(Vec<'n, i64>),
}

#[derive(Debug)]
pub struct RootTag<'n> {
    pub name: &'n str,
    pub tag: Tag<'n>,
}

impl<'n> Display for RootTag<'n> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult<()> {
        f.write_str(&pretty_print(self)?)
    }
}

// read

fn read_string<'n, R: Read>(reader: &mut R, bump: &'n Bump) -> Result<&'n str> {
    let size = reader.read_u16::<BE>()?;
    let result = bump.alloc_slice_fill_default(size as usize);
    reader.read_exact(result)?;
    let s = str::from_utf8(result)?;
    Ok(s)
}

fn read_impl<'n, R: Read>(mut reader: &mut R, tag: u8, bump: &'n Bump) -> Result<Tag<'n>> {
    let result = match tag {
        1 => Tag::Byte(reader.read_i8()?),
        2 => Tag::Short(reader.read_i16::<BE>()?),
        3 => Tag::Int(reader.read_i32::<BE>()?),
        4 => Tag::Long(reader.read_i64::<BE>()?),
        5 => Tag::Float(reader.read_f32::<BE>()?),
        6 => Tag::Double(reader.read_f64::<BE>()?),
        7 => {
            let size = reader.read_i32::<BE>()? as usize;
            let bytes = bump.alloc_slice_fill_default(size);
            reader.read_exact(bytes)?;
            Tag::ByteArray(bytes)
        }
        8 => Tag::String(read_string(&mut reader, bump)?),
        9 => {
            let kind = reader.read_u8()?;
            let size = reader.read_i32::<BE>()? as usize;
            let mut values = Vec::with_capacity_in(size, bump);
            for _ in 0..size {
                let v = read_impl(reader, kind, bump)?;
                values.push(v);
            }
            Tag::List(values)
        }
        10 => {
            let mut map = HashMap::new();
            loop {
                let kind = reader.read_u8()?;
                if kind == 0 {
                    // tag_end
                    break;
                }
                let name = read_string(reader, bump)?;
                let tag = read_impl(reader, kind, bump)?;
                map.insert(name, tag);
            }
            Tag::Compound(map)
        }
        11 => {
            let size = reader.read_i32::<BE>()? as usize;
            let mut values = Vec::with_capacity_in(size, bump);
            for _ in 0..size {
                values.push(reader.read_i32::<BE>()?);
            }
            Tag::IntArray(values)
        }
        12 => {
            let size = reader.read_i32::<BE>()? as usize;
            let mut values = Vec::with_capacity_in(size, bump);
            for _ in 0..size {
                values.push(reader.read_i64::<BE>()?);
            }
            Tag::LongArray(values)
        }
        _ => return Err(anyhow!("unknown tag {}", tag)),
    };
    Ok(result)
}

fn read_start<R: Read>(mut reader: R, tag: u8, bump: &Bump) -> Result<RootTag> {
    let reader = &mut reader;
    if tag != 10 {
        return Err(anyhow!(
            "expected the stream to start with a compound tag, found {}",
            tag
        ));
    }
    let name = read_string(reader, bump)?;
    let tag = read_impl(reader, tag, bump)?;
    Ok(RootTag { name, tag })
}

pub fn read_option<R: Read>(mut reader: R, bump: &Bump) -> Result<Option<RootTag>> {
    let reader = &mut reader;
    let tag = reader.read_u8()?;
    if tag == 0 {
        Ok(None)
    } else {
        let t = read_start(reader, tag, bump)?;
        Ok(Some(t))
    }
}

pub fn read<R: Read>(mut reader: R, bump: &Bump) -> Result<RootTag> {
    let reader = &mut reader;
    let tag = reader.read_u8()?;
    read_start(reader, tag, bump)
}

// skip

struct Skipper<'r, R: Read> {
    tmp_buf: [u8; 1024],
    reader: &'r mut R,
}

impl<'r, R: Read> Skipper<'r, R> {
    fn skip_buf(&mut self, mut size: usize) -> Result<()> {
        while size > 0 {
            let to_read = size.min(self.tmp_buf.len());
            let read = self.reader.read(&mut self.tmp_buf[..to_read])?;
            size -= read;
        }
        Ok(())
    }

    fn skip_string(&mut self) -> Result<()> {
        let size = self.reader.read_u16::<BE>()? as usize;
        self.skip_buf(size)
    }

    fn skip_impl(&mut self, tag: u8) -> Result<()> {
        match tag {
            1 => self.skip_buf(1)?, // tag_byte
            2 => self.skip_buf(2)?, // tag_short
            3 => self.skip_buf(4)?, // tag_int
            4 => self.skip_buf(8)?, // tag_long
            5 => self.skip_buf(4)?, // tag_float
            6 => self.skip_buf(8)?, // tag_double
            7 => {
                // tag_byte_array
                let size = self.reader.read_i32::<BE>()? as usize;
                self.skip_buf(size)?;
            }
            8 => {
                // tag_string
                self.skip_string()?;
            }
            9 => {
                // tag_list
                let kind = self.reader.read_u8()?;
                let size = self.reader.read_i32::<BE>()?;
                for _ in 0..size {
                    self.skip_impl(kind)?;
                }
            }
            10 => {
                // tag_compound
                loop {
                    let kind = self.reader.read_u8()?;
                    if kind == 0 {
                        // tag_end
                        break;
                    }
                    self.skip_string()?;
                    self.skip_impl(kind)?;
                }
            }
            11 => {
                // tag_int_array
                let size = self.reader.read_i32::<BE>()?;
                for _ in 0..size {
                    self.skip_buf(4)?;
                }
            }
            12 => {
                // tag_long_array
                let size = self.reader.read_i32::<BE>()?;
                for _ in 0..size {
                    self.skip_buf(8)?;
                }
            }
            _ => return Err(anyhow!("unknown tag {}", tag)),
        }
        Ok(())
    }
}

fn skip_start<R: Read>(reader: &mut R, tag: u8) -> Result<()> {
    if tag != 10 {
        return Err(anyhow!(
            "expected the stream to start with a compound tag, found {}",
            tag
        ));
    }
    let mut skipper = Skipper {
        tmp_buf: [0; 1024],
        reader,
    };
    skipper.skip_string()?;
    skipper.skip_impl(tag)?;
    Ok(())
}

pub(crate) fn skip_option<R: Read>(mut reader: R) -> Result<bool> {
    let reader = &mut reader;
    let tag = reader.read_u8()?;
    let r = if tag == 0 {
        false
    } else {
        skip_start(reader, tag)?;
        true
    };
    Ok(r)
}

pub(crate) fn skip<R: Read>(mut reader: R) -> Result<()> {
    let reader = &mut reader;
    let tag = reader.read_u8()?;
    skip_start(reader, tag)
}

// print

fn print_indent(output: &mut String, indent: usize) {
    let space = "                                                                ";
    output.push_str(&space[..indent]);
    // maybe don't have more than 64 spaces of indentation?
}

type FmtResult<T> = std::result::Result<T, std::fmt::Error>;

fn print_compound(
    output: &mut String,
    tag: &Tag,
    name: Option<&str>,
    indent: usize,
) -> FmtResult<()> {
    let map = match tag {
        Tag::Compound(x) => x,
        _ => unreachable!(),
    };
    let entries = if map.len() == 1 { "entry" } else { "entries" };
    match name {
        Some(x) => write!(output, "'{}' -> {} {}", x, map.len(), entries)?,
        None => write!(output, "{} {}", map.len(), entries)?,
    }
    *output += " {";

    for (key, value) in map {
        output.push('\n');
        print_indent(output, indent + 4);
        write!(output, "'{}' -> ", key)?;
        print_impl(output, value, indent + 4)?;
    }
    output.push('\n');
    print_indent(output, indent);
    output.push('}');

    Ok(())
}

fn print_impl(output: &mut String, tag: &Tag, indent: usize) -> FmtResult<()> {
    match tag {
        Tag::Byte(x) => write!(output, "{}b", x)?,
        Tag::Short(x) => write!(output, "{}s", x)?,
        Tag::Int(x) => write!(output, "{}i", x)?,
        Tag::Long(x) => write!(output, "{}l", x)?,
        Tag::Float(x) => write!(output, "{}f", x)?,
        Tag::Double(x) => write!(output, "{}d", x)?,
        Tag::ByteArray(x) => write!(output, "{} bytes", x.len())?,
        Tag::String(x) => write!(output, "'{}'", x)?,
        Tag::List(x) => {
            *output += "[\n";
            for i in x {
                print_indent(output, indent + 4);
                print_impl(output, i, indent + 4)?;
                output.push('\n');
            }
            print_indent(output, indent);
            *output += "]";
        }
        Tag::Compound(_) => print_compound(output, tag, None, indent)?,
        Tag::IntArray(x) => write!(output, "{} ints", x.len())?,
        Tag::LongArray(x) => write!(output, "{} longs", x.len())?,
    }
    Ok(())
}

fn pretty_print(root: &RootTag) -> FmtResult<String> {
    let mut result = String::new();
    print_compound(&mut result, &root.tag, Some(root.name), 0)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::nbt::{read, read_option};
    use bumpalo::Bump;

    #[test]
    fn hello_world() {
        const DATA: &[u8] = include_bytes!("../../tests/hello_world.nbt");
        let bump = Bump::new();

        let tag = read(DATA, &bump).unwrap();
        let _ = tag.to_string();
    }

    #[test]
    fn option() {
        const DATA: &[u8] = &[0];
        let bump = Bump::new();

        let tag = read_option(DATA, &bump).unwrap();
        assert!(tag.is_none());
    }
}
