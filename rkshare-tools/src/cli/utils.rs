/// 小工具
#[derive(argh::FromArgs, Debug)]
#[argh(subcommand, name = "utils")]
pub struct Utils {
    #[argh(subcommand)]
    command: Command,
}

#[derive(argh::FromArgs, Debug)]
#[argh(subcommand)]
pub enum Command {
    /// 股票代码格式转换
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
