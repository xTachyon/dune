use super::{
    width_for_bitfields, ConnectionState, Direction, Packet, State, Ty, TyArray, TyBitfield, TyKey,
    TyOption, TyStruct, TyStructField, TypesMap,
};
use crate::{
    protocol::{Constant, TyBuffer, TyBufferCountKind, TyEnum, VariantField, Variants},
    read_file,
};
use bumpalo::Bump;
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use serde_derive::Deserialize;
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::{Path, PathBuf},
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
    // handshaking: JsonState,
    // status: JsonState,
    // login: JsonState,
    play: JsonState,
}

struct Parser<'y, 'x> {
    types: &'y mut TypesMap<'x>,
    unknown_types: HashMap<&'x str, Vec<&'x str>>,
    strs: HashSet<&'x str>,

    ty_u8: TyKey,
    ty_u16: TyKey,
    ty_u128: TyKey,

    ty_i8: TyKey,
    ty_i16: TyKey,
    ty_i32: TyKey,
    ty_i64: TyKey,

    ty_f32: TyKey,
    ty_f64: TyKey,

    ty_bool: TyKey,
    ty_varint: TyKey,
    ty_varlong: TyKey,
    ty_string: TyKey,
    ty_buffer_with_varint: TyKey,
    ty_rest_buffer: TyKey,

    ty_position: TyKey,
    ty_slot: TyKey,
    ty_nbt: TyKey,
    ty_optional_nbt: TyKey,
    ty_chunk_block_entity: TyKey,
    ty_vec3f64: TyKey,
}
impl<'y, 'x> Parser<'y, 'x> {
    fn new(types: &'y mut TypesMap<'x>) -> Parser<'y, 'x> {
        let ty_u8 = types.insert(Ty::U8);
        let ty_u16 = types.insert(Ty::U16);
        let ty_u128 = types.insert(Ty::U128);

        let ty_i8 = types.insert(Ty::I8);
        let ty_i16 = types.insert(Ty::I16);
        let ty_i32 = types.insert(Ty::I32);
        let ty_i64 = types.insert(Ty::I64);

        let ty_f32 = types.insert(Ty::F32);
        let ty_f64 = types.insert(Ty::F64);

        let ty_bool = types.insert(Ty::Bool);
        let ty_varint = types.insert(Ty::VarInt);
        let ty_varlong = types.insert(Ty::VarLong);
        let ty_string = types.insert(Ty::String);
        let ty_rest_buffer = types.insert(Ty::RestBuffer);
        let ty_buffer_with_varint = types.insert(Ty::Buffer(TyBuffer {
            kind: TyBufferCountKind::Varint,
        }));

        let ty_position = types.insert(Ty::Position);
        let ty_slot = types.insert(Ty::Slot);
        let ty_nbt = types.insert(Ty::Nbt);
        let ty_optional_nbt = types.insert(Ty::OptionNbt);
        let ty_chunk_block_entity = types.insert(Ty::ChunkBlockEntity);
        let ty_vec3f64 = types.insert(Ty::Vec3f64);

        Parser {
            types,
            unknown_types: HashMap::with_capacity(32),
            strs: HashSet::with_capacity(32),

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
            ty_vec3f64,
        }
    }

    fn alloc_type<'a: 'x>(&mut self, ty: Ty<'a>) -> TyKey {
        self.types.insert(ty)
    }

    fn alloc_str<S: AsRef<str>>(&mut self, bump: &'x Bump, x: S) -> &'x str {
        fn inner<'x>(parser: &mut Parser<'_, 'x>, bump: &'x Bump, x: &str) -> &'x str {
            match parser.strs.get(x) {
                Some(x) => x,
                None => {
                    let r = bump.alloc_str(x);
                    parser.strs.insert(r);
                    r
                }
            }
        }
        inner(self, bump, x.as_ref())
    }

    fn add_unknown_type(&mut self, bump: &'x Bump, unk_ty: &str, packet_ty: &str) {
        let unk_ty = self.alloc_str(bump, unk_ty);
        let packet_ty = self.alloc_str(bump, packet_ty);

        self.unknown_types
            .entry(unk_ty)
            .or_default()
            .push(packet_ty);
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

#[derive(Debug)]
struct ParentData<'x> {
    parent_struct_name: &'x str,
    parent_field: Option<&'x str>,
    last_type: Option<TyKey>,
    switch_updated: bool,
}

fn parse_container<'x>(
    parser: &mut Parser<'_, 'x>,
    bump: &'x Bump,
    input: &Value,
    parent: &ParentData,
    is_bitfield: bool,
) -> Option<TyKey> {
    let mut fields: Vec<TyStructField> = Vec::new();
    let mut failed = false;
    let mut bitfield_range = 0;

    for i in input.as_array().unwrap() {
        let name = i["name"].as_str().unwrap();
        let name = name.to_case(Case::Snake);
        let name = match name.as_str() {
            "type" | "match" => name + "_",
            _ => name,
        };
        let name = parser.alloc_str(bump, name);

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
            let mut parent = ParentData {
                parent_struct_name: parent.parent_struct_name,
                parent_field: Some(name),
                last_type: fields.last().map(|x| x.ty),
                switch_updated: false,
            };

            match parse_type(parser, bump, ty, &mut parent) {
                Some(_) if parent.switch_updated => continue,
                Some(x) => x,
                None => {
                    failed = true;
                    break;
                }
            }
        };

        fields.push(TyStructField { name, ty });
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

    let mut name = parent.parent_struct_name.to_string();
    if let Some(parent_field) = parent.parent_field {
        name += "_";
        name += &snake_to_pascal(parent_field);
    }
    let name = bump.alloc_str(&name);

    let t = Ty::Struct(TyStruct {
        name,
        fields,
        base_type,
        failed,
    });
    Some(parser.alloc_type(t))
}
fn parse_option<'x>(
    parser: &mut Parser<'_, 'x>,
    bump: &'x Bump,
    input: &Value,
    parent: &mut ParentData,
) -> Option<TyKey> {
    let subtype = parse_type(parser, bump, input, parent)?;
    let t = Ty::Option(TyOption { subtype });
    Some(parser.alloc_type(t))
}
fn parse_buffer(parser: &mut Parser<'_, '_>, input: &Value) -> TyKey {
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
    parser: &mut Parser<'_, 'x>,
    bump: &'x Bump,
    input: &Value,
    parent: &mut ParentData,
) -> Option<TyKey> {
    let count_ty = &input["countType"];
    let count_ty = parse_type(parser, bump, count_ty, parent)?;

    let subtype = &input["type"];
    let subtype = parse_type(parser, bump, subtype, parent)?;

    let count_ty_val = &parser.types[count_ty];
    let subtype_val = &parser.types[subtype];

    let t = if *count_ty_val == Ty::VarInt && *subtype_val == Ty::U8 {
        parser.ty_buffer_with_varint
    } else {
        let t = Ty::Array(TyArray { count_ty, subtype });
        parser.alloc_type(t)
    };

    Some(t)
}
fn parse_switch<'x>(
    parser: &mut Parser<'_, 'x>,
    bump: &'x Bump,
    input: &Value,
    parent: &mut ParentData,
) -> Option<TyKey> {
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SwitchJson {
        compare_to: String,
        // default: String,
        fields: IndexMap<String, Value>,
    }
    let switch: SwitchJson = serde_json::from_value(input.clone()).unwrap();

    if !switch
        .compare_to
        .chars()
        .all(|x| matches!(x, 'a'..='z' | 'A'..='Z'))
    {
        return None;
    }
    let compare_to = switch.compare_to.to_case(Case::Snake);

    let mut first_constant = None;
    let mut variants: BTreeMap<Constant<'x>, Variants<'x>> = BTreeMap::new();
    for (k, v) in switch.fields {
        let constant = match k.as_str() {
            "true" => Constant::Bool(true),
            "false" => Constant::Bool(false),
            _ => match k.parse() {
                Ok(x) => Constant::Int(x),
                Err(_) => Constant::String(bump.alloc_str(&k)),
            },
        };
        if first_constant.is_none() {
            first_constant = Some(constant);
        }
        let ty = parse_type(parser, bump, &v, parent)?;

        let variant_name = bump.alloc_str(&format!("Variant_{}", constant));
        variants
            .entry(constant)
            .or_insert_with(|| Variants {
                name: variant_name,
                fields: Vec::new(),
            })
            .fields
            .push(VariantField {
                name: bump.alloc_str(parent.parent_field.unwrap()),
                ty,
            });
    }

    let mut new_last_type = None;
    let last_type = if let Some(last_type) = parent.last_type {
        if let Ty::Enum(last_type) = &mut parser.types[last_type] {
            Some(last_type)
        } else {
            None
        }
    } else {
        None
    };
    let discriminator_type = match first_constant.unwrap() {
        Constant::Bool(_) => "bool",
        Constant::Int(_) => "i32",
        Constant::String(_) => "&str",
    };
    let ty_enum = last_type.unwrap_or_else(|| {
        new_last_type = Some(TyEnum {
            name: bump.alloc_str(&format!("{}_Enum", parent.parent_struct_name)),
            compare_to: bump.alloc_str(&compare_to),
            variants: BTreeMap::new(),
            discriminator_type,
        });
        new_last_type.as_mut().unwrap()
    });

    for (k, v) in variants {
        match ty_enum.variants.get_mut(&k) {
            Some(x) => x.fields.extend(v.fields),
            None => {
                ty_enum.variants.insert(k, v);
                ()
            }
        }
    }

    parent.switch_updated = true;
    match new_last_type {
        Some(x) => Some(parser.alloc_type(Ty::Enum(x))),
        None => parent.last_type,
    }
}
fn parse_type_simple<'x>(
    parser: &mut Parser<'_, 'x>,
    bump: &'x Bump,
    input: &str,
    struct_name: &str,
) -> Option<TyKey> {
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
        "vec3f64" => parser.ty_vec3f64,

        _ => {
            parser.add_unknown_type(bump, input, struct_name);
            return None;
        }
    };
    Some(r)
}
fn parse_type<'x>(
    parser: &mut Parser<'_, 'x>,
    bump: &'x Bump,
    input: &Value,
    parent: &mut ParentData,
) -> Option<TyKey> {
    if let Some(x) = input.as_str() {
        return parse_type_simple(parser, bump, x, parent.parent_struct_name);
    }

    let name = input[0].as_str().unwrap();
    let arg1 = &input[1];
    match name {
        "container" => parse_container(parser, bump, arg1, parent, false),
        "bitfield" => parse_container(parser, bump, arg1, parent, true),
        "option" => parse_option(parser, bump, arg1, parent),
        "buffer" => Some(parse_buffer(parser, input)),
        "array" => parse_array(parser, bump, arg1, parent),
        "switch" => parse_switch(parser, bump, arg1, parent),
        _ => {
            parser.add_unknown_type(bump, name, parent.parent_struct_name);
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
    parser: &mut Parser<'_, 'x>,
    bump: &'x Bump,
    mut direction: JsonDirection,
    kind: &str,
    state_kind: ConnectionState,
) -> Direction<'x> {
    let raw_mappings = do_mapping(&direction.types.shift_remove("packet").unwrap());
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
        let name = bump.alloc_str(&name);

        let ty = if is_ignored {
            parser.alloc_type(Ty::Struct(TyStruct {
                name,
                fields: Vec::new(),
                base_type: None,
                failed: true,
            }))
        } else {
            let mut parent = ParentData {
                parent_struct_name: name,
                parent_field: None,
                last_type: None,
                switch_updated: false,
            };
            match parse_type(parser, bump, &value, &mut parent) {
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

fn state<'x>(
    parser: &mut Parser<'_, 'x>,
    bump: &'x Bump,
    state: JsonState,
    kind: ConnectionState,
) -> State<'x> {
    State {
        kind,
        c2s: direction(parser, bump, state.to_server, "_request", kind),
        s2c: direction(parser, bump, state.to_client, "_response", kind),
    }
}

pub(super) fn parse<'x>(
    types: &mut TypesMap<'x>,
    bump: &'x Bump,
    path: &Path,
    depends: &mut Vec<PathBuf>,
) -> [State<'x>; 1] {
    let content = read_file(path, depends);
    let root: Root = serde_json::from_str(&content).unwrap();

    let mut parser = Parser::new(types);

    let result = [
        // state(&parser, root.handshaking, ConnectionState::Handshaking),
        // state(&parser, root.status, ConnectionState::Status),
        // state(&parser, root.login, ConnectionState::Login),
        state(&mut parser, bump, root.play, ConnectionState::Play),
    ];

    let mut unknown_types: Vec<_> = parser.unknown_types.into_iter().collect();
    unknown_types.sort_by_key(|x| x.0);

    let width = unknown_types
        .iter()
        .map(|x| x.0.len())
        .max()
        .unwrap_or_default();

    eprintln!();
    for (index, (key, mut value)) in unknown_types.into_iter().enumerate() {
        value.sort();

        let width = width - key.len();
        eprintln!(
            "{0:2}. unknown type `{1}` {2:3$}: {4}",
            index + 1,
            key,
            "",
            width,
            value.join(", ")
        );
    }

    result
}
