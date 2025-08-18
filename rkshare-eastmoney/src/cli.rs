use anyhow::Result;
use rkshare_utils::data::{Data, Fetch, HasTypeHint};

/// 东方财富
#[derive(argh::FromArgs, Debug)]
#[argh(subcommand, name = "eastmoney")]
pub struct Eastmoney {
    #[argh(subcommand)]
    command: Command,
}

#[derive(argh::FromArgs, Debug)]
#[argh(subcommand)]
pub enum Command {
    BasicOrgInfo(crate::basic_org_info::Args),
    CenterGridlist(crate::center_gridlist::Args),
}

impl Fetch for Eastmoney {
    async fn fetch(self) -> Result<Data> {
        match self.command {
            Command::BasicOrgInfo(args) => args.fetch().await,
            Command::CenterGridlist(args) => args.fetch().await,
        }
    }
}

impl HasTypeHint for Eastmoney {
    fn type_hint(&self) -> Option<rkshare_utils::data::TypeHint> {
        match &self.command {
            Command::BasicOrgInfo(args) => args.type_hint(),
            Command::CenterGridlist(args) => args.type_hint(),
        }
    }
}
