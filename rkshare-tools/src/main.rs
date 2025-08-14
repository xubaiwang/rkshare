//! 本模块实现了 rkshare 的基本 CLI, server 等.

use clap::Parser;
use cli::Cli;

mod cli;

fn main() -> anyhow::Result<()> {
    let args: Cli = Cli::parse();
    args.action()?;

    Ok(())
}
