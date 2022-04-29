import json
import subprocess
import re
import enum


def unreachable():
    assert False


def camel_to_snake(name):
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()


class Ty(enum.Enum):
    STRING = "string"
    VARINT = "varint"
    U8 = "u8"
    U16 = "u16"


class Field:
    def __init__(self, name, ty):
        self.name = name
        self.ty = ty


class Packet:
    def __init__(self, name, fields):
        self.name = name
        self.fields = fields


class Parser:
    def gen_container(self, j, name):
        packet_name = name.title().replace('_', '')
        fields = []

        for i in j:
            name = i["name"]
            name = camel_to_snake(name)
            ty = i["type"]
            if ty == "u8":
                ty = Ty.U8
            elif ty == "u16":
                ty = Ty.U16
            elif ty == "string":
                ty = Ty.STRING
            elif ty == "varint":
                ty = Ty.VARINT
            else:
                unreachable()
            fields.append(Field(name, ty))

        return Packet(packet_name, fields)

    def gen_direction(self, j, name):
        packets = []
        j = j[name]["types"]
        for name in j:
            value = j[name]
            name = name.removeprefix("packet_")
            ty = value[0]
            if name == "packet":
                continue
            if ty == "container":
                p = self.gen_container(value[1], name)
                packets.append(p)
            else:
                unreachable()

        return packets

    def gen_state(self, j, name):
        packets = self.gen_direction(j[name], "toServer")
        return packets


class Generator:
    def __init__(self):
        self.out = '''
use std::io::Read;
use super::super::de::MinecraftDeserialize;
use super::super::de::Reader;
use anyhow::Result;

        '''

    def packet(self, p):
        needs_lifetime = False
        for i in p.fields:
            if i.ty == Ty.STRING:
                needs_lifetime = True
                break

        lifetime_simple = f'''{"'p" if needs_lifetime else ""}'''
        lifetime = f'''{"<'p>" if needs_lifetime else ""}'''
        self.out += f'''pub struct {p.name} {lifetime} {{'''
        for i in p.fields:
            if i.ty == Ty.VARINT:
                ty = "i32"
            elif i.ty == Ty.STRING:
                ty = "&'p str"
            else:
                ty = i.ty.value
            self.out += f"pub {i.name}: {ty},"

        self.out += "}"
        self.out += f'''pub fn de_{camel_to_snake(p.name)}{lifetime}(reader: &{lifetime_simple} mut Reader{lifetime}) -> Result<{p.name}{lifetime}> {{'''
        for i in p.fields:
            self.out += f"let {i.name} = "
            if i.ty == Ty.STRING:
                self.out += "reader.read_str()?;"
            else:
                self.out += "MinecraftDeserialize::deserialize(&mut reader.cursor)?;"

        for i in p.fields:
            if i.ty == Ty.STRING:
                self.out += f"let {i.name} = "
                self.out += "reader.get_str_from(server_host)?;"

        self.out += f"\n\nlet result = {p.name} {{"
        for i in p.fields:
            self.out += f"{i.name},"

        self.out += "};"
        self.out += "Ok(result)"
        self.out += "}"


def main():
    with open("protocol.json") as f:
        j = json.load(f)

    parser = Parser()
    packets = parser.gen_state(j, "handshaking")

    gen = Generator()
    for i in packets:
        gen.packet(i)

    out_file = "../proxy_lib/src/pro/v1_18_1.rs"
    with open(out_file, "w") as f:
        f.write(gen.out)
    subprocess.run(["rustfmt", out_file])


if __name__ == "__main__":
    main()
