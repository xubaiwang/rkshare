use anyhow::Result;
use argh::FromArgs;
use rkshare_utils::data::{Data, Fetch};

#[derive(FromArgs, Debug)]
/// 东方财富
#[argh(subcommand, name = "eastmoney")]
pub struct Eastmoney {
    #[argh(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Subcommand {
    BasicOrgInfo(crate::basic_org_info::Args),
}

impl Fetch for Eastmoney {
    async fn fetch(self) -> Result<Data> {
        match self.subcommand {
            Subcommand::BasicOrgInfo(args) => args.fetch().await,
        }
    }
}
