use std::collections::HashMap;
use serde::Serialize;
use serde::Deserialize;
use std::io::ErrorKind::InvalidData;
use std::ops::Index;
use worker::{console_log, Method};
use worker::js_sys::{Map, TypeError};

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
        if value.is_empty() {
            return Err("Id cannot be empty".into());
        }
        if  value.len() > 254 {
            return Err(format!("Id cannot be longer than 255 characters, got {}", value.len()).into());
        }
        for ch in value.chars() {
            if !ch.is_ascii_alphanumeric() && ch != '-' && ch != '_' {
                return Err(format!(
                    "Id contains invalid character {:?} (U+{:04X})",
                    ch, ch as u32
                ));
            }
        }


        Ok(Self(format!("a{}", value)))

    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::try_from(s).map_err(serde::de::Error::custom)
    }
}


#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Date(String);
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct UTCDate(String);


impl TryFrom<String> for UTCDate {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() < 20 {
            return Err(format!(
                "UTCDate must be at least 20 characters (YYYY-MM-DDTHH:MM:SSZ), got {:?}",
                s
            ));
        }
        let b = s.as_bytes();
        // Tail after the seconds: `Z`, or `.` + non-zero digit run + `Z`.
        let tail_ok = match &b[19..] {
            [b'Z'] => true,
            [b'.', frac @ .., b'Z'] => {
                !frac.is_empty()
                    && frac.iter().all(u8::is_ascii_digit)
                    && frac.iter().any(|&d| d != b'0')
            }
            _ => false,
        };
        // Fixed separators: dashes, T, colons.
        if !tail_ok || b[4] != b'-' || b[7] != b'-' || b[10] != b'T' || b[13] != b':' || b[16] != b':' {
            return Err(format!(
                "UTCDate has wrong structure, expected YYYY-MM-DDTHH:MM:SS[.fff]Z, got {:?}",
                s
            ));
        }
        // Digit positions: 0-3 (year), 5-6 (month), 8-9 (day),
        //                  11-12 (hour), 14-15 (min), 17-18 (sec).
        for &pos in &[0, 1, 2, 3, 5, 6, 8, 9, 11, 12, 14, 15, 17, 18] {
            if !b[pos].is_ascii_digit() {
                return Err(format!(
                    "UTCDate position {} is not a digit in {:?}",
                    pos, s
                ));
            }
        }

        // Value-range checks. Shape is now confirmed, so digit positions
        // are guaranteed ASCII digits and the parses are infallible.
        let parse = |start: usize, end: usize| -> i64 {
            let mut n: i64 = 0;
            for &byte in &b[start..end] {
                n = n * 10 + (byte - b'0') as i64;
            }
            n
        };
        let year = parse(0, 4);
        let month = parse(5, 7) as u32;
        let day = parse(8, 10) as u32;
        let hour = parse(11, 13) as u32;
        let minute = parse(14, 16) as u32;
        let second = parse(17, 19) as u32;

        if !(1..=12).contains(&month) {
            return Err(format!(
                "UTCDate month must be 1..=12, got {month} in {s:?}"
            ));
        }
        let max_day = days_in_month(year, month);
        if !(1..=max_day).contains(&day) {
            return Err(format!(
                "UTCDate day must be 1..={max_day} for {year:04}-{month:02}, got {day} in {s:?}"
            ));
        }
        if hour > 23 {
            return Err(format!(
                "UTCDate hour must be 0..=23, got {hour} in {s:?}"
            ));
        }
        if minute > 59 {
            return Err(format!(
                "UTCDate minute must be 0..=59, got {minute} in {s:?}"
            ));
        }
        // No leap seconds in RFC 8620 §1.4 (it specifies seconds 0..=59).
        if second > 59 {
            return Err(format!(
                "UTCDate second must be 0..=59, got {second} in {s:?}"
            ));
        }
        Ok(Self(s))
    }
}

fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 0, // unreachable when month range is validated by caller
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub email: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub is_primary: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub can_read: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    can_write: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    can_delete: bool,
}



fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = f64::deserialize(deserializer)?;
    Ok(value != 0.0)
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub using: Vec<String>,
    pub method_calls: Vec<(String, serde_json::Value, String)>, // TODO
    pub created_ids: Option<HashMap<Id, Id>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub method_responses: Vec<String>,
    pub created_ids: Option<HashMap<Id, Id>>,
    pub session_state: String,
}

