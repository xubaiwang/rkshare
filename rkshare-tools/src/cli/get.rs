use argh::FromArgs;
use rkshare::{
    eastmoney::cli::Eastmoney,
    sse::cli::Sse,
    utils::{
        data::{Data, Fetch},
        pretty::pretty_print,
    },
    xueqiu::cli::Xueqiu,
};

#[derive(FromArgs, Debug)]
/// 请求数据
#[argh(subcommand, name = "get")]
pub struct Get {
    #[argh(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Subcommand {
    Sse(Sse),
    Eastmoney(Eastmoney),
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
        match self.subcommand {
            Subcommand::Sse(sse) => sse.fetch().await,
            Subcommand::Eastmoney(eastmoney) => eastmoney.fetch().await,
            Subcommand::Xueqiu(xueqiu) => xueqiu.fetch().await,
        }
    }
}

fn rt() -> tokio::io::Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
}
