use anyhow::{bail, Result};
use cfg_if::cfg_if;
use dune_lib::record::AuthData;
use fs_err as fs;
use serde_derive::Deserialize;
use std::path::PathBuf;

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
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            use std::env;
            let mut path = PathBuf::new();
            path.push(env::var("appdata")?);
            path.push(_path);
            path.push("accounts.json");

            Ok(path)
        } else if #[cfg(target_os = "linux")] {
            use std::env;

            let home = env::var("HOME")?;
            Ok(format!("{}/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/accounts.json", home).into())
        } else if #[cfg(target_os = "macos")] {
            use users::{get_current_uid, get_user_by_uid};
            use anyhow::anyhow;

            let user = get_user_by_uid(get_current_uid());
            match user {
                Some(x) => Ok(format!(
                    "/Users/{}/Library/Application Support/{}/accounts.json",
                    x.name()
                        .to_str()
                        .ok_or_else(|| anyhow!("Unknown characters in username"))?,
                        _path
                ).into()),
                None => bail!("can't find the config of any supported launcher")
            }
        } else {
            bail!("Platform is not supported yet!")
        }
    }
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
    let prism;
    match get_access_token_prism(profile, "PrismLauncher") {
        Ok(x) => return Ok(x),
        Err(e) => prism = e,
    }
    let poly;
    match get_access_token_prism(profile, "PolyMC") {
        Ok(x) => return Ok(x),
        Err(e) => poly = e,
    }
    anyhow::bail!(
        "can't find the config of any supported launcher
PrismLauncher: {prism}
PolyMC       : {poly}",
    )
}
