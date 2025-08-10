//! 股票代码转换。
//!
// TODO: add example to module doc

use std::str::FromStr;

use derive_more::Display;

/// 股票代码后缀。
#[derive(Debug, Clone, Display)]
pub enum SymbolExtension {
    /// 上海。
    // TODO: 等待 rename_all 特性
    #[display("SH")]
    Sh,
    /// 深圳。
    #[display("SZ")]
    Sz,
    /// 北京。
    #[display("BJ")]
    Bj,
    /// 香港。
    #[display("HK")]
    Hk,
    // TODO: 美股似乎格式不同
}

impl SymbolExtension {
    /// 从原始格式中猜测拓展。
    pub fn guess_from_raw(raw: &str) -> Option<Self> {
        // 沪深京港
        if raw.chars().all(|c| c.is_digit(10)) {
            // 沪深京
            if raw.len() == 6 {
                match raw.chars().next().unwrap() {
                    '6' | '9' => Some(Self::Sh),
                    '0' | '2' | '3' => Some(Self::Sz),
                    '8' => Some(Self::Bj),
                    _ => None,
                }
            }
            // 港
            else if raw.len() == 5 {
                None
            } else {
                None
            }
        } else {
            // TODO: 美股暂不处理
            None
        }
    }
}

impl FromStr for SymbolExtension {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sh" | "Sh" | "SH" => Ok(Self::Sh),
            "sz" | "Sz" | "SZ" => Ok(Self::Sz),
            "bj" | "Bj" | "BJ" => Ok(Self::Bj),
            "hk" | "Hk" | "HK" => Ok(Self::Hk),
            _ => Err(s.to_string()),
        }
    }
}

/// 股票代码。
#[derive(Debug, Clone, Display)]
#[display("{raw}.{extension}")]
pub struct Symbol {
    /// 交易所原始格式。
    raw: String,
    /// 拓展后缀。
    extension: SymbolExtension,
}

impl Symbol {
    /// 转化为拓展格式。
    pub fn to_extended(&self) -> String {
        format!("{}.{}", self.raw, self.extension)
    }

    /// 转化为交易所原始格式。
    pub fn as_raw(&self) -> &str {
        &self.raw
    }
}

impl FromStr for Symbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('.') {
            //
            Some((raw, extension)) => {
                let extension = SymbolExtension::from_str(extension)?;
                // XXX: check raw
                Ok(Self {
                    raw: raw.to_string(),
                    extension,
                })
            }
            // raw
            None => {
                let extension = SymbolExtension::guess_from_raw(s).ok_or_else(|| s.to_string())?;
                Ok(Self {
                    raw: s.to_string(),
                    extension,
                })
            }
        }
    }
}

impl TryFrom<&str> for Symbol {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}
