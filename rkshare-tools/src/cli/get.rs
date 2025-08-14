use rkshare::{
    eastmoney::cli::Eastmoney,
    sse::cli::Sse,
    utils::{
        data::{Data, Fetch},
        pretty::pretty_print,
    },
    xueqiu::cli::Xueqiu,
};

/// 请求数据
#[derive(clap::Subcommand, Debug)]
pub enum Get {
    #[command(subcommand)]
    Sse(Sse),
    #[command(subcommand)]
    Eastmoney(Eastmoney),
    #[command(subcommand)]
    Xueqiu(Xueqiu),
}

impl Get {
    pub fn action(self) -> anyhow::Result<()> {
        let data = rt()?.block_on(self.fetch())?;
        pretty_print(data)?;
        Ok(())
    }
}

impl Fetch for Get {
    async fn fetch(self) -> anyhow::Result<Data> {
        match self {
            Self::Sse(sse) => sse.fetch().await,
            Self::Eastmoney(eastmoney) => eastmoney.fetch().await,
            Self::Xueqiu(xueqiu) => xueqiu.fetch().await,
        }
    }
}

fn rt() -> tokio::io::Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
}
