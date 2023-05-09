use super::Direction;
use super::Packet;
use super::State;
use super::Ty;
use super::TyBufferCountKind;
use super::TyStruct;
use std::borrow::Cow;
use std::fmt::Write;

type Result<T> = std::result::Result<T, std::fmt::Error>;

fn lifetime(ty: &Ty) -> &'static str {
    let b = ty.needs_lifetime()
        && !matches!(
            ty,
            Ty::String | Ty::Buffer(_) | Ty::RestBuffer | Ty::Slot | Ty::NBT | Ty::OptionNBT
        );
    if b {
        "<'p>"
    } else {
        ""
    }
}
fn get_type_name<'x>(ty: &'x Ty) -> Cow<'x, str> {
    match ty {
        Ty::Option(x) => format!(
            "Option<{}{}>",
            get_type_name(x.subtype),
            lifetime(x.subtype)
        )
        .into(),
        Ty::Array(x) => {
            if x.subtype.is_rs_builtin() {
                // should've made a type with this..
                format!(
                    "UnalignedSlice{}<'p>",
                    x.subtype.get_simple_type().to_uppercase()
                )
                .into()
            } else {
                format!("Vec<{}{}>", get_type_name(x.subtype), lifetime(x.subtype)).into()
            }
        }
        Ty::Struct(x) => x.name.as_str().into(),
        Ty::Buffer(x) => match x.kind {
            TyBufferCountKind::Fixed(count) => format!("&'p [u8; {}]", count).into(),
            TyBufferCountKind::Varint => "&'p [u8]".into(),
        },
        _ => ty.get_simple_type().into(),
    }
}
fn deserialize_one(
    out: &mut String,
    name: &str,
    ty: &Ty,
    bitfield_base_width: u16,
    count: u32,
) -> Result<()> {
    if let Ty::Array(x) = ty {
        deserialize_one(
            out,
            "array_count",
            x.count_ty,
            bitfield_base_width,
            count + 1,
        )?;
        if x.subtype.is_rs_builtin() {
            write!(
                out,
                "let mem = reader.read_mem(array_count as usize * size_of::<{}>())?;
                let {} = UnalignedSlice{}::new(mem);",
                x.subtype.get_simple_type(),
                name,
                x.subtype.get_simple_type().to_uppercase(),
            )?;
            return Ok(());
        }

        write!(
            out,
            "let mut {} = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {{",
            name
        )?;
        let elem = if count == 1 {
            "x".to_string()
        } else {
            format!("x_{}", count)
        };
        deserialize_one(out, &elem, x.subtype, bitfield_base_width, count + 1)?;
        writeln!(out, "{}.push({}); }}", name, elem)?;
        return Ok(());
    }
    write!(out, "let {}: {} = ", name, get_type_name(ty))?;
    if let Ty::Bitfield(x) = ty {
        let left_shift = bitfield_base_width - x.range_end;
        let right_shift = bitfield_base_width - (x.range_end - x.range_begin);
        write!(out, "(value << {} >> {}) as _;", left_shift, right_shift)?;

        return Ok(());
    }

    match ty {
        Ty::VarInt => *out += "read_varint(&mut reader)?;",
        Ty::VarLong => *out += "read_varlong(&mut reader)?;",
        Ty::RestBuffer => *out += "&reader[..]; *reader = &[];",
        Ty::Struct(x) => write!(out, "{}::deserialize(reader)?;", x.name)?,
        _ => *out += "MD::deserialize(reader)?;",
    }
    writeln!(out)?;

    Ok(())
}
fn serialize_one(out: &mut String, name: &str, ty: &Ty) -> Result<()> {
    match ty {
        Ty::VarInt => write!(out, "write_varint(&mut writer, {} as u32)?;", name)?,
        Ty::VarLong => write!(out, "write_varlong(&mut writer, {} as u64)?;", name)?,
        Ty::RestBuffer => write!(out, "writer.write_all({})?;", name)?,
        _ => {
            write!(out, "{}.serialize(&mut writer)?;", name)?;
        }
    }
    writeln!(out)?;

    Ok(())
}
fn underscore(b: bool) -> &'static str {
    if b {
        "_"
    } else {
        ""
    }
}
fn serialize_struct(
    out: &mut String,
    ty: &Ty,
    ty_struct: &TyStruct,
    name: &str,
    id: Option<u16>,
) -> Result<()> {
    // TODO:
    if name == "UseEntityRequest" {
        *out += r#"
        #[derive(Debug)]
        pub struct Coords {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }
        #[derive(Debug)]
        pub enum UseEntityKind {
            Interact,
            Attack,
            InteractAt(Coords),
        }
        
        #[derive(Debug)]
        pub struct UseEntityRequest {
            pub entity_id: i32,
            pub kind: UseEntityKind,
            pub sneaking: bool,
        }
        
        impl<'p> MD<'p> for UseEntityRequest {
            fn deserialize(mut reader: &mut &[u8]) -> Result<UseEntityRequest> {
                let entity_id = read_varint(&mut reader)?;
                let kind = read_varint(&mut reader)?;
                let kind = match kind {
                    0 => {
                        let _ = read_varint(&mut reader)?;
                        UseEntityKind::Interact
                    }
                    1 => UseEntityKind::Attack,
                    2 => {
                        let x = MD::deserialize(&mut reader)?;
                        let y = MD::deserialize(&mut reader)?;
                        let z = MD::deserialize(&mut reader)?;
                        let _ = read_varint(&mut reader)?;
    
                        UseEntityKind::InteractAt(Coords { x, y, z })
                    }
                    _ => anyhow::bail!("unknown use entity kind {}", kind),
                };
                let sneaking = MD::deserialize(&mut reader)?;
    
                Ok(UseEntityRequest {
                    entity_id,
                    kind,
                    sneaking,
                })
            }
            fn serialize<W: Write>(&self, mut writer: &mut W) -> IoResult<()> {
                unimplemented!()
            }
        }
        "#;
        return Ok(());
    }

    let (lifetime, lifetime_simple) = if ty.needs_lifetime() {
        ("<'p>", "'p")
    } else {
        ("", "")
    };
    writeln!(out, "#[derive(Debug)] pub struct {}{} {{", name, lifetime)?;

    for (name, ty) in &ty_struct.fields {
        writeln!(out, "pub {}: {},", name, get_type_name(ty))?;
    }

    *out += "}";

    write!(out, "impl<'p> MD<'p> for {}{lifetime} {{", ty_struct.name,)?;

    writeln!(
        out,
        "fn deserialize(mut {}reader: &mut &{lifetime_simple}[u8]) -> Result<{}{lifetime}> {{",
        underscore(ty_struct.fields.is_empty()),
        ty_struct.name
    )?;

    if ty_struct.failed {
        *out += "// failed\n";
    }

    let bitfield_base_width = if let Some(base_type) = ty_struct.base_type {
        deserialize_one(out, "value", base_type, 0, 1)?;
        base_type.width()
    } else {
        0
    };
    for (name, ty) in &ty_struct.fields {
        deserialize_one(out, name, ty, bitfield_base_width, 1)?;
    }

    write!(out, "\nlet result = {} {{", ty_struct.name)?;

    for (name, _) in &ty_struct.fields {
        writeln!(out, "{},", name)?;
    }

    *out += "}; Ok(result) }";

    let has_serialization = bitfield_base_width == 0
        && !ty_struct
            .fields
            .iter()
            .any(|x| matches!(x.1, Ty::Array(_) | Ty::Struct(_)));
    write!(
        out,
        "fn serialize<W: Write>(&self, mut {}writer: &mut W) -> IoResult<()> {{",
        underscore(!has_serialization)
    )?;

    if has_serialization {
        if let Some(id) = id {
            write!(out, "write_varint(&mut writer, {:#02x})?;", id)?;
        }
        for (name, ty) in &ty_struct.fields {
            serialize_one(out, &format!("self.{}", name), ty)?;
        }
        *out += "Ok(())";
    } else {
        *out += "unimplemented!();";
    }

    *out += "}}";

    Ok(())
}
fn write_all_structs(out: &mut String, ty: &Ty, id: Option<u16>) -> Result<()> {
    match ty {
        Ty::Struct(x) => {
            for (_, ty_struct) in x.fields.iter() {
                write_all_structs(out, ty_struct, None)?;
            }
            serialize_struct(out, ty, x, &x.name, id)?;
        }
        Ty::Option(x) => {
            write_all_structs(out, x.subtype, None)?;
        }
        Ty::Array(x) => {
            write_all_structs(out, x.subtype, None)?;
        }
        _ => {}
    }
    Ok(())
}

fn direction(out: &mut String, direction: &Direction) -> Result<()> {
    for packet in &direction.packets {
        write_all_structs(out, packet.ty, Some(packet.id))?;
    }

    Ok(())
}

fn state(out: &mut String, state: &State) -> Result<()> {
    write!(
        out,
        "
        pub mod {} {{
            use super::*;
            
            ",
        state.kind.name(false)
    )?;

    direction(out, &state.c2s)?;
    direction(out, &state.s2c)?;

    *out += "}";

    Ok(())
}

pub(super) fn write(states: [State; 4]) -> Result<String> {
    let mut out = String::with_capacity(4096);

    out += "
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(clippy::needless_borrow)]
// fix
#![allow(unreachable_code)]
#![allow(unused_variables)]
// fix

use crate::protocol::de::Position;
use crate::protocol::de::MD;
use crate::protocol::de::cautious_size;
use crate::protocol::varint::read_varint;
use crate::protocol::varint::read_varlong;
use crate::protocol::varint::write_varint;
use crate::protocol::varint::write_varlong;
use crate::protocol::ConnectionState;
use crate::protocol::IndexedNbt;
use crate::protocol::IndexedOptionNbt;
use crate::protocol::InventorySlot;
use crate::protocol::PacketDirection;
use crate::protocol::ChunkBlockEntity;
use crate::protocol::UnalignedSliceI64;
use crate::protocol::UnalignedSliceU128;
use super::de::MemoryExt;
use anyhow::{anyhow, Result};
use std::io::{Result as IoResult, Write};
use std::mem::size_of;
    ";
    for i in states.iter() {
        state(&mut out, i)?;
    }

    out += "#[derive(Debug)] pub enum Packet<'p> {";
    for state in &states {
        for direction in [&state.c2s, &state.s2c] {
            for packet in &direction.packets {
                write!(
                    &mut out,
                    "{0}({1}::{0}{2}),",
                    packet.name,
                    state.kind.name(false),
                    lifetime(packet.ty),
                )?;
            }
        }
    }
    out += "
}
            
pub fn deserialize<'r>(state: ConnectionState, direction: PacketDirection, id: u32, reader: &mut &'r[u8]) -> Result<Packet<'r>> {
    use PacketDirection as D;
    use ConnectionState as S;
    
    let packet = match (state, direction, id) {
";

    for state in states {
        for (direction, direction_string) in [(state.c2s, "C2S"), (state.s2c, "S2C")] {
            let mut packets: Vec<Packet> = direction.packets.into_iter().collect();
            packets.sort_unstable_by_key(|x| x.id);

            for packet in packets {
                write!(
                    out,
                    "(S::{}, D::{}, {:#x}) => {{ let p = {}::{}::deserialize(reader)?; Packet::{}(p) }}",
                    state.kind.name(true),
                    direction_string,
                    packet.id,
                    state.kind.name(false),
                    packet.name,
                    packet.name
                )?;
            }
        }
    }

    out += r#"_ => { return Err(anyhow!("unknown packet id={}", id)); } }; Ok(packet) }"#;

    Ok(out)
}
