import json
import subprocess
import re
import enum


def unreachable():
    assert False


def pascal_to_snake(name):
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()


def snake_to_pascal(name):
    return name.title().replace("_", "")


class BuiltinType(enum.Enum):
    STRING = "string"
    BUFFER = "buffer"
    REST_BUFFER = "rest_buffer"
    SLOT = "slot"
    VARINT = "varint"
    VARLONG = "varlong"
    POSITION = "position"
    BOOL = "bool"

    U8 = "u8"
    U16 = "u16"
    UUID = "u128"

    I8 = "i8"
    I16 = "i16"
    I32 = "i32"
    I64 = "i64"

    F32 = "f32"
    F64 = "f64"


class ArrayType:
    def __init__(self, subtype, count_type):
        self.subtype = subtype
        self.count_type = count_type


class OptionType:
    def __init__(self, subtype):
        self.subtype = subtype


class StructType:
    def __init__(self, name, fields, needs_lifetime):
        self.name = name
        self.fields = fields
        self.needs_lifetime = needs_lifetime


def is_builtin(x):
    return isinstance(x, BuiltinType)


def is_array(x):
    return isinstance(x, ArrayType)


def is_option(x):
    return isinstance(x, OptionType)


def is_struct(x):
    return isinstance(x, StructType)


class Field:
    def __init__(self, name, ty):
        self.name = name
        self.ty = ty


class Packet:
    def __init__(self, name, struct, valid):
        self.name = name
        self.struct = struct
        self.valid = valid


class State(enum.Enum):
    HANDSHAKING = "handshaking"
    STATUS = "status"
    LOGIN = "login"
    PLAY = "play"


class Direction(enum.Enum):
    C2S = "toServer"
    S2C = "toClient"


class DirectionInfo:
    def __init__(self, direction, structs, packets, mappings):
        self.direction = direction
        self.structs = structs
        self.packets = packets
        self.mappings = mappings


class StateInfo:
    def __init__(self, state, directions):
        self.state = state
        self.directions = directions


def parse_array(ty, parent_name, structs):
    count_type = parse_type(ty["countType"], parent_name, structs)
    ty = parse_type(ty["type"], parent_name, structs)
    return ArrayType(ty, count_type)


def parse_option(ty, parent_name, structs):
    subtype = parse_type(ty, parent_name, structs)
    return OptionType(subtype)


def parse_type(ty, parent_name, structs):
    original_ty = ty
    if isinstance(ty, str):
        if ty == "u8":
            return BuiltinType.U8
        if ty == "u16":
            return BuiltinType.U16

        if ty == "i8":
            return BuiltinType.I8
        if ty == "i16":
            return BuiltinType.I16
        if ty == "i32":
            return BuiltinType.I32
        if ty == "i64":
            return BuiltinType.I64

        if ty == "f32":
            return BuiltinType.F32
        if ty == "f64":
            return BuiltinType.F64

        if ty == "bool":
            return BuiltinType.BOOL
        if ty == "UUID":
            return BuiltinType.UUID
        if ty == "string":
            return BuiltinType.STRING
        if ty == "slot":
            return BuiltinType.SLOT
        if ty == "varint":
            return BuiltinType.VARINT
        if ty == "varlong":
            return BuiltinType.VARLONG
        if ty == "position":
            return BuiltinType.POSITION
        if ty == "restBuffer":
            return BuiltinType.REST_BUFFER
    ty = ty[0]
    if ty == "buffer":
        return BuiltinType.BUFFER
    if ty == "array":
        return parse_array(original_ty[1], parent_name, structs)
    if ty == "option":
        return parse_option(original_ty[1], parent_name, structs)
    if ty == "container":
        return parse_container(original_ty[1], parent_name, structs)

    raise Exception("unknown type")


def parse_container(j, struct_name, structs, root=False):
    fields = []

    needs_lifetime = True
    for i in j:
        name = i["name"]
        name = pascal_to_snake(name)
        if name == "type":
            name = "type_"
        if name == "match":
            name = "match_"
        ty = i["type"]
        try:
            ty = parse_type(ty, struct_name, structs)
        except:
            print(f"couldn't parse name=`{name}`,type=`{str(ty)[:15]}` in packet `{struct_name}`")
            raise

        needs_lifetime = needs_lifetime or ty == BuiltinType.STRING or ty == BuiltinType.BUFFER
        fields.append(Field(name, ty))

    if not root:
        for i in fields:
            struct_name += i.name.title()

    result = StructType(struct_name, fields, needs_lifetime)
    structs.append(result)
    return result


def make_name_direction(state, direction, name):
    if name == "packet" or name.endswith("_request") or name.endswith("_response"):
        return name

    if direction == Direction.C2S:
        name += "_request"
    else:
        name += "_response"

    if name == "ping_response":
        if state == State.PLAY:
            name = "play_ping_response"
    return name


class Parser:
    def parse_direction(self, j, state, direction):
        packets = []
        structs = []
        j = j[direction.value]["types"]
        m = {}
        for name in j:
            value = j[name]
            name = name.removeprefix("packet_")
            name = make_name_direction(state, direction, name)

            ty = value[0]
            if name == "packet":
                mappings = None
                for i in value[1]:
                    if i["name"] == "name":
                        mappings = i
                mappings = mappings["type"][1]["mappings"]
                for m_id in mappings:
                    mapping_name = mappings[m_id]
                    mapping_name = make_name_direction(state, direction, mapping_name)
                    m_id = int(m_id, 16)
                    m[m_id] = mapping_name
            elif ty == "container":
                packet_name = name.title().replace('_', '')
                try:
                    struct = parse_container(value[1], packet_name, structs, True)
                    p = Packet(packet_name, struct, True)
                    packets.append(p)
                except:
                    structs.append(StructType(packet_name, [], False))
                    packets.append(Packet(packet_name, None, False))
            else:
                unreachable()

        return DirectionInfo(direction, structs, packets, m)

    def parse_state(self, j, state):
        j = j[state.value]
        directions = [self.parse_direction(j, state, Direction.C2S), self.parse_direction(j, state, Direction.S2C)]

        return StateInfo(state, directions)

    def parse(self, j):
        result = [
            self.parse_state(j, State.HANDSHAKING),
            self.parse_state(j, State.STATUS),
            self.parse_state(j, State.LOGIN),
            self.parse_state(j, State.PLAY)
        ]
        return result


def get_type(ty):
    if is_array(ty):
        return f"Vec<{get_type(ty.subtype)}>"
    if is_option(ty):
        return f"Option<{get_type(ty.subtype)}>"
    if is_struct(ty):
        return ty.name

    if ty == BuiltinType.VARINT:
        return "i32"
    if ty == BuiltinType.VARLONG:
        return "i64"
    if ty == BuiltinType.SLOT:
        return "InventorySlot"
    if ty == BuiltinType.STRING:
        return "IndexedString"
    if ty == BuiltinType.BUFFER or ty == BuiltinType.REST_BUFFER:
        return "IndexedBuffer"
    if ty == BuiltinType.POSITION:
        return "crate::protocol::de::Position"
    return ty.value


def deserialize_type(name, ty, current_element_count):
    if is_array(ty):
        out = deserialize_type("count_array", ty.count_type, current_element_count + 1)
        out += f'''let mut {name} = Vec::with_capacity(count_array as usize); for _ in 0..count_array {{'''
        elem_name = "x"
        if current_element_count > 1:
            elem_name += f"_{current_element_count}"
        out += deserialize_type(elem_name, ty.subtype, current_element_count + 1)
        out += f'''{name}.push({elem_name});}}'''
        return out
    out = f"let {name}: {get_type(ty)} = "
    if is_struct(ty):
        out += f"packet_{pascal_to_snake(ty.name)}(reader)?;"
    elif ty == BuiltinType.STRING:
        out += "reader.read_indexed_string()?;"
    elif ty == BuiltinType.BUFFER:
        out += "reader.read_indexed_buffer()?;"
    elif ty == BuiltinType.REST_BUFFER:
        out += "reader.read_rest_buffer();"
    elif ty == BuiltinType.VARINT:
        out += "read_varint(&mut reader)?;"
    elif ty == BuiltinType.VARLONG:
        out += "read_varlong(&mut reader)?;"
    else:
        out += "MD::deserialize(reader)?;"
    return out


class Generator:
    def __init__(self):
        self.out = "#![allow(unused_imports)] #![allow(unused_mut)] #![allow(non_camel_case_types)] #![allow(non_snake_case)]"

    def gen_struct(self, struct):
        needs_lifetime = True

        for i in struct.fields:
            if i.ty == BuiltinType.STRING or i.ty == BuiltinType.BUFFER:
                needs_lifetime = True
                break
        needs_lifetime = False

        lifetime_simple = f'''{"'p" if needs_lifetime else ""}'''
        lifetime = f'''{"<'p>" if needs_lifetime else ""}'''
        self.out += f'''#[derive(Debug)] pub struct {struct.name} {lifetime} {{'''
        for i in struct.fields:
            ty = get_type(i.ty)
            self.out += f"pub {i.name}: {ty},"

        self.out += "}"
        underscore = "_" if len(struct.fields) == 0 else ""
        self.out += f'''pub(super) fn packet_{pascal_to_snake(struct.name)}{lifetime}(mut {underscore}reader: &{lifetime_simple} mut Reader{lifetime}) 
        -> Result<{struct.name}{lifetime}> {{ '''
        for i in struct.fields:
            self.out += deserialize_type(i.name, i.ty, 1)

        self.out += f"\n\nlet result = {struct.name} {{"
        for i in struct.fields:
            self.out += f"{i.name},"

        self.out += "};"
        self.out += "Ok(result)"
        self.out += "}"

    def gen(self, states):
        for state in states:
            self.out += f'''
pub mod {state.state.value} {{
use anyhow::Result;
use crate::protocol::IndexedBuffer;
use crate::protocol::IndexedString;
use crate::protocol::InventorySlot;
use crate::protocol::de::MD;
use crate::protocol::de::Reader;
use crate::protocol::varint::read_varint;
use crate::protocol::varint::read_varlong;

'''
            for direction in state.directions:
                for i in direction.structs:
                    self.gen_struct(i)

            self.out += "}"

        self.out += f'''
use anyhow::{{anyhow, Result}};
use crate::protocol::ConnectionState as S;
use crate::protocol::PacketDirection as D;
use crate::protocol::de::Reader;

#[derive(Debug)]
pub enum Packet {{
'''

        for state in states:
            for direction in state.directions:
                for i in direction.packets:
                    needs_lifetime = True
                    if i.struct is not None:
                        needs_lifetime = i.struct.needs_lifetime
                    needs_lifetime = False
                    self.out += f'''{i.name}({state.state.value}::{i.name}{"<'p>" if needs_lifetime else ""}),'''

        self.out += '''
}
        
pub fn de_packets<'r>(state: S, direction: D, id: u32, reader: &'r mut Reader<'r>) -> Result<Packet> {
let packet = match (state, direction, id) {
'''

        for state in states:
            for direction in state.directions:
                for m_id in direction.mappings:
                    name = direction.mappings[m_id]

                    state_name = state.state.value.title()
                    dir_name = "ClientToServer" if direction.direction == Direction.C2S else "ServerToClient"
                    self.out += f"(S::{state_name}, D::{dir_name}, {m_id:#x}) => {{ let p = {state.state.value}::packet_{name}(reader)?; Packet::{snake_to_pascal(name)}(p) }}"

        self.out += '''_ => { return Err(anyhow!("unknown packet id={}", id)); } }; Ok(packet) }'''


def main():
    with open("minecraft-data/data/pc/1.18.2/protocol.json") as f:
        j = json.load(f)

    parser = Parser()
    states = parser.parse(j)

    gen = Generator()
    gen.gen(states)

    out_file = "../melon/src/protocol/v1_18_2.rs"
    with open(out_file, "w") as f:
        f.write(gen.out)
    subprocess.run(["rustfmt", out_file])


if __name__ == "__main__":
    main()
