use anyhow::anyhow;
use anyhow::Result;
use serde_json::Value;

const LANG_ENG: &[(&str, &str)] = &[
    ("chat.type.text", "<%s> %s"),
    ("multiplayer.player.joined", "%s joined the game"),
];

#[derive(Debug)]
pub enum ChatComponent {
    Text(String),
    Translate {
        format: &'static str,
        with: Vec<ChatComponent>,
    },
}

impl ChatComponent {
    fn to_string_impl(&self, out: &mut String) {
        match self {
            ChatComponent::Text(text) => *out += text,
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
                    }
                    else if c == '%' {
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
    let translate_format = LANG_ENG.iter().find(|x| x.0 == translate_spec);
    let translate_format = some_or_err!(
        translate_format,
        format!("no translate with that name: {}", translate_spec)
    )
    .1;

    let mut childs = Vec::new();
    if let Some(with) = input.get("with") {
        let arr = some_or_err!(with.as_array(), "expected array");

        for i in arr {
            childs.push(parse_component(i)?);
        }
    }

    Ok(ChatComponent::Translate {
        format: translate_format,
        with: childs,
    })
}

fn parse_component(input: &Value) -> Result<ChatComponent> {
    if let Some(_) = input.get("translate") {
        return parse_translate(input);
    }
    if let Some(text) = input.get("text") {
        let text = some_or_err!(text.as_str(), "expected string for text component");
        return Ok(ChatComponent::Text(text.to_string()));
    }

    unimplemented!()
}

pub fn parse_chat(input: &str) -> Result<ChatComponent> {
    let j: Value = serde_json::from_str(input)?;
    parse_component(&j)
}
