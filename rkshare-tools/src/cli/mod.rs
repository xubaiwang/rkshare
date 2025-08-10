use anyhow::Result;
use argh::FromArgs;

use self::{get::Get, serve::Serve, utils::Utils};

mod get;
mod serve;
mod utils;

#[derive(FromArgs, Debug)]
/// 开源财经数据接口库 RKShare 命令行工具。
pub struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum Subcommand {
    Get(Get),
    Serve(Serve),
    Utils(Utils),
}

impl Args {
    pub fn action(self) -> Result<()> {
        match self.subcommand {
            Subcommand::Get(get) => get.action()?,
            Subcommand::Serve(serve) => serve.action()?,
            Subcommand::Utils(utils) => utils.action()?,
        }
        Ok(())
    }
}
