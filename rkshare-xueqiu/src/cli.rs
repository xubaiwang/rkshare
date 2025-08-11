use argh::FromArgs;
use rkshare_utils::data::Fetch;

#[derive(FromArgs, Debug)]
/// 雪球
#[argh(subcommand, name = "xueqiu")]
pub struct Xueqiu {
    #[argh(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Subcommand {
    Detail(crate::detail::Args),
}

impl Fetch for Xueqiu {
    fn fetch(
        self,
    ) -> impl std::future::Future<Output = anyhow::Result<rkshare_utils::data::Data>> + Send {
        match self.subcommand {
            Subcommand::Detail(args) => args.fetch(),
        }
    }
}
