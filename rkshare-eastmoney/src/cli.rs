use anyhow::Result;
use rkshare_utils::data::{Data, Fetch};

#[derive(clap::Subcommand, Debug)]
/// 东方财富
pub enum Eastmoney {
    BasicOrgInfo(crate::basic_org_info::Args),
}

impl Fetch for Eastmoney {
    async fn fetch(self) -> Result<Data> {
        match self {
            Self::BasicOrgInfo(args) => args.fetch().await,
        }
    }
}
