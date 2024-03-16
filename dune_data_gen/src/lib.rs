mod protocol;

use fs_err as fs;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
struct Version {
    items_path: PathBuf,
    enchants_path: PathBuf,
    protocol_path: PathBuf,
}

fn get_path(mc_data_path: &Path, version_path: &str, name: &str) -> PathBuf {
    mc_data_path
        .join("data")
        .join(version_path)
        .join(format!("{}.json", name))
}

fn read_file<P: AsRef<Path>>(path: P, depends: &mut Vec<PathBuf>) -> String {
    depends.push(path.as_ref().to_owned());
    fs::read_to_string(path).unwrap()
}

fn new(name: &str, mc_data_path: &Path, depends: &mut Vec<PathBuf>) -> Version {
    #[derive(Debug, Deserialize)]
    struct DataPathsVersion<'x> {
        items: Option<&'x str>,
        enchantments: Option<&'x str>,
        protocol: Option<&'x str>,
    }
    #[derive(Debug, Deserialize)]
    struct DataPathsJson<'x> {
        #[serde(borrow)]
        pc: HashMap<&'x str, DataPathsVersion<'x>>,
    }
    let content = read_file(mc_data_path.join("data/dataPaths.json"), depends);
    let data: DataPathsJson = serde_json::from_str(&content).unwrap();

    let v = data.pc.get(name).unwrap();
    let items_path = get_path(mc_data_path, v.items.unwrap(), "items");
    let enchants_path = get_path(mc_data_path, v.enchantments.unwrap(), "enchantments");
    let protocol_path = get_path(mc_data_path, v.protocol.unwrap(), "protocol");
    Version {
        items_path,
        enchants_path,
        protocol_path,
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

fn process_items(versions: &HashMap<&str, Version>, depends: &mut Vec<PathBuf>) {
    #[derive(Debug, Deserialize)]
    struct InItemsJson<'x> {
        name: String,
        #[serde(rename = "displayName")]
        display_name: &'x str,
        id: u16,
    }

    const JSON_PATH: &str = "dune_data/src/items.json";
    const ITEMS_RS_PATH: &str = "dune_data/src/items.rs";

    let original_items_data = read_file(JSON_PATH, depends);
    let mut items: Vec<OutItemsJson> = serde_json::from_str(&original_items_data).unwrap();
    let mut items_by_version = HashMap::new();

    for (name, version) in versions {
        let items_v: &mut HashMap<String, u16> = items_by_version.entry(name).or_default();

        let json = read_file(&version.items_path, depends);
        let in_items: Vec<InItemsJson> = serde_json::from_str(&json).unwrap();

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
    *out += "use std::collections::HashMap;
    use Item::*;
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum Item {";
    for (index, item) in items.iter().enumerate() {
        write!(out, "{} = {},", title_case(&item.name), index).unwrap();
    }

    *out += "}
    impl Item {";
    *out += r#"pub fn from_str_id(id: &str) -> anyhow::Result<Self> {
        const MINECRAFT: &str = "minecraft:";
        let id = match id.strip_prefix(MINECRAFT) {
            Some(x) => x,
            None => anyhow::bail!("unknown item id: {}", id),
        };
        items().get(id).copied().ok_or_else(|| anyhow::anyhow!("unknown item id: {}", id))
    }"#;

    for (v, map) in items_by_version {
        write!(
            out,
            "pub fn from_{}(id: u16) -> anyhow::Result<Self> {{
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

    writeln!(out, "const DATA: [(&str, Item); {}] = [", items.len()).unwrap();
    for item in items.iter() {
        write!(out, r#"("{}", {}),"#, item.name, title_case(&item.name)).unwrap();
    }
    *out += "];
    fn items() -> &'static HashMap<&'static str, Item> {
        // Faster than a match, somehow.

        use std::sync::OnceLock;
        static ITEMS: OnceLock<HashMap<&'static str, Item>> = OnceLock::new();
        ITEMS.get_or_init(|| HashMap::from(DATA))
    }
    ";

    write_file_and_fmt(ITEMS_RS_PATH, out);
}

fn process_enchants(versions: &HashMap<&str, Version>, depends: &mut Vec<PathBuf>) {
    #[derive(Debug, Serialize, Deserialize)]
    struct EnchJson {
        name: String,
        id: u16,
    }

    const JSON_PATH: &str = "dune_data/src/enchantments.json";
    const RS_PATH: &str = "dune_data/src/enchantments.rs";

    let original_items_data = read_file(JSON_PATH, depends);
    let mut enchants: Vec<EnchJson> = serde_json::from_str(&original_items_data).unwrap();

    for version in versions.values() {
        let json = read_file(&version.enchants_path, depends);
        let in_items: Vec<EnchJson> = serde_json::from_str(&json).unwrap();

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

pub fn run(out_dir: &str, mc_data_path: &Path) -> Vec<PathBuf> {
    const VERSIONS: &[&str] = &["1.18.2", "1.19.3", "1.20.2"];

    Command::new("git")
        .args(["submodule", "update", "--init"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let mut depends = Vec::new();

    let versions: HashMap<&str, Version> = VERSIONS
        .iter()
        .map(|&x| (x, new(x, mc_data_path, &mut depends)))
        .collect();

    if false {
        process_items(&versions, &mut depends);
        process_enchants(&versions, &mut depends);
    }

    for v in VERSIONS {
        protocol::run(v, &versions[v].protocol_path, out_dir, &mut depends);
    }

    depends
}
