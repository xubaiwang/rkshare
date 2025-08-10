use std::str::FromStr;

use argh::FromArgs;

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
