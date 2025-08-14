use anyhow::Result;

use self::{get::Get, serve::Serve, utils::Utils};

mod get;
mod serve;
mod utils;

/// RkShare 开源财经数据接口库——命令行工具。
#[derive(clap::Parser, Debug)]
#[command(arg_required_else_help(true), disable_help_subcommand(true))]
pub struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    #[command(subcommand)]
    Get(Get),
    Serve(Serve),
    #[command(subcommand)]
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
