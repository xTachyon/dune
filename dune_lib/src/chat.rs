use std::fmt::{Display, Write};

use anyhow::{Result, anyhow, bail};
use serde_json::Value;

struct LangFormat {
    name: &'static str,
    format: &'static str,
    no_of_captures: usize,
}
const LANG_ENG: &[LangFormat] = &[
    LangFormat {
        name: "chat.type.text",
        format: "<%s> %s",
        no_of_captures: 2,
    },
    LangFormat {
        name: "chat.type.announcement",
        format: "[%s] %s",
        no_of_captures: 2,
    },
    LangFormat {
        name: "multiplayer.player.joined",
        format: "%s joined the game",
        no_of_captures: 1,
    },
    LangFormat {
        name: "multiplayer.player.left",
        format: "%s left the game",
        no_of_captures: 1,
    },
    LangFormat {
        name: "block.minecraft.set_spawn",
        format: "Respawn point set",
        no_of_captures: 0,
    },
    LangFormat {
        name: "sleep.skipping_night",
        format: "Sleeping through this night",
        no_of_captures: 0,
    },
];

#[derive(Debug)]
pub enum ChatComponent {
    Text {
        text: String,
        extra: Vec<ChatComponent>,
    },
    Translate {
        format: &'static str,
        with: Vec<ChatComponent>,
    },
}

impl Display for ChatComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatComponent::Text { text, extra } => {
                f.write_str(text)?;
                for i in extra {
                    f.write_fmt(format_args!("{}", i))?;
                }
            }
            ChatComponent::Translate { format, with } => {
                let mut offset = 0;
                let mut last_percent = false;

                for c in format.chars() {
                    if last_percent {
                        if c == '%' {
                            f.write_char('%')?;
                        } else if c == 's' {
                            f.write_fmt(format_args!("{}", with[offset]))?;
                            offset += 1;
                        } else {
                            unimplemented!();
                        }

                        last_percent = false;
                    } else if c == '%' {
                        last_percent = true;
                    } else {
                        f.write_char(c)?;
                    }
                }
            }
        }
        Ok(())
    }
}

macro_rules! some_or_err {
    ($opt:expr, $err:expr) => {
        match $opt {
            Some(x) => x,
            None => return Err(anyhow!($err)),
        }
    };
}

macro_rules! get_str {
    ($e:expr, $key:expr) => {{
        let s = match $e.get($key) {
            Some(x) => x,
            None => return Err(anyhow!("missing key")),
        };
        match s.as_str() {
            Some(x) => x,
            None => return Err(anyhow!("expected string")),
        }
    }};
}

fn parse_translate(input: &Value) -> Result<ChatComponent> {
    let translate_spec = get_str!(input, "translate");
    let translate_format = LANG_ENG.iter().find(|x| x.name == translate_spec);
    let translate_format = some_or_err!(
        translate_format,
        format!("no translate with that name: {}", translate_spec)
    );

    let arr = match input.get("with") {
        Some(with) => {
            let arr = some_or_err!(with.as_array(), "expected array");
            arr.as_slice()
        }
        None => &[],
    };

    if arr.len() != translate_format.no_of_captures {
        bail!(
            "expected {} captures, found {}",
            translate_format.no_of_captures,
            arr.len()
        );
    }

    let mut childs = Vec::new();
    for i in arr {
        childs.push(parse_component(i)?);
    }

    Ok(ChatComponent::Translate {
        format: translate_format.format,
        with: childs,
    })
}

fn strip_old_control(x: &str) -> String {
    let mut res = String::with_capacity(x.len());

    let mut last_is_control = false;
    for c in x.chars() {
        if last_is_control {
            last_is_control = false;
            continue;
        }
        if c == '§' {
            last_is_control = true;
        } else {
            res.push(c);
        }
    }

    res
}

fn parse_text(input: &Value) -> Result<ChatComponent> {
    let text = get_str!(input, "text");
    let arr = match input.get("extra") {
        Some(x) => {
            let arr = some_or_err!(x.as_array(), "expected array for extra");
            arr.as_slice()
        }
        None => &[],
    };

    let mut extra = Vec::new();
    for i in arr {
        extra.push(parse_component(i)?);
    }

    Ok(ChatComponent::Text {
        text: strip_old_control(text),
        extra,
    })
}

fn parse_component(input: &Value) -> Result<ChatComponent> {
    if input.get("translate").is_some() {
        return parse_translate(input);
    }
    if input.get("text").is_some() {
        return parse_text(input);
    }
    if let Some(text) = input.as_str() {
        return Ok(ChatComponent::Text {
            text: strip_old_control(text),
            extra: vec![],
        });
    }

    unimplemented!()
}

pub fn parse_chat(input: &str) -> Result<ChatComponent> {
    let j: Value = serde_json::from_str(input)?;
    parse_component(&j)
}
