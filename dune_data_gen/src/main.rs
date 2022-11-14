use serde_derive::{Deserialize, Serialize};
use std::fmt::Write;
use std::process::Command;
use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Version {
    items_path: String,
    enchants_path: String,
}

fn get_path(version_path: &str, name: &str) -> String {
    format!(
        "packet_gen/minecraft-data/data/{}/{}.json",
        version_path, name
    )
}

fn new(name: &str) -> Version {
    #[derive(Debug, Deserialize)]
    struct DataPathsVersion<'x> {
        items: Option<&'x str>,
        enchantments: Option<&'x str>,
    }
    #[derive(Debug, Deserialize)]
    struct DataPathsJson<'x> {
        #[serde(borrow)]
        pc: HashMap<&'x str, DataPathsVersion<'x>>,
    }
    let content = fs::read("packet_gen/minecraft-data/data/dataPaths.json").unwrap();
    let data: DataPathsJson = serde_json::from_slice(&content).unwrap();

    let v = data.pc.get(name).unwrap();
    let items_path = get_path(v.items.unwrap(), "items");
    let enchants_path = get_path(v.enchantments.unwrap(), "enchantments");
    Version {
        items_path,
        enchants_path,
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OutItemsJson {
    name: String,
    display_name: String,
    id: u16,
}

fn write_file_and_fmt(path: &str, content: &str) {
    fs::write(path, content).unwrap();
    Command::new("rustfmt")
        .arg(path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn title_case(input: &str) -> String {
    let mut res = String::with_capacity(input.len());
    let mut next_is_upper = true;
    for mut c in input.chars() {
        if next_is_upper {
            c = c.to_ascii_uppercase();
            next_is_upper = false;
        } else if c == '_' {
            next_is_upper = true;
            continue;
        }
        res.push(c);
    }

    res
}

const V1_18_2: &str = "1.18.2";

fn process_items(versions: &HashMap<&str, Version>) {
    #[derive(Debug, Deserialize)]
    struct InItemsJson<'x> {
        name: String,
        #[serde(rename = "displayName")]
        display_name: &'x str,
        id: u16,
    }

    const JSON_PATH: &str = "dune_lib/src/data/items.json";
    const ITEMS_RS_PATH: &str = "dune_lib/src/data/items.rs";

    let original_items_data = fs::read(JSON_PATH).unwrap();
    let mut items: Vec<OutItemsJson> = serde_json::from_slice(&original_items_data).unwrap();
    let mut items_by_version = HashMap::new();

    for (name, version) in versions {
        let items_v: &mut HashMap<String, u16> = items_by_version.entry(name).or_default();

        let json = fs::read(&version.items_path).unwrap();
        let in_items: Vec<InItemsJson> = serde_json::from_slice(&json).unwrap();

        for i in in_items {
            if !items.iter().any(|x| x.name == i.name) {
                items.push(OutItemsJson {
                    name: i.name.to_string(),
                    display_name: i.display_name.to_string(),
                    id: items.len() as u16,
                });
            }

            items_v.insert(i.name, i.id);
        }
    }

    let out = serde_json::to_vec_pretty(&items).unwrap();
    fs::write(JSON_PATH, out).unwrap();

    let out = &mut String::with_capacity(4096);
    *out += "#[derive(Debug)] pub enum Item {";
    for (index, item) in items.iter().enumerate() {
        write!(out, "{} = {},", title_case(&item.name), index).unwrap();
    }

    *out += "} impl Item {";

    for (v, map) in items_by_version {
        write!(
            out,
            "pub fn from_{}(id: u16) -> anyhow::Result<Self> {{
                use Item::*;
                let result = match id {{",
            v.replace('.', "_")
        )
        .unwrap();

        let mut sorted: Vec<(&str, u16)> =
            map.iter().map(|(name, id)| (name.as_str(), *id)).collect();
        sorted.sort_by(|(_, id1), (_, id2)| id1.cmp(id2));

        for (name, version_id) in sorted {
            let item = items.iter().find(|x| x.name == name).unwrap();
            write!(out, "{} => {},", version_id, title_case(&item.name)).unwrap();
        }

        write!(
            out,
            r#"_ => anyhow::bail!("unknown item id: {{}}", id) }}; Ok(result) }}"#
        )
        .unwrap();
    }
    *out += "}";

    write_file_and_fmt(ITEMS_RS_PATH, out);
}

fn process_enchants(versions: &HashMap<&str, Version>) {
    #[derive(Debug, Serialize, Deserialize)]
    struct EnchJson {
        name: String,
        id: u16,
    }

    const JSON_PATH: &str = "dune_lib/src/data/enchantments.json";
    const RS_PATH: &str = "dune_lib/src/data/enchantments.rs";

    let original_items_data = fs::read(JSON_PATH).unwrap();
    let mut enchants: Vec<EnchJson> = serde_json::from_slice(&original_items_data).unwrap();

    for version in versions.values() {
        let json = fs::read(&version.enchants_path).unwrap();
        let in_items: Vec<EnchJson> = serde_json::from_slice(&json).unwrap();

        for i in in_items {
            if !enchants.iter().any(|x| x.name == i.name) {
                enchants.push(EnchJson {
                    name: i.name.to_string(),
                    id: enchants.len() as u16,
                });
            }
        }
    }

    let out = serde_json::to_vec_pretty(&enchants).unwrap();
    fs::write(JSON_PATH, out).unwrap();

    let out = &mut String::with_capacity(4096);
    *out += "#[derive(Debug)] pub enum Enchantment {";

    for i in enchants.iter() {
        write!(out, "{} = {},", title_case(&i.name), i.id).unwrap();
    }

    *out += r#"}
impl Enchantment {
    pub fn from(input: &str) -> anyhow::Result<Self> {
        const MINECRAFT: &str = "minecraft:";
        let s = match input.strip_prefix(MINECRAFT) {
            Some(x) => x,
            None => anyhow::bail!("unknown enchantment: {}", input),
        };
        use Enchantment::*;

        let result = match s {"#;
    for i in enchants.iter() {
        write!(out, r#""{}" => {},"#, i.name, title_case(&i.name)).unwrap();
    }

    *out += r#"
            _ => anyhow::bail!("unknown enchantment: {}", input)
        };
        Ok(result)
}}"#;

    write_file_and_fmt(RS_PATH, out);
}

fn main() {
    let versions: HashMap<&str, Version> = [(V1_18_2, new(V1_18_2))].into_iter().collect();
    process_items(&versions);
    process_enchants(&versions);
}
