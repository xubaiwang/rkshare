use rkshare_shared::data::{Fetch, HasTypeHint};

/// 上海证券交易所
#[derive(argh::FromArgs, Debug)]
#[argh(subcommand, name = "sse")]
pub struct Sse {
    #[argh(subcommand)]
    command: Command,
}

#[derive(argh::FromArgs, Debug)]
#[argh(subcommand)]
pub enum Command {
    Stock(Stock),
}

impl Fetch for Sse {
    fn fetch(
        self,
    ) -> impl std::future::Future<Output = anyhow::Result<rkshare_shared::data::Data>> + Send {
        match self.command {
            Command::Stock(stock) => stock.fetch(),
        }
    }
}

/// 股票市场
#[derive(argh::FromArgs, Debug)]
#[argh(subcommand, name = "stock")]
pub struct Stock {
    #[argh(subcommand)]
    command: StockCommand,
}

/// 股票市场
#[derive(argh::FromArgs, Debug)]
#[argh(subcommand)]
pub enum StockCommand {
    Summary(crate::stock::summary::Args),
    DealDaily(crate::stock::deal_daily::Args),
}

impl Fetch for Stock {
    async fn fetch(self) -> anyhow::Result<rkshare_shared::data::Data> {
        match self.command {
            StockCommand::Summary(args) => args.fetch().await,
            StockCommand::DealDaily(args) => args.fetch().await,
        }
    }
}

impl HasTypeHint for Stock {
    fn type_hint(&self) -> Option<rkshare_shared::data::TypeHint> {
        match &self.command {
            StockCommand::Summary(args) => args.type_hint(),
            StockCommand::DealDaily(args) => args.type_hint(),
        }
    }
}

impl HasTypeHint for Sse {
    fn type_hint(&self) -> Option<rkshare_shared::data::TypeHint> {
        match &self.command {
            Command::Stock(stock) => stock.type_hint(),
        }
    }
}
