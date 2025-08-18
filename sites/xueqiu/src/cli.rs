use rkshare_shared::data::{Fetch, HasTypeHint};

/// 雪球
#[derive(argh::FromArgs, Debug)]
#[argh(subcommand, name = "xueqiu")]
pub struct Xueqiu {
    #[argh(subcommand)]
    pub command: Command,
}

#[derive(argh::FromArgs, Debug)]
#[argh(subcommand)]
pub enum Command {
    Detail(crate::detail::Args),
}

impl Fetch for Xueqiu {
    fn fetch(
        self,
    ) -> impl std::future::Future<Output = anyhow::Result<rkshare_shared::data::Data>> + Send {
        match self.command {
            Command::Detail(args) => args.fetch(),
        }
    }
}

impl HasTypeHint for Xueqiu {
    fn type_hint(&self) -> Option<rkshare_shared::data::TypeHint> {
        match &self.command {
            Command::Detail(args) => args.type_hint(),
        }
    }
}
