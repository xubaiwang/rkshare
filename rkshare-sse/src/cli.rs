use argh::FromArgs;
use rkshare_utils::data::Fetch;

#[derive(FromArgs, Debug)]
/// 上海证券交易所
#[argh(subcommand, name = "sse")]
pub struct Sse {
    #[argh(subcommand)]
    pub subcommand: Subcommand,
}

impl Fetch for Sse {
    fn fetch(
        self,
    ) -> impl std::future::Future<Output = anyhow::Result<rkshare_utils::data::Data>> + Send {
        match self.subcommand {
            Subcommand::Stock(stock) => stock.fetch(),
        }
    }
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Subcommand {
    Stock(Stock),
}

#[derive(FromArgs, Debug)]
/// 股票市场
#[argh(subcommand, name = "stock")]
pub struct Stock {
    #[argh(subcommand)]
    pub subcommand: StockSubcommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum StockSubcommand {
    Summary(crate::stock::summary::Args),
    DealDaily(crate::stock::deal_daily::Args),
}

impl Fetch for Stock {
    async fn fetch(self) -> anyhow::Result<rkshare_utils::data::Data> {
        match self.subcommand {
            StockSubcommand::Summary(args) => args.fetch().await,
            StockSubcommand::DealDaily(args) => args.fetch().await,
        }
    }
}
