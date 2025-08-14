use clap::Subcommand;
use rkshare_utils::data::{Fetch, HasTypeHint};

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

impl HasTypeHint for Xueqiu {
    fn type_hint(&self) -> Option<rkshare_utils::data::TypeHint> {
        match self {
            Xueqiu::Detail(args) => args.type_hint(),
        }
    }
}
