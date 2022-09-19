use anyhow::{anyhow, Result};
use byteorder::{ReadBytesExt, BE};
use std::collections::HashMap;
use std::fmt::Write;
use std::io::Read;

#[derive(Debug)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Debug)]
pub struct RootTag {
    pub name: String,
    pub tag: Tag,
}

fn read_string<R: Read>(reader: &mut R) -> Result<String> {
    let size = reader.read_u16::<BE>()?;
    let mut vec = vec![0; size as usize];
    reader.read_exact(&mut vec)?;
    let s = String::from_utf8(vec)?;
    Ok(s)
}

fn read_impl<R: Read>(mut reader: &mut R, tag: u8) -> Result<Tag> {
    let result = match tag {
        1 => Tag::Byte(reader.read_i8()?),
        2 => Tag::Short(reader.read_i16::<BE>()?),
        3 => Tag::Int(reader.read_i32::<BE>()?),
        4 => Tag::Long(reader.read_i64::<BE>()?),
        5 => Tag::Float(reader.read_f32::<BE>()?),
        6 => Tag::Double(reader.read_f64::<BE>()?),
        7 => {
            let size = reader.read_i32::<BE>()?;
            let mut vec = vec![0; size as usize];
            reader.read_exact(&mut vec)?;
            Tag::ByteArray(vec)
        }
        8 => Tag::String(read_string(&mut reader)?),
        9 => {
            let kind = reader.read_u8()?;
            let size = reader.read_i32::<BE>()? as usize;
            let mut values = Vec::with_capacity(size);
            for _ in 0..size {
                let v = read_impl(reader, kind)?;
                values.push(v);
            }
            Tag::List(values)
        }
        10 => {
            let mut map = HashMap::new();
            loop {
                let kind = reader.read_u8()?;
                if kind == 0 {
                    break;
                }
                let name = read_string(reader)?;
                let tag = read_impl(reader, kind)?;
                map.insert(name, tag);
            }
            Tag::Compound(map)
        }
        11 => {
            let mut values = Vec::new();
            let size = reader.read_i32::<BE>()?;
            for _ in 0..size {
                values.push(reader.read_i32::<BE>()?);
            }
            Tag::IntArray(values)
        }
        12 => {
            let mut values = Vec::new();
            let size = reader.read_i32::<BE>()?;
            for _ in 0..size {
                values.push(reader.read_i64::<BE>()?);
            }
            Tag::LongArray(values)
        }
        _ => return Err(anyhow!("unknown tag {}", tag)),
    };
    Ok(result)
}

fn read_start<R: Read>(mut reader: R, tag: u8) -> Result<RootTag> {
    let reader = &mut reader;
    if tag != 10 {
        return Err(anyhow!(
            "expected the stream to start with a compound tag, found {}",
            tag
        ));
    }
    let name = read_string(reader)?;
    let tag = read_impl(reader, tag)?;
    Ok(RootTag { name, tag })
}

pub fn read_option<R: Read>(mut reader: R) -> Result<Option<RootTag>> {
    let reader = &mut reader;
    let tag = reader.read_u8()?;
    if tag == 0 {
        Ok(None)
    } else {
        let t = read_start(reader, tag)?;
        Ok(Some(t))
    }
}

pub fn read<R: Read>(mut reader: R) -> Result<RootTag> {
    let reader = &mut reader;
    let tag = reader.read_u8()?;
    read_start(reader, tag)
}

fn print_indent(output: &mut String, indent: usize) {
    let space = "                                                                ";
    output.push_str(&space[..indent]);
    // maybe don't have more than 64 spaces of indentation?
}

fn print_compound(output: &mut String, tag: &Tag, name: Option<&str>, indent: usize) -> Result<()> {
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

fn print_impl(output: &mut String, tag: &Tag, indent: usize) -> Result<()> {
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

pub fn pretty_print(root: &RootTag) -> Result<String> {
    let mut result = String::new();
    print_compound(&mut result, &root.tag, Some(&root.name), 0)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::nbt::{read, pretty_print, read_option};

    #[test]
    fn hello_world() {
        const DATA: &[u8] = include_bytes!("../../tests/hello_world.nbt");
        let tag = read(DATA).unwrap();
        pretty_print(&tag).unwrap();
    }

    #[test]
    fn option() {
        const DATA: &[u8] = &[0];
        let tag = read_option(DATA).unwrap();
        assert!(tag.is_none());
    }
}