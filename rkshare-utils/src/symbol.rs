use std::str::FromStr;

use derive_more::Display;

/// 股票代码后缀。
#[derive(Debug, Clone, Display)]
#[display(rename_all = "UPPERCASE")]
pub enum SymbolExtension {
    /// 上海。
    Sh,
    /// 深圳。
    Sz,
    /// 北京。
    Bj,
    /// 香港。
    Hk,
    // TODO: 美股似乎格式不同
}

impl SymbolExtension {
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
            "sh" | "SH" => Ok(Self::Sh),
            "sz" | "SZ" => Ok(Self::Sz),
            "bj" | "BJ" => Ok(Self::Bj),
            "hk" | "HK" => Ok(Self::Hk),
            _ => Err(s.to_string()),
        }
    }
}

/// 股票代码。
#[derive(Debug, Clone, Display)]
#[display("{raw}.{extension}")]
pub struct Symbol {
    raw: String,
    extension: SymbolExtension,
}

impl Symbol {
    /// 转化为拓展格式
    pub fn to_extended(&self) -> String {
        format!("{}.{}", self.raw, self.extension)
    }
}

impl FromStr for Symbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('.') {
            //
            Some((raw, extension)) => {
                let extension = SymbolExtension::from_str(extension)?;
                // TODO: check raw
                Ok(Self {
                    raw: raw.to_string(),
                    extension,
                })
            }
            // raw
            None => {
                let raw = s;
                let extension =
                    SymbolExtension::guess_from_raw(raw).ok_or_else(|| s.to_string())?;
                Ok(Self {
                    raw: raw.to_string(),
                    extension,
                })
            }
        }
    }
}
