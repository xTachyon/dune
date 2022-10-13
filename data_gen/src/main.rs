use serde_derive::{Deserialize, Serialize};
use std::fmt::Write;
use std::process::Command;
use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Version {
    items_path: String,
    // enchantments_path: String,
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
        // enchantments: Option<&'x str>,
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
    // let enchantments_path = get_path(v.enchantments.unwrap(), "enchantments");
    Version {
        items_path,
        // enchantments_path,
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
    let mut res = String::new();
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

fn process_items(versions: &[&str]) {
    #[derive(Debug, Deserialize)]
    struct InItemsJson<'x> {
        name: String,
        #[serde(rename = "displayName")]
        display_name: &'x str,
        id: u32,
    }

    const JSON_PATH: &str = "melon/src/data/items.json";
    const ITEMS_RS_PATH: &str = "melon/src/data/items.rs";

    let original_items_data = fs::read(JSON_PATH).unwrap();
    let mut items: Vec<OutItemsJson> = serde_json::from_slice(&original_items_data).unwrap();
    let mut items_by_version = HashMap::new();

    for v in versions {
        let version = new(v);
        let items_v: &mut HashMap<String, u32> = items_by_version.entry(v).or_default();

        let json = fs::read(version.items_path).unwrap();
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

        for (name, version_id) in map {
            let item = items.iter().find(|x| x.name == name).unwrap();
            write!(out, "{} => {},", version_id, title_case(&item.name)).unwrap();
        }

        write!(
            out,
            r#"_ => anyhow::bail!("unknown item id") }}; Ok(result) }}"#
        )
        .unwrap();
    }
    *out += "}";

    write_file_and_fmt(ITEMS_RS_PATH, out);
}

fn main() {
    process_items(&["1.18.2"]);
}
