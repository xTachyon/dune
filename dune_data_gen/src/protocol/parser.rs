use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
};

use bumpalo::Bump;
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use serde_derive::Deserialize;
use serde_json::Value;

use crate::protocol::{TyBuffer, TyBufferCountKind};

use super::{
    width_for_bitfields, ConnectionState, Direction, Packet, State, Ty, TyArray, TyBitfield,
    TyOption, TyStruct,
};

#[derive(Debug, Deserialize)]
struct JsonDirection {
    types: IndexMap<String, Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonState {
    to_client: JsonDirection,
    to_server: JsonDirection,
}

#[derive(Debug, Deserialize)]
struct Root {
    handshaking: JsonState,
    status: JsonState,
    login: JsonState,
    play: JsonState,
}

struct Parser<'x> {
    bump: &'x Bump,
    types: RefCell<HashSet<&'x Ty<'x>>>,

    ty_u8: &'x Ty<'x>,
    ty_u16: &'x Ty<'x>,
    ty_u128: &'x Ty<'x>,

    ty_i8: &'x Ty<'x>,
    ty_i16: &'x Ty<'x>,
    ty_i32: &'x Ty<'x>,
    ty_i64: &'x Ty<'x>,

    ty_f32: &'x Ty<'x>,
    ty_f64: &'x Ty<'x>,

    ty_bool: &'x Ty<'x>,
    ty_varint: &'x Ty<'x>,
    ty_varlong: &'x Ty<'x>,
    ty_string: &'x Ty<'x>,
    ty_buffer_with_varint: &'x Ty<'x>,
    ty_rest_buffer: &'x Ty<'x>,

    ty_position: &'x Ty<'x>,
    ty_slot: &'x Ty<'x>,
    ty_nbt: &'x Ty<'x>,
    ty_optional_nbt: &'x Ty<'x>,
    ty_chunk_block_entity: &'x Ty<'x>,
}
impl<'x> Parser<'x> {
    fn new(bump: &Bump) -> Parser {
        let ty_u8 = bump.alloc(Ty::U8);
        let ty_u16 = bump.alloc(Ty::U16);
        let ty_u128 = bump.alloc(Ty::U128);

        let ty_i8 = bump.alloc(Ty::I8);
        let ty_i16 = bump.alloc(Ty::I16);
        let ty_i32 = bump.alloc(Ty::I32);
        let ty_i64 = bump.alloc(Ty::I64);

        let ty_f32 = bump.alloc(Ty::F32);
        let ty_f64 = bump.alloc(Ty::F64);

        let ty_bool = bump.alloc(Ty::Bool);
        let ty_varint = bump.alloc(Ty::VarInt);
        let ty_varlong = bump.alloc(Ty::VarLong);
        let ty_string = bump.alloc(Ty::String);
        let ty_rest_buffer = bump.alloc(Ty::RestBuffer);
        let ty_buffer_with_varint = bump.alloc(Ty::Buffer(TyBuffer {
            kind: TyBufferCountKind::Varint,
        }));

        let ty_position = bump.alloc(Ty::Position);
        let ty_slot = bump.alloc(Ty::Slot);
        let ty_nbt = bump.alloc(Ty::Nbt);
        let ty_optional_nbt = bump.alloc(Ty::OptionNbt);
        let ty_chunk_block_entity = bump.alloc(Ty::ChunkBlockEntity);

        Parser {
            bump,
            types: RefCell::new(HashSet::new()),

            ty_u8,
            ty_u16,
            ty_u128,

            ty_i8,
            ty_i16,
            ty_i32,
            ty_i64,

            ty_f32,
            ty_f64,

            ty_bool,
            ty_varint,
            ty_varlong,
            ty_string,
            ty_buffer_with_varint,
            ty_rest_buffer,

            ty_position,
            ty_slot,
            ty_nbt,
            ty_optional_nbt,
            ty_chunk_block_entity,
        }
    }

    fn alloc_type<'a: 'x>(&self, ty: Ty<'a>) -> &'x Ty<'x> {
        let mut types = self.types.borrow_mut();
        match types.get(&ty) {
            Some(x) => x,
            None => {
                let r = self.bump.alloc(ty);
                types.insert(r);
                r
            }
        }
    }
}

fn snake_to_pascal(input: &str) -> String {
    assert!(input.is_ascii());

    let mut result = String::with_capacity(input.len());
    let mut last_is_underscore = true;
    for c in input.chars() {
        if last_is_underscore {
            last_is_underscore = false;
            result.push(c.to_ascii_uppercase());
        } else if c == '_' {
            last_is_underscore = true;
        } else {
            result.push(c);
        }
    }
    result
}

fn parse_container<'x>(
    parser: &Parser<'x>,
    input: &Value,
    struct_name: &str,
    parent_field: Option<&str>,
    is_bitfield: bool,
) -> Option<&'x Ty<'x>> {
    let mut fields = Vec::new();
    let mut failed = false;
    let mut bitfield_range = 0;

    for i in input.as_array().unwrap() {
        let name = i["name"].as_str().unwrap();
        let name = name.to_case(Case::Snake);
        let name = match name.as_str() {
            "type" | "match" => name + "_",
            _ => name,
        };

        let ty = if is_bitfield {
            let signed = i["signed"].as_bool().unwrap();
            let size = i["size"].as_u64().unwrap().try_into().unwrap();

            let ty = TyBitfield {
                range_begin: bitfield_range,
                range_end: bitfield_range + size,
                base_type_size: width_for_bitfields(size),
                unsigned: !signed,
            };
            bitfield_range += size;
            parser.alloc_type(Ty::Bitfield(ty))
        } else {
            let ty = &i["type"];

            match parse_type(parser, ty, struct_name, Some(&name)) {
                Some(x) => x,
                None => {
                    failed = true;
                    break;
                }
            }
        };

        fields.push((name, ty));
    }

    if failed {
        fields.clear();
    }
    let base_type = if bitfield_range == 0 {
        None
    } else {
        let ty = match bitfield_range {
            64 => parser.ty_i64,
            _ => unreachable!("unknown type with size={}", bitfield_range),
        };
        Some(ty)
    };

    let mut name = struct_name.to_string();
    if let Some(parent_field) = parent_field {
        name += "_";
        name += &snake_to_pascal(parent_field);
    }

    let t = Ty::Struct(TyStruct {
        name,
        fields,
        base_type,
        failed,
    });
    Some(parser.alloc_type(t))
}
fn parse_option<'x>(
    parser: &Parser<'x>,
    input: &Value,
    struct_name: &str,
    parent_field: Option<&str>,
) -> Option<&'x Ty<'x>> {
    let subtype = parse_type(parser, input, struct_name, parent_field)?;
    let t = Ty::Option(TyOption { subtype });
    Some(parser.alloc_type(t))
}
fn parse_buffer<'x>(parser: &Parser<'x>, input: &Value) -> &'x Ty<'x> {
    let arg1 = &input[1];

    let kind = if let Value::String(x) = &arg1["countType"] {
        assert_eq!(x, "varint");
        TyBufferCountKind::Varint
    } else if let Value::Number(x) = &arg1["count"] {
        let count = x.as_u64().unwrap().try_into().unwrap();
        TyBufferCountKind::Fixed(count)
    } else {
        panic!("unknown buffer kind");
    };

    let t = Ty::Buffer(TyBuffer { kind });
    parser.alloc_type(t)
}
fn parse_array<'x>(
    parser: &Parser<'x>,
    input: &Value,
    struct_name: &str,
    parent_field: Option<&str>,
) -> Option<&'x Ty<'x>> {
    let count_ty = &input["countType"];
    let count_ty = parse_type(parser, count_ty, struct_name, parent_field)?;

    let subtype = &input["type"];
    let subtype = parse_type(parser, subtype, struct_name, parent_field)?;

    let t = if *count_ty == Ty::VarInt && *subtype == Ty::U8 {
        parser.ty_buffer_with_varint
    } else {
        let t = Ty::Array(TyArray { count_ty, subtype });
        parser.alloc_type(t)
    };

    Some(t)
}
fn parse_type_simple<'x>(
    parser: &Parser<'x>,
    input: &str,
    struct_name: &str,
) -> Option<&'x Ty<'x>> {
    let r = match input {
        "u8" => parser.ty_u8,
        "u16" => parser.ty_u16,
        "UUID" => parser.ty_u128,

        "i8" => parser.ty_i8,
        "i16" => parser.ty_i16,
        "i32" => parser.ty_i32,
        "i64" => parser.ty_i64,

        "f32" => parser.ty_f32,
        "f64" => parser.ty_f64,

        "bool" => parser.ty_bool,
        "varint" => parser.ty_varint,
        "varlong" => parser.ty_varlong,
        "string" => parser.ty_string,
        "restBuffer" => parser.ty_rest_buffer,

        "position" => parser.ty_position,
        "slot" => parser.ty_slot,
        "nbt" => parser.ty_nbt,
        "optionalNbt" => parser.ty_optional_nbt,
        "chunkBlockEntity" => parser.ty_chunk_block_entity,

        _ => {
            eprintln!("unknown type `{}` in `{}`", input, struct_name);
            return None;
        }
    };
    Some(r)
}
fn parse_type<'x>(
    parser: &Parser<'x>,
    input: &Value,
    struct_name: &str,
    parent_field: Option<&str>,
) -> Option<&'x Ty<'x>> {
    if let Some(x) = input.as_str() {
        return parse_type_simple(parser, x, struct_name);
    }

    let name = input[0].as_str().unwrap();
    let arg1 = &input[1];
    match name {
        "container" => parse_container(parser, arg1, struct_name, parent_field, false),
        "bitfield" => parse_container(parser, arg1, struct_name, parent_field, true),
        "option" => parse_option(parser, arg1, struct_name, parent_field),
        "buffer" => Some(parse_buffer(parser, input)),
        "array" => parse_array(parser, arg1, struct_name, parent_field),
        _ => {
            eprintln!("unknown type `{}` in `{}`", name, struct_name);
            None
        }
    }
}

fn do_mapping(input: &Value) -> HashMap<String, u16> {
    let input = &input[1];
    let input = &input[0];
    let input = &input["type"];
    let input = &input[1];
    let input = &input["mappings"];
    let mut mappings = HashMap::new();

    for (id, name) in input.as_object().unwrap() {
        let id = id.strip_prefix("0x").unwrap();
        let id: u16 = u16::from_str_radix(id, 16).unwrap();
        let name = name.as_str().unwrap();

        mappings.insert(name.to_string(), id);
    }
    mappings
}
fn direction<'x>(
    parser: &Parser<'x>,
    mut direction: JsonDirection,
    kind: &str,
    state_kind: ConnectionState,
) -> Direction<'x> {
    let raw_mappings = do_mapping(&direction.types.remove("packet").unwrap());
    let mut packets = Vec::with_capacity(direction.types.len());

    for (name, value) in direction.types {
        let is_ignored = name == "packet_advancements";
        let name = name.strip_prefix("packet_").unwrap().to_string();
        let id = raw_mappings[&name];
        let name = if state_kind == ConnectionState::Play && name == "ping" {
            "play_".to_string() + &name
        } else {
            name
        };
        let name = if name.ends_with("_request") || name.ends_with("_response") {
            name
        } else {
            name + kind
        };
        let name = snake_to_pascal(&name);
        let ty = if is_ignored {
            parser.alloc_type(Ty::Struct(TyStruct {
                name: name.clone(),
                fields: Vec::new(),
                base_type: None,
                failed: true,
            }))
        } else {
            match parse_type(parser, &value, &name, None) {
                Some(x) => x,
                None => {
                    eprintln!("---\ncan't parse {}", name);
                    continue;
                }
            }
        };

        packets.push(Packet { ty, name, id });
    }

    Direction { packets }
}

fn state<'x>(parser: &Parser<'x>, state: JsonState, kind: ConnectionState) -> State<'x> {
    State {
        kind,
        c2s: direction(parser, state.to_server, "_request", kind),
        s2c: direction(parser, state.to_client, "_response", kind),
    }
}

pub(super) fn parse<'x>(path: &str, bump: &'x Bump) -> [State<'x>; 4] {
    let content = fs::read_to_string(path).unwrap();
    let root: Root = serde_json::from_str(&content).unwrap();

    let parser = Parser::new(bump);

    [
        state(&parser, root.handshaking, ConnectionState::Handshaking),
        state(&parser, root.status, ConnectionState::Status),
        state(&parser, root.login, ConnectionState::Login),
        state(&parser, root.play, ConnectionState::Play),
    ]
}
