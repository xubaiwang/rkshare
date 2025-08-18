use anyhow::Result;

use self::{get::Get, serve::Serve, utils::Utils};

mod get;
mod serve;
mod utils;

/// RkShare 开源财经数据接口库——命令行工具。
#[derive(argh::FromArgs, Debug)]
pub struct Cli {
    #[argh(subcommand)]
    commands: Commands,
}

#[derive(argh::FromArgs, Debug)]
#[argh(subcommand)]
enum Commands {
    Get(Get),
    Serve(Serve),
    Utils(Utils),
}

impl Cli {
    pub fn action(self) -> Result<()> {
        match self.commands {
            Commands::Get(get) => get.action()?,
            Commands::Serve(serve) => serve.action()?,
            Commands::Utils(utils) => utils.action()?,
        }
        Ok(())
    }
}
