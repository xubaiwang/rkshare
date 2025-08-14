/// 小工具
#[derive(clap::Subcommand, Debug)]
pub enum Utils {
    /// 股票代码格式转换
    Convert(rkshare::utils::cli::Convert),
}

impl Utils {
    pub fn action(&self) -> anyhow::Result<()> {
        match self {
            // TODO: 应该让其满足 Error trait 以避免 unwrap
            Self::Convert(convert) => convert.action().unwrap(),
        }
        Ok(())
    }
}
