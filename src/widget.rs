use crate::themes::Theme;
use serde_json::value::Value;

#[derive(Debug, Copy, Clone)]
pub enum State {
    Idle,
    Info,
    Good,
    Warning,
    Critical,
}

impl State {
    pub fn theme_keys(self, theme: &Theme) -> (&String, &String) {
        use self::State::*;
        match self {
            Idle => (&theme.idle_bg, &theme.idle_fg),
            Info => (&theme.info_bg, &theme.info_fg),
            Good => (&theme.good_bg, &theme.good_fg),
            Warning => (&theme.warning_bg, &theme.warning_fg),
            Critical => (&theme.critical_bg, &theme.critical_fg),
        }
    }
}

pub trait I3BarWidget {
    fn to_string(&self) -> String;
    fn get_rendered(&self) -> &Value;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="camelCase")]
pub enum Align {
    Left,
    Center,
    Right
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct BaseConfig {
    min_width: Option<usize>,   
    align: Option<Align>,
}

impl BaseConfig {
    pub fn update(&self, value: &mut Value) {
        if let Some(v) = value.as_object_mut() {
            if let Some(min_width) = self.min_width {
                v.insert("min_width".into(), min_width.into());
            }
            if let Some(ref align) = self.align {
                v.insert("align".into(), serde_json::to_value(&align).unwrap());
            }
        }
    }
}

