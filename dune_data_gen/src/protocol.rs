mod parser;
mod writer;

use bumpalo::Bump;
use std::{fs, process::Command};

#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct TyStruct<'x> {
    name: String,
    fields: Vec<(String, &'x Ty<'x>)>,
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
    U128,

    I8,
    I16,
    I32,
    I64,

    F32,
    F64,

    BOOL,
    VARINT,
    VARLONG,
    STRING,
    BUFFER,
    RESTBUFFER,
    POSITION,
    SLOT,
    NBT,
    OPTIONALNBT,

    Struct(TyStruct<'x>),
    Option(TyOption<'x>),
    Array(TyArray<'x>),
}
use Ty::*;

impl<'x> Ty<'x> {
    fn needs_lifetime(&self) -> bool {
        match self {
            STRING | BUFFER | RESTBUFFER | SLOT | NBT | OPTIONALNBT => true,
            Struct(x) => x.fields.iter().map(|x| x.1).any(Ty::needs_lifetime),
            Option(x) => x.subtype.needs_lifetime(),
            Array(x) => x.subtype.needs_lifetime(),
            _ => false,
        }
    }
    fn get_simple_type(&self) -> &'static str {
        match self {
            U8 => "u8",
            U16 => "u16",
            U128 => "u128",
            I8 => "i8",
            I16 => "i16",
            I32 => "i32",
            I64 => "i64",
            F32 => "f32",
            F64 => "f64",
            BOOL => "bool",
            VARINT => "i32",
            VARLONG => "i64",
            STRING => "&'p str",
            BUFFER | RESTBUFFER => "&'p [u8]",
            POSITION => "Position",
            SLOT => "InventorySlot<'p>",
            NBT => "IndexedNbt<'p>",
            OPTIONALNBT => "IndexedOptionNbt<'p>",
            _ => unreachable!("{:?}", self),
        }
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
    Handshaking,
    Status,
    Login,
    Play,
}
impl ConnectionState {
    fn name(&self, title: bool) -> &'static str {
        match self {
            ConnectionState::Handshaking => {
                if title {
                    "Handshaking"
                } else {
                    "handshaking"
                }
            }
            ConnectionState::Status => {
                if title {
                    "Status"
                } else {
                    "status"
                }
            }
            ConnectionState::Login => {
                if title {
                    "Login"
                } else {
                    "login"
                }
            }
            ConnectionState::Play => {
                if title {
                    "Play"
                } else {
                    "play"
                }
            }
        }
    }
}
struct State<'x> {
    pub kind: ConnectionState,
    pub c2s: Direction<'x>,
    pub s2c: Direction<'x>,
}

pub(super) fn run(path: &str) {
    let bump = Bump::new();
    let states = parser::parse(path, &bump);
    let out = writer::write(states).unwrap();

    let path = "dune_lib/src/protocol/v1_18_2.rs";
    fs::write(path, out).unwrap();

    Command::new("rustfmt").arg(path).spawn().unwrap();
}
