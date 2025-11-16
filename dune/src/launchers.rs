use std::path::PathBuf;

use anyhow::{Result, bail};
use dune_lib::record::AuthData;
use fs_err as fs;
use serde_derive::Deserialize;

pub struct AuthDataExt {
    pub data: AuthData,
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

fn get_access_token_prism_path(_path: &str) -> Result<PathBuf> {
    if cfg!(target_os = "windows") {
        use std::env;

        let mut path = PathBuf::new();
        path.push(env::var("appdata")?);
        path.push(_path);
        path.push("accounts.json");

        return Ok(path);
    }
    if cfg!(target_os = "linux") {
        use std::env;

        let mut path = PathBuf::new();
        path.push(env::var("HOME")?);
        path.push("/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/accounts.json");

        return Ok(path);
    }

    #[cfg(target_os = "macos")]
    {
        use anyhow::anyhow;
        use users::{get_current_uid, get_user_by_uid};

        let Some(user) = get_user_by_uid(get_current_uid()) else {
            bail!("couldn't get current user");
        };
        let mut path = PathBuf::new();

        path.push("/Users");
        path.push(user.name());
        path.push("Library/Application Support");
        path.push(_path);
        path.push("accounts.json");

        return Ok(path);
    }

    bail!("Platform is not supported!")
}

fn get_access_token_prism(profile: &str, path: &str) -> Result<AuthDataExt> {
    let path = get_access_token_prism_path(path)?;
    let content = fs::read_to_string(path)?;
    let value: PrismJson = serde_json::from_str(&content)?;
    let acc = value.accounts.iter().find(|x| x.profile.name == profile);
    let acc = match acc {
        Some(x) => x,
        None => bail!("account with name `{}` not found", profile),
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
            name: acc.profile.name.to_string(),
        },
        online,
    })
}

pub fn get_access_token(profile: &str) -> Result<AuthDataExt> {
    let prism = match get_access_token_prism(profile, "PrismLauncher") {
        Ok(x) => return Ok(x),
        Err(e) => e,
    };
    let poly = match get_access_token_prism(profile, "PolyMC") {
        Ok(x) => return Ok(x),
        Err(e) => e,
    };
    bail!(
        "can't find the config of any supported launcher
PrismLauncher: {prism}
PolyMC       : {poly}",
    )
}
