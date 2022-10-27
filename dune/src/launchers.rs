use anyhow::{bail, Result};
use dune_lib::record::AuthData;
use serde_derive::Deserialize;
use std::env;

pub struct AuthDataExt {
    pub data: AuthData,
    pub name: String,
    pub online: bool,
}

#[derive(Deserialize)]
struct PrismProfile<'x> {
    name: &'x str,
    id: &'x str,
}
#[derive(Deserialize)]
struct PrismYgg<'x> {
    token: &'x str,
}
#[derive(Deserialize)]
struct PrismAccount<'x> {
    #[serde(borrow)]
    profile: PrismProfile<'x>,
    #[serde(borrow)]
    ygg: PrismYgg<'x>,
    #[serde(rename = "type")]
    ty: &'x str,
}
#[derive(Deserialize)]
struct PrismJson<'x> {
    #[serde(borrow)]
    accounts: Vec<PrismAccount<'x>>,
}

fn get_access_token_prism(profile: &str, path: &str) -> Result<AuthDataExt> {
    let path = format!("{}/{}/accounts.json", env::var("appdata")? , path);
    let content = std::fs::read_to_string(path)?;
    let value: PrismJson = serde_json::from_str(&content)?;
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

pub fn get_access_token(profile: &str) -> Result<AuthDataExt> {
    if let Ok(x) = get_access_token_prism(profile, "PrismLauncher") {
        return Ok(x);
    }
    if let Ok(x) = get_access_token_prism(profile, "PolyMC") {
        return Ok(x);
    }
    anyhow::bail!("can't find the config of any supported launcher")
}