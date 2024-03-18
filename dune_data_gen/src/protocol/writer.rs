use crate::protocol::Constant;

use super::Direction;
use super::State;
use super::Ty;
use super::TyBufferCountKind;
use super::TyEnum;
use super::TyKey;
use super::TyStruct;
use super::TypesMap;
use std::borrow::Cow;
use std::fmt::Arguments;

trait FmtWriteNoFail {
    fn write_fmt(&mut self, args: Arguments);
}
impl FmtWriteNoFail for String {
    fn write_fmt(&mut self, args: Arguments) {
        std::fmt::Write::write_fmt(self, args).expect("write on string failed 😢");
    }
}

fn lifetime(ty: TyKey, types: &TypesMap) -> &'static str {
    let ty = &types[ty];
    let b = ty.needs_lifetime(types)
        && !matches!(
            ty,
            Ty::String | Ty::Buffer(_) | Ty::RestBuffer | Ty::Slot | Ty::Nbt | Ty::OptionNbt
        );
    if b {
        "<'p>"
    } else {
        ""
    }
}
fn get_type_name<'x>(ty_key: TyKey, types: &'x TypesMap) -> Cow<'x, str> {
    let ty = &types[ty_key];

    match ty {
        Ty::Option(x) => format!(
            "Option<{}{}>",
            get_type_name(x.subtype, types),
            lifetime(x.subtype, types)
        )
        .into(),
        Ty::Array(x) => {
            if types[x.subtype].is_rs_builtin() {
                // should've made a type with this..
                format!(
                    "UnalignedSlice{}<'p>",
                    types[x.subtype].get_simple_type().to_uppercase()
                )
                .into()
            } else {
                format!(
                    "Vec<{}{}>",
                    get_type_name(x.subtype, types),
                    lifetime(x.subtype, types)
                )
                .into()
            }
        }
        Ty::Struct(x) => x.name.into(),
        Ty::Enum(x) => format!("{}{}", x.name, lifetime(ty_key, types)).into(),
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
    ty_key: TyKey,
    types: &TypesMap,
    bitfield_base_width: u16,
    count: u32,
) {
    let ty = &types[ty_key];
    if let Ty::Array(x) = ty {
        deserialize_one(
            out,
            "array_count",
            x.count_ty,
            types,
            bitfield_base_width,
            count + 1,
        );
        let subtype = &types[x.subtype];
        if subtype.is_rs_builtin() {
            write!(
                out,
                "let mem = reader.read_mem(array_count as usize * size_of::<{}>())?;
                let {} = UnalignedSlice{}::new(mem);",
                subtype.get_simple_type(),
                name,
                subtype.get_simple_type().to_uppercase(),
            );
            return;
        }

        write!(
            out,
            "let mut {} = Vec::with_capacity(cautious_size(array_count as usize));
            for _ in 0..array_count {{",
            name
        );
        let elem = if count == 1 {
            "x".to_string()
        } else {
            format!("x_{}", count)
        };
        deserialize_one(out, &elem, x.subtype, types, bitfield_base_width, count + 1);
        writeln!(out, "{}.push({}); }}", name, elem);
        return;
    }
    if let Ty::Enum(x) = ty {
        // TODO: get the correct type in parser so not do .into()
        write!(
            out,
            "let {}: {} = {}::deserialize(&mut reader, {}.into())?;",
            name,
            get_type_name(ty_key, types),
            x.name,
            x.compare_to
        );
        return;
    };
    write!(out, "let {}: {} = ", name, get_type_name(ty_key, types));
    if let Ty::Bitfield(x) = ty {
        let left_shift = bitfield_base_width - x.range_end;
        let right_shift = bitfield_base_width - (x.range_end - x.range_begin);
        write!(out, "(value << {} >> {}) as _;", left_shift, right_shift);

        return;
    }

    match ty {
        Ty::VarInt => *out += "read_varint(&mut reader)?;",
        Ty::VarLong => *out += "read_varlong(&mut reader)?;",
        Ty::RestBuffer => *out += "&reader[..]; *reader = &[];",
        Ty::Struct(x) => write!(out, "{}::deserialize(reader)?;", x.name),
        _ => *out += "MD::deserialize(reader)?;",
    }
    writeln!(out);
}
fn serialize_one(out: &mut String, name: &str, ty_key: TyKey, types: &TypesMap) {
    let ty = &types[ty_key];
    match ty {
        Ty::VarInt => write!(out, "write_varint(&mut writer, {} as u32)?;", name),
        Ty::VarLong => write!(out, "write_varlong(&mut writer, {} as u64)?;", name),
        Ty::RestBuffer => write!(out, "writer.write_all({})?;", name),
        _ => {
            write!(out, "{}.serialize(&mut writer)?;", name);
        }
    }
    writeln!(out);
}
fn underscore(b: bool) -> &'static str {
    if b {
        "_"
    } else {
        ""
    }
}
fn life(needs_lifetime: bool) -> (&'static str, &'static str) {
    if needs_lifetime {
        ("<'p>", "'p")
    } else {
        ("", "")
    }
}
fn serialize_struct(
    out: &mut String,
    ty_key: TyKey,
    types: &TypesMap,
    ty_struct: &TyStruct,
    name: &str,
) {
    let ty = &types[ty_key];
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
        }
        "#;
        return;
    }

    let (lifetime, lifetime_simple) = life(ty.needs_lifetime(types));
    writeln!(out, "#[derive(Debug)] pub struct {}{} {{", name, lifetime);

    for field in &ty_struct.fields {
        writeln!(
            out,
            "pub {}: {},",
            field.name,
            get_type_name(field.ty, types)
        );
    }

    *out += "}";

    write!(out, "impl<'p> MD<'p> for {}{lifetime} {{", ty_struct.name);

    writeln!(
        out,
        "fn deserialize(mut {}reader: &mut &{lifetime_simple}[u8]) -> Result<{}{lifetime}> {{",
        underscore(ty_struct.fields.is_empty()),
        ty_struct.name
    );

    if ty_struct.failed {
        *out += "// failed\n";
    }

    let bitfield_base_width = if let Some(base_type) = ty_struct.base_type {
        deserialize_one(out, "value", base_type, types, 0, 1);
        types[base_type].width()
    } else {
        0
    };
    for field in &ty_struct.fields {
        deserialize_one(out, field.name, field.ty, types, bitfield_base_width, 1);
    }

    write!(out, "\nlet result = {} {{", ty_struct.name);

    for field in &ty_struct.fields {
        writeln!(out, "{},", field.name);
    }

    *out += "}; Ok(result) }";

    let has_serialization = bitfield_base_width == 0
        && !ty_struct.fields.iter().any(|x| {
            let ty = &types[x.ty];
            matches!(ty, Ty::Array(_) | Ty::Struct(_) | Ty::Enum(_))
        });

    if has_serialization && !ty_struct.failed {
        write!(
            out,
            "fn serialize<W: Write>(&self, mut {}writer: &mut W) -> IoResult<()> {{",
            underscore(!has_serialization)
        );
        for field in &ty_struct.fields {
            serialize_one(out, &format!("self.{}", field.name), field.ty, types);
        }
        *out += "Ok(()) }";
    }

    *out += "}";
}
fn serialize_enum(out: &mut String, ty_key: TyKey, types: &TypesMap, ty_enum: &TyEnum, name: &str) {
    let ty = &types[ty_key];
    let (lifetime, lifetime_simple) = life(ty.needs_lifetime(types));
    writeln!(out, "#[derive(Debug)] pub enum {}{} {{", name, lifetime);

    for (_, variants) in ty_enum.variants.iter() {
        writeln!(out, "{} {{", variants.name);
        for i in variants.fields.iter() {
            writeln!(out, "{}: {},", i.name, get_type_name(i.ty, types));
        }
        writeln!(out, "}},");
    }

    let discriminator_type = ty_enum.discriminator_type;
    write!(out, "}} impl{lifetime} {name}{lifetime} {{
            fn deserialize(mut reader: &mut &{lifetime_simple} [u8], discriminator: {discriminator_type})
                     -> Result<{name}{lifetime}> {{
                let r = match discriminator {{
            ");

    for (constant, variants) in ty_enum.variants.iter() {
        match constant {
            Constant::Bool(x) => write!(out, "{}", x),
            Constant::Int(x) => write!(out, "{}", x),
            Constant::String(x) => write!(out, r#""{}""#, x),
        }
        writeln!(out, "=> {{");

        for i in variants.fields.iter() {
            deserialize_one(out, i.name, i.ty, types, 0, 1);
        }

        write!(out, "\n{}::{} {{", ty_enum.name, variants.name);

        for field in variants.fields.iter() {
            writeln!(out, "{},", field.name);
        }

        *out += "}}";
    }
    *out += "
    _ => todo!()
";

    *out += "}; Ok(r) }}";
}
fn write_all_structs(out: &mut String, ty_key: TyKey, types: &TypesMap) {
    let ty = &types[ty_key];
    match ty {
        Ty::Struct(x) => {
            for field in x.fields.iter() {
                write_all_structs(out, field.ty, types);
            }
            serialize_struct(out, ty_key, types, x, x.name);
        }
        Ty::Enum(x) => {
            for (_, v) in x.variants.iter() {
                for i in v.fields.iter() {
                    write_all_structs(out, i.ty, types);
                }
            }
            serialize_enum(out, ty_key, types, x, x.name);
        }
        Ty::Option(x) => {
            write_all_structs(out, x.subtype, types);
        }
        Ty::Array(x) => {
            write_all_structs(out, x.subtype, types);
        }
        _ => {}
    }
}

fn direction(out: &mut String, types: &TypesMap, direction: &Direction) {
    for packet in &direction.packets {
        write_all_structs(out, packet.ty, types);
    }
}

fn state(out: &mut String, types: &TypesMap, state: &State) {
    write!(
        out,
        "
        pub mod {} {{
            use super::*;
            
            ",
        state.kind.name(false)
    );

    direction(out, types, &state.c2s);
    direction(out, types, &state.s2c);

    *out += "}";
}

fn deserialize_fn(out: &mut String, states: &[State]) {
    *out += "
}
            
pub fn deserialize<'r>(state: ConnectionState, direction: PacketDirection, id: PacketId, reader: &mut &'r[u8]) -> Result<Packet<'r>> {
    use PacketDirection as D;
    use ConnectionState as S;
    
    let packet = match (state, direction, id.0) {
";

    for state in states {
        for (direction, direction_string) in [(&state.c2s, "C2S"), (&state.s2c, "S2C")] {
            for packet in &direction.packets {
                write!(
                    out,
                    "(S::{}, D::{}, {:#x}) => {{ let p = {}::{}::deserialize(reader)?; Packet::{}(p) }}",
                    state.kind.name(true),
                    direction_string,
                    packet.id,
                    state.kind.name(false),
                    packet.name,
                    packet.name
                );
            }
        }
    }

    *out += r#"_ => { return Err(anyhow!("unknown packet id={}", id)); } }; Ok(packet) }"#;
}

fn serialize_fn(out: &mut String, states: &[State]) {
    *out += "
            
pub fn serialize<'r, W: Write>(mut writer: &mut W, packet: Packet) -> IoResult<()> {
    match packet {
";

    for state in states {
        for direction in [&state.c2s, &state.s2c] {
            for packet in &direction.packets {
                write!(
                    out,
                    "Packet::{}(p) => {{ write_varint(&mut writer, {:#02x})?; p.serialize(writer) }}",
                    packet.name,
                    packet.id,
                );
            }
        }
    }

    *out += r#"}}"#;
}

pub(super) fn write(types: &TypesMap, mut states: [State; 1]) -> String {
    let mut out = String::with_capacity(4096);

    out += "
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(non_camel_case_types)]
// #![allow(unused_imports)]
// #![allow(clippy::needless_borrow)]
// #![allow(clippy::needless_borrows_for_generic_args)]
// #![allow(clippy::identity_op)]
// // fix
// #![allow(unreachable_code)]
// #![allow(unused_variables)]
// // fix

use crate::protocol::PacketId;
use crate::protocol::de::Position;
use crate::protocol::de::Vec3f64;
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
        state(&mut out, types, i);
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
                    lifetime(packet.ty, types),
                );
            }
        }
    }

    for i in &mut states {
        i.c2s.packets.sort_by_key(|x| x.id);
        i.s2c.packets.sort_by_key(|x| x.id);
    }
    deserialize_fn(&mut out, &states);
    serialize_fn(&mut out, &states);

    out
}
