use std::str::FromStr;

use serde::{Deserialize, Deserializer};

pub(crate) fn try_from_str<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.parse().ok())
}

pub(crate) fn try_from_str_comma<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.replace(",", "").parse().ok())
}

pub(crate) fn str_no_nbsp<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.replace("&nbsp;", ""))
}

#[derive(Deserialize)]
pub(crate) struct CommonMessage<I> {
    pub data: I,
}
