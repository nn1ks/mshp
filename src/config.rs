use serde::de::{Deserializer, Error, Unexpected};
use serde::Deserialize;
use yansi::Color;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(deserialize_with = "deserialize_color")]
    pub cwd_color: Color,
    // #[serde(deserialize_with = "deserialize_bool")]
    // pub disable_git: bool,
    pub git_branch_icon: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub git_branch_color: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub git_branch_disable: bool,
    pub git_staged_icon: String,
    pub git_unstaged_icon: String,
    pub git_untracked_icon: String,
    pub git_ahead_icon: String,
    pub git_behind_icon: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub git_status_color: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub git_status_disable: bool,
    pub user_indicator: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub user_indicator_color: Color,
    pub root_indicator: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub root_indicator_color: Color,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cwd_color: Color::Blue,
            git_branch_icon: "".to_owned(),
            git_branch_color: Color::Cyan,
            git_branch_disable: false,
            git_staged_icon: "+".to_owned(),
            git_unstaged_icon: "!".to_owned(),
            git_untracked_icon: "?".to_owned(),
            git_ahead_icon: "↥".to_owned(),
            git_behind_icon: "↧".to_owned(),
            git_status_color: Color::Cyan,
            git_status_disable: false,
            user_indicator: "$".to_owned(),
            user_indicator_color: Color::Green,
            root_indicator: "#".to_owned(),
            root_indicator_color: Color::Green,
        }
    }
}

fn deserialize_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let string = String::deserialize(deserializer)?;
    Ok(match string.as_ref() {
        "1" => true,
        _ => false,
    })
}

fn deserialize_color<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Color, D::Error> {
    let string = String::deserialize(deserializer)?;
    match string.to_lowercase().as_ref() {
        "default" | "" => Ok(Color::Default),
        "black" => Ok(Color::Black),
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::Blue),
        "magenta" => Ok(Color::Magenta),
        "cyan" => Ok(Color::Cyan),
        "white" => Ok(Color::White),
        _ => {
            if string.starts_with('#') {
                match string.len() {
                    4 => {
                        let r = u8::from_str_radix(&string[1..2], 16).map_err(D::Error::custom)?;
                        let g = u8::from_str_radix(&string[2..3], 16).map_err(D::Error::custom)?;
                        let b = u8::from_str_radix(&string[3..4], 16).map_err(D::Error::custom)?;
                        Ok(Color::RGB(r * 16 + r, g * 16 + g, b * 16 + b))
                    }
                    7 => {
                        let r = u8::from_str_radix(&string[1..3], 16).map_err(D::Error::custom)?;
                        let g = u8::from_str_radix(&string[3..5], 16).map_err(D::Error::custom)?;
                        let b = u8::from_str_radix(&string[5..7], 16).map_err(D::Error::custom)?;
                        Ok(Color::RGB(r, g, b))
                    }
                    _ => Err(D::Error::invalid_value(
                        Unexpected::Str(&string),
                        &"expected a string with 4 or 7 characters including the `#`",
                    )),
                }
            } else if string.chars().all(|c| c.is_numeric()) {
                Ok(Color::Fixed(string.parse().map_err(D::Error::custom)?))
            } else {
                let expected = format!(
                    "expected a hex color beginning with `#` or one of `unset`, `default`, {}",
                    "`black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`"
                );
                Err(D::Error::invalid_value(
                    Unexpected::Str(&string),
                    &expected.as_ref(),
                ))
            }
        }
    }
}
