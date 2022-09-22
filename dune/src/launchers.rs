use anyhow::{bail, Result};
use melon::record::AuthData;
use serde_derive::Deserialize;
use std::env;

pub struct AuthDataExt {
    pub data: AuthData,
    pub name: String,
    pub online: bool,
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
    #[serde(rename = "type")]
    ty: &'x str,
}
#[derive(Deserialize)]
struct PolyJson<'x> {
    #[serde(borrow)]
    accounts: Vec<PolyAccount<'x>>,
}

pub fn get_access_token(profile: &str) -> Result<AuthDataExt> {
    let path = env::var("appdata")? + "/PolyMC/accounts.json";
    let content = std::fs::read_to_string(path)?;
    let value: PolyJson = serde_json::from_str(&content)?;
    let acc = value.accounts.iter().find(|x| x.profile.name == profile);
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
