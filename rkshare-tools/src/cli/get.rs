use argh::FromArgs;
use rkshare::{
    eastmoney::cli::Eastmoney,
    utils::{
        data::{Data, Fetch},
        pretty::pretty_print,
    },
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
    Eastmoney(Eastmoney),
    // Xueqiu(GetXueqiu),
}

impl Get {
    pub fn action(self) -> anyhow::Result<()> {
        let data = rt()?.block_on(self.fetch())?;
        pretty_print(data)?;
        Ok(())
    }
}

impl Fetch for Get {
    fn fetch(self) -> impl Future<Output = anyhow::Result<Data>> {
        match self.subcommand {
            Subcommand::Eastmoney(eastmoney) => eastmoney.fetch(),
        }
    }
}

fn rt() -> tokio::io::Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
}
