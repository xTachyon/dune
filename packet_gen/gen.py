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


class Ty(enum.Enum):
    STRING = "string"
    BUFFER = "buffer"
    VARINT = "varint"
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


class Field:
    def __init__(self, name, ty):
        self.name = name
        self.ty = ty


class Packet:
    def __init__(self, name, fields, needs_lifetime, valid):
        self.name = name
        self.fields = fields
        self.needs_lifetime = needs_lifetime
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
    def __init__(self, direction, packets, mappings):
        self.direction = direction
        self.packets = packets
        self.mappings = mappings


class StateInfo:
    def __init__(self, state, directions):
        self.state = state
        self.directions = directions


class Parser:
    def parse_container(self, j, name):
        packet_name = name.title().replace('_', '')
        fields = []

        needs_lifetime = False
        for i in j:
            name = i["name"]
            name = pascal_to_snake(name)
            if name == "type":
                name = "type_"
            ty = i["type"]
            if isinstance(ty, str):
                if ty == "u8":
                    ty = Ty.U8
                elif ty == "u16":
                    ty = Ty.U16

                elif ty == "i8":
                    ty = Ty.I8
                elif ty == "i16":
                    ty = Ty.I16
                elif ty == "i32":
                    ty = Ty.I32
                elif ty == "i64":
                    ty = Ty.I64

                elif ty == "f32":
                    ty = Ty.F32
                elif ty == "f64":
                    ty = Ty.F64

                elif ty == "bool":
                    ty = Ty.BOOL
                elif ty == "UUID":
                    ty = Ty.UUID
                elif ty == "string":
                    ty = Ty.STRING
                elif ty == "varint":
                    ty = Ty.VARINT
                elif ty == "position":
                    ty = Ty.POSITION
            else:
                ty = ty[0]
                if ty == "buffer":
                    ty = Ty.BUFFER

            if not isinstance(ty, Ty):
                print(f"couldn't parse name=`{name}`,type=`{ty}` in packet `{packet_name}`")
                return Packet(packet_name, [], False, False)

            needs_lifetime = needs_lifetime or ty == Ty.STRING or ty == Ty.BUFFER
            fields.append(Field(name, ty))

        return Packet(packet_name, fields, needs_lifetime, True)

    def make_name_direction(self, state, direction, name):
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

    def parse_direction(self, j, state, direction):
        packets = []
        j = j[direction.value]["types"]
        m = {}
        for name in j:
            value = j[name]
            name = name.removeprefix("packet_")
            name = self.make_name_direction(state, direction, name)

            ty = value[0]
            if name == "packet":
                mappings = None
                for i in value[1]:
                    if i["name"] == "name":
                        mappings = i
                mappings = mappings["type"][1]["mappings"]
                for m_id in mappings:
                    mapping_name = mappings[m_id]
                    mapping_name = self.make_name_direction(state, direction, mapping_name)
                    m_id = int(m_id, 16)
                    m[m_id] = mapping_name
            elif ty == "container":
                p = self.parse_container(value[1], name)
                packets.append(p)
            else:
                unreachable()

        return DirectionInfo(direction, packets, m)

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


class Generator:
    def __init__(self):
        self.out = ""

    def gen_packet(self, p):
        needs_lifetime = False
        for i in p.fields:
            if i.ty == Ty.STRING or i.ty == Ty.BUFFER:
                needs_lifetime = True
                break

        lifetime_simple = f'''{"'p" if needs_lifetime else ""}'''
        lifetime = f'''{"<'p>" if needs_lifetime else ""}'''
        self.out += f'''#[derive(Debug)] pub struct {p.name} {lifetime} {{'''
        for i in p.fields:
            if i.ty == Ty.VARINT:
                ty = "i32"
            elif i.ty == Ty.STRING:
                ty = "&'p str"
            elif i.ty == Ty.BUFFER:
                ty = "&'p [u8]"
            elif i.ty == Ty.POSITION:
                ty = "crate::de::Position"
            else:
                ty = i.ty.value
            self.out += f"pub {i.name}: {ty},"

        self.out += "}"
        underscore = "_" if len(p.fields) == 0 else ""
        self.out += f'''pub(super) fn packet_{pascal_to_snake(p.name)}{lifetime}({underscore}reader: &{lifetime_simple} mut Reader{lifetime}) 
        -> Result<{p.name}{lifetime}> {{ '''
        for i in p.fields:
            self.out += f"let {i.name} = "
            if i.ty == Ty.STRING or i.ty == Ty.BUFFER:
                self.out += "reader.read_range()?;"
            else:
                self.out += "MinecraftDeserialize::deserialize(&mut reader.cursor)?;"

        for i in p.fields:
            if i.ty == Ty.STRING:
                self.out += f"let {i.name} = "
                self.out += f"reader.get_str_from({i.name})?;"
            elif i.ty == Ty.BUFFER:
                self.out += f"let {i.name} = "
                self.out += f"reader.get_buf_from({i.name})?;"

        self.out += f"\n\nlet result = {p.name} {{"
        for i in p.fields:
            self.out += f"{i.name},"

        self.out += "};"
        self.out += "Ok(result)"
        self.out += "}"

    def gen_map(self, c2s, s2c):
        pass

    def gen(self, states):
        for state in states:
            self.out += f'''
pub mod {state.state.value} {{
use anyhow::Result;
use crate::de::MinecraftDeserialize;
use crate::de::Reader;

'''
            for direction in state.directions:
                for i in direction.packets:
                    self.gen_packet(i)

            self.out += "}"

        self.out += f'''
use anyhow::{{anyhow, Result}};
use crate::protocol::ConnectionState as S;
use crate::protocol::PacketDirection as D;
use crate::de::Reader;

#[derive(Debug)]
pub enum Packet<'p> {{
'''

        for state in states:
            for direction in state.directions:
                for i in direction.packets:
                    self.out += f'''{i.name}({state.state.value}::{i.name}{"<'p>" if i.needs_lifetime else ""}),'''

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
    with open("protocol.json") as f:
        j = json.load(f)

    parser = Parser()
    states = parser.parse(j)

    gen = Generator()
    gen.gen(states)

    out_file = "../proxy_lib/src/protocol/v1_18_1.rs"
    with open(out_file, "w") as f:
        f.write(gen.out)
    subprocess.run(["rustfmt", out_file])


if __name__ == "__main__":
    main()
