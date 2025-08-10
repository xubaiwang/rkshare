use std::time::Duration;

use argh::FromArgs;

/// 启动 API 服务器（暂不可用）
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "serve")]
pub struct Serve {}

impl Serve {
    pub fn action(&self) -> anyhow::Result<()> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        println!("暂未实现，假装服务正在运行，未来应该在 rkshare-api 实现");
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(10)).await;
                println!("假装请求传入");
            }
        });
        Ok(())
    }
}
