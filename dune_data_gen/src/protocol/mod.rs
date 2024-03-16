mod parser;
mod writer;

use bumpalo::Bump;
use std::{fs, process::Command};

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
    name: String,
    fields: Vec<(String, &'x Ty<'x>)>,
    base_type: Option<&'x Ty<'x>>, // only for bitfields
    failed: bool,
}
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct TyOption<'x> {
    subtype: &'x Ty<'x>,
}
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct TyArray<'x> {
    count_ty: &'x Ty<'x>,
    subtype: &'x Ty<'x>,
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

    Struct(TyStruct<'x>),
    Option(TyOption<'x>),
    Array(TyArray<'x>),
    Bitfield(TyBitfield),
}

impl<'x> Ty<'x> {
    fn needs_lifetime(&self) -> bool {
        use Ty::*;
        match self {
            String | Buffer(_) | RestBuffer | Slot | Nbt | OptionNbt | ChunkBlockEntity => true,
            Struct(x) => x.fields.iter().map(|x| x.1).any(Ty::needs_lifetime),
            Option(x) => x.subtype.needs_lifetime(),
            Array(x) => x.subtype.needs_lifetime() || x.subtype.is_rs_builtin(),
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
    pub name: String,
    pub ty: &'x Ty<'x>,
    pub id: u16,
}
struct Direction<'x> {
    pub packets: Vec<Packet<'x>>,
}
#[derive(PartialEq, Eq, Clone, Copy)]
enum ConnectionState {
    // Handshaking,
    // Status,
    // Login,
    Play,
}
impl ConnectionState {
    fn name(&self, title: bool) -> &'static str {
        use ConnectionState::*;

        match self {
            // Handshaking if title => "Handshaking",
            // Handshaking => "handshaking",

            // Status if title => "Status",
            // Status => "status",

            // Login if title => "Login",
            // Login => "login",

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

pub(super) fn run(version: &str, path: &str) {
    let bump = Bump::new();
    let states = parser::parse(path, &bump);
    let out = writer::write(states).unwrap();

    let path = format!("dune_data/src/protocol/v{}.rs", version.replace('.', "_"));
    fs::write(&path, out).unwrap();

    Command::new("rustfmt").arg(path).spawn().unwrap();
}
