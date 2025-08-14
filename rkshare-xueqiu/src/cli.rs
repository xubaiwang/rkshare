use clap::Subcommand;
use rkshare_utils::data::Fetch;

/// 雪球
#[derive(Subcommand, Debug)]
pub enum Xueqiu {
    Detail(crate::detail::Args),
}

impl Fetch for Xueqiu {
    fn fetch(
        self,
    ) -> impl std::future::Future<Output = anyhow::Result<rkshare_utils::data::Data>> + Send {
        match self {
            Self::Detail(args) => args.fetch(),
        }
    }
}
