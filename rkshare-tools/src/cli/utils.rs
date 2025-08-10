use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// 小工具
#[argh(subcommand, name = "utils")]
pub struct Utils {
    #[argh(subcommand)]
    pub command: Command,
}

/// 小工具子命令。
#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Command {
    /// 股票代码转换。
    Convert(rkshare::utils::cli::Convert),
}

impl Utils {
    pub fn action(&self) -> anyhow::Result<()> {
        match &self.command {
            // TODO: 应该让其满足 Error trait 以避免 unwrap
            Command::Convert(convert) => convert.action().unwrap(),
        }
        Ok(())
    }
}
