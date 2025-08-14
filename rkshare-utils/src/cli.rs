use std::str::FromStr;

use clap::Args;

use crate::Symbol;

/// 股票代码格式转换
#[derive(Args, Debug)]
#[command(arg_required_else_help(true))]
pub struct Convert {
    symbol: String,
}

impl Convert {
    /// 运行。
    pub fn action(&self) -> Result<(), String> {
        let symbol = Symbol::from_str(&self.symbol)?;
        println!("{}", symbol.to_extended());

        Ok(())
    }
}
