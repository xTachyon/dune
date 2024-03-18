mod parser;
mod writer;

use bumpalo::Bump;
use humansize::{format_size, BINARY};
use slotmap::{new_key_type, SlotMap};
use std::{
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

new_key_type! {
    struct TyKey;
}

type TypesMap<'x> = SlotMap<TyKey, Ty<'x>>;

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct TyBitfield {
    range_begin: u16, // Range<u16> is not PartialOrd :thinking:
    range_end: u16,
    base_type_size: u16,
    unsigned: bool,
}
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum TyBufferCountKind {
    Fixed(u16),
    Varint,
}
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct TyBuffer {
    kind: TyBufferCountKind,
}
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct TyStruct<'x> {
    name: &'x str,
    fields: Vec<(&'x str, TyKey)>,
    base_type: Option<TyKey>, // only for bitfields
    failed: bool,
}
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Constant<'x> {
    Bool(bool),
    Int(u32),
    String(&'x str),
}
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct TyEnum<'x> {
    name: &'x str,
    compare_to: &'x str,
    variants: Vec<(Constant<'x>, TyKey)>,
}
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct TyOption {
    subtype: TyKey,
}
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct TyArray {
    count_ty: TyKey,
    subtype: TyKey,
}
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Ty<'x> {
    U8,
    U16,
    U32,
    U64,
    U128,

    I8,
    I16,
    I32,
    I64,

    F32,
    F64,

    Bool,
    VarInt,
    VarLong,
    String,
    Buffer(TyBuffer),
    RestBuffer,
    Position,
    Slot,
    Nbt,
    OptionNbt,
    ChunkBlockEntity,
    Vec3f64,

    Struct(TyStruct<'x>),
    Enum(TyEnum<'x>),
    Option(TyOption),
    Array(TyArray),
    Bitfield(TyBitfield),
}

impl<'x> Ty<'x> {
    fn needs_lifetime(&self, types: &TypesMap) -> bool {
        use Ty::*;
        match self {
            String | Buffer(_) | RestBuffer | Slot | Nbt | OptionNbt | ChunkBlockEntity => true,
            Struct(x) => x
                .fields
                .iter()
                .map(|x| x.1)
                .any(|x| types[x].needs_lifetime(types)),
            Option(x) => types[x.subtype].needs_lifetime(types),
            Array(x) => types[x.subtype].needs_lifetime(types) || types[x.subtype].is_rs_builtin(),
            _ => false,
        }
    }
    fn get_simple_type(&self) -> &'static str {
        use Ty::*;
        match self {
            U8 => "u8",
            U16 => "u16",
            U32 => "u32",
            U64 => "u64",
            U128 => "u128",

            I8 => "i8",
            I16 => "i16",
            I32 => "i32",
            I64 => "i64",

            F32 => "f32",
            F64 => "f64",

            Bool => "bool",
            VarInt => "i32",
            VarLong => "i64",
            String => "&'p str",
            RestBuffer => "&'p [u8]",
            Position => "Position",
            Slot => "InventorySlot<'p>",
            Nbt => "IndexedNbt<'p>",
            OptionNbt => "IndexedOptionNbt<'p>",
            ChunkBlockEntity => "ChunkBlockEntity",
            Vec3f64 => "Vec3f64",
            Bitfield(x) => {
                let size = width_for_bitfields(x.range_end - x.range_begin);
                let base_type = match (size, x.unsigned) {
                    (32, false) => Ty::U32,
                    (64, false) => Ty::U64,
                    _ => unreachable!(
                        "unknown bitfield with size={},unsigned={}",
                        size, x.unsigned
                    ),
                };
                base_type.get_simple_type()
            }
            _ => unreachable!("{:?}", self),
        }
    }
    fn is_rs_builtin(&self) -> bool {
        use Ty::*;
        matches!(
            self,
            Bool | U8 | U16 | U32 | U64 | U128 | I8 | I16 | I32 | I64 | F32 | F64
        )
    }
    fn width(&self) -> u16 {
        use Ty::*;
        match self {
            I64 => 64,
            _ => unreachable!("unknown type {:?}", self),
        }
    }
}
fn width_for_bitfields(size: u16) -> u16 {
    match size {
        0..=8 => 8,
        9..=16 => 16,
        17..=32 => 32,
        33..=64 => 64,
        _ => unreachable!(),
    }
}

struct Packet<'x> {
    pub name: &'x str,
    pub ty: TyKey,
    pub id: u16,
}
struct Direction<'x> {
    pub packets: Vec<Packet<'x>>,
}
#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy)]
enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}
impl ConnectionState {
    fn name(&self, title: bool) -> &'static str {
        use ConnectionState::*;

        match self {
            Handshaking if title => "Handshaking",
            Handshaking => "handshaking",

            Status if title => "Status",
            Status => "status",

            Login if title => "Login",
            Login => "login",
            Play if title => "Play",
            Play => "play",
        }
    }
}
struct State<'x> {
    pub kind: ConnectionState,
    pub c2s: Direction<'x>,
    pub s2c: Direction<'x>,
}

pub(super) fn run(version: &str, path: &Path, out_dir: &str, depends: &mut Vec<PathBuf>) {
    let bump = Bump::new();
    let mut types = TypesMap::with_capacity_and_key(32);

    let states = parser::parse(&mut types, &bump, path, depends);
    let out = writer::write(&types, states);

    let syntax_tree = syn::parse_file(&out).unwrap();
    let out = prettyplease::unparse(&syntax_tree);

    let path = format!("{}/v{}.rs", out_dir, version.replace('.', "_"));
    fs::write(&path, out).unwrap();

    let bytes = bump.allocated_bytes();
    println!(
        "bump size: {} ({} bytes)",
        format_size(bytes, BINARY),
        bytes
    );
}
