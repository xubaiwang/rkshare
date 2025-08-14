use rkshare_utils::data::Fetch;

#[derive(clap::Subcommand, Debug)]
/// 上海证券交易所
pub enum Sse {
    #[command(subcommand)]
    Stock(Stock),
}

impl Fetch for Sse {
    fn fetch(
        self,
    ) -> impl std::future::Future<Output = anyhow::Result<rkshare_utils::data::Data>> + Send {
        match self {
            Self::Stock(stock) => stock.fetch(),
        }
    }
}

#[derive(clap::Subcommand, Debug)]
/// 股票市场
pub enum Stock {
    Summary(crate::stock::summary::Args),
    DealDaily(crate::stock::deal_daily::Args),
}

impl Fetch for Stock {
    async fn fetch(self) -> anyhow::Result<rkshare_utils::data::Data> {
        match self {
            Self::Summary(args) => args.fetch().await,
            Self::DealDaily(args) => args.fetch().await,
        }
    }
}
