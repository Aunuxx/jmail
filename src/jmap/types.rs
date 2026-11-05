use serde::Serialize;
use std::io::ErrorKind::InvalidData;
use worker::js_sys::TypeError;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Id(String);

impl Id {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Id {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() < 255 {
            return Err("Id cannot be empty".into());
        }
        for ch in value.chars() {
            if !ch.is_ascii_alphanumeric() && ch != '-' && ch != '_' {
                return Err(format!(
                    "Id contains invalid character {:?} (U+{:04X})",
                    ch, ch as u32
                ));
            }
        }

        Ok(Self(value))
    }
}
