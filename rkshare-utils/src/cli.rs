use std::{marker::PhantomData, str::FromStr};

use argh::{FromArgValue, FromArgs};
use serde::{Deserialize, Serialize};

use crate::Symbol;

/// 股票代码格式转换
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "convert")]
pub struct Convert {
    /// 股票代码
    #[argh(positional)]
    pub symbol: String,
}

impl Convert {
    /// 运行。
    pub fn action(&self) -> Result<(), String> {
        let symbol = Symbol::from_str(&self.symbol)?;
        println!("{}", symbol.to_extended());

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhantomArg<Flatten>(PhantomData<Flatten>);

impl<Flatten> Default for PhantomArg<Flatten> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Flatten> FromArgValue for PhantomArg<Flatten> {
    fn from_arg_value(_value: &str) -> std::result::Result<Self, String> {
        Ok(Self::default())
    }
}
