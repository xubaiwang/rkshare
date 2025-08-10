use anyhow::Result;
use argh::FromArgs;
use rkshare_utils::data::Data;

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
    BasicOrgInfo(crate::basic_org_info::cli::Args),
}

impl Eastmoney {
    pub async fn call(self) -> Result<Data> {
        match self.subcommand {
            Subcommand::BasicOrgInfo(args) => args.call().await,
        }
    }
}
