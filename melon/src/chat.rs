use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
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
        name: "multiplayer.player.joined",
        format: "%s joined the game",
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

impl ChatComponent {
    fn to_string_impl(&self, out: &mut String) {
        match self {
            ChatComponent::Text { text, extra } => {
                *out += text;
                for i in extra {
                    i.to_string_impl(out);
                }
            }
            ChatComponent::Translate { format, with } => {
                let mut offset = 0;
                let mut last_percent = false;

                for c in format.chars() {
                    if last_percent {
                        if c == '%' {
                            out.push('%');
                        } else if c == 's' {
                            with[offset].to_string_impl(out);
                            offset += 1;
                        } else {
                            unimplemented!();
                        }

                        last_percent = false;
                    } else if c == '%' {
                        last_percent = true;
                    } else {
                        out.push(c);
                    }
                }
            }
        }
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        self.to_string_impl(&mut s);
        s
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
        text: text.to_string(),
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

    unimplemented!()
}

pub fn parse_chat(input: &str) -> Result<ChatComponent> {
    let j: Value = serde_json::from_str(input)?;
    parse_component(&j)
}
