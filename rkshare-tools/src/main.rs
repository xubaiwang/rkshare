//! 本模块实现了 rkshare 的基本 CLI, server 等.

use cli::Args;

mod cli;
mod pretty;

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();
    args.action()?;

    Ok(())
}
