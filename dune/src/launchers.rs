use anyhow::{bail, Result};
use melon::record::AuthData;
use serde_derive::Deserialize;
use std::{env, fs::File};

pub struct AuthDataExt {
    pub data: AuthData,
    pub name: String,
    pub online: bool,
}

fn get_access_token_tlauncher() -> Result<AuthDataExt> {
    let path = env::var("appdata")? + "/.minecraft/TlauncherProfiles.json";
    let file = File::open(path)?;
    let value: serde_json::Value = serde_json::from_reader(file)?;

    let selected_acc = value.get("selectedAccountUUID").unwrap().as_str().unwrap();
    let accounts = value.get("accounts").unwrap().as_object().unwrap();
    let acc = accounts.get(selected_acc).unwrap().as_object().unwrap();
    let token = acc.get("accessToken").unwrap().as_str().unwrap();

    Ok(AuthDataExt {
        data: AuthData {
            selected_profile: selected_acc.to_string(),
            access_token: token.to_string(),
        },
        name: "".to_string(),
        online: false,
    })
}

#[derive(Deserialize)]
struct PolyProfile<'x> {
    name: &'x str,
    id: &'x str,
}
#[derive(Deserialize)]
struct PolyYgg<'x> {
    token: &'x str,
}
#[derive(Deserialize)]
struct PolyAccount<'x> {
    #[serde(borrow)]
    profile: PolyProfile<'x>,
    #[serde(borrow)]
    ygg: PolyYgg<'x>,
    active: Option<bool>,
    #[serde(rename = "type")]
    ty: &'x str,
}
#[derive(Deserialize)]
struct PolyJson<'x> {
    #[serde(borrow)]
    accounts: Vec<PolyAccount<'x>>,
}

fn get_access_token_polymc() -> Result<AuthDataExt> {
    let path = env::var("appdata")? + "/PolyMC/accounts.json";
    let content = std::fs::read_to_string(path)?;
    let value: PolyJson = serde_json::from_str(&content)?;
    let acc = value.accounts.iter().find(|x| x.active.unwrap_or(false));
    let acc = match acc {
        Some(x) => x,
        None => bail!("there should be at least an account"),
    };
    let online = match acc.ty {
        "Mojang" | "MSA" => true,
        "Offline" => false,
        _ => bail!("unknown account type {}", acc.ty),
    };

    Ok(AuthDataExt {
        data: AuthData {
            selected_profile: acc.profile.id.to_string(),
            access_token: acc.ygg.token.to_string(),
        },
        name: acc.profile.name.to_string(),
        online,
    })
}

pub fn get_access_token() -> Result<AuthDataExt> {
    if let Ok(x) = get_access_token_tlauncher() {
        return Ok(x);
    }
    get_access_token_polymc()
}
