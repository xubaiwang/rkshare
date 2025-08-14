use anyhow::Result;
use rkshare_utils::data::{Data, Fetch};

/// 东方财富
#[derive(clap::Subcommand, Debug)]
#[command(rename_all = "snake_case")]
pub enum Eastmoney {
    BasicOrgInfo(crate::basic_org_info::Args),
    CenterGridlist(crate::center_gridlist::Args),
}

impl Fetch for Eastmoney {
    async fn fetch(self) -> Result<Data> {
        match self {
            Eastmoney::BasicOrgInfo(args) => args.fetch().await,
            Eastmoney::CenterGridlist(args) => args.fetch().await,
        }
    }
}
