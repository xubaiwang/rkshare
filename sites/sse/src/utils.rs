use std::str::FromStr;

use reqwest::{Client, header::HeaderMap};
use serde::{Deserialize, Deserializer};

pub(crate) fn configured_client() -> anyhow::Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert("Referer", "https://www.sse.com.cn".parse()?);
    headers.insert(
        "User-Agent",
        concat!(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
            "AppleWebKit/537.36 (KHTML, like Gecko)",
            "Chrome/89.0.4389.90",
            "Safari/537.36"
        )
        .parse()?,
    );
    let client = Client::builder().default_headers(headers).build()?;
    Ok(client)
}

/// 上交所通用响应消息。
#[derive(Deserialize)]
pub(crate) struct CommonMessage<I> {
    pub result: I,
}

pub(crate) fn try_from_str<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.parse().ok())
}
