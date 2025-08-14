use std::{
    fs::File,
    path::{Path, PathBuf},
};

use arrow::json::writer::JsonArray;
use rkshare::{
    eastmoney::cli::Eastmoney,
    sse::cli::Sse,
    utils::{
        data::{Data, Fetch, HasTypeHint, TypeHint},
        pretty::pretty_print,
    },
    xueqiu::cli::Xueqiu,
};

/// 从各平台接口获取数据
#[derive(clap::Args, Debug)]
pub struct Get {
    // TODO: allow specify format
    /// 输出结果到文件
    #[arg(long, short, global = true)]
    output: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// 上海证券交易所
    #[command(subcommand, visible_aliases = &["sh", "上海证券交易所", "上交所"])]
    Sse(Sse),
    /// 东方财富
    #[command(subcommand, visible_aliases = &["em", "东方财富", "东财"])]
    Eastmoney(Eastmoney),
    /// 雪球
    #[command(subcommand, visible_aliases = &["xq", "雪球"])]
    Xueqiu(Xueqiu),
}

impl HasTypeHint for Get {
    fn type_hint(&self) -> Option<TypeHint> {
        match &self.command {
            Command::Sse(sse) => sse.type_hint(),
            Command::Eastmoney(eastmoney) => eastmoney.type_hint(),
            Command::Xueqiu(xueqiu) => xueqiu.type_hint(),
        }
    }
}

impl Get {
    pub fn action(self) -> anyhow::Result<()> {
        let type_hint = self.type_hint();
        let Self { output, command } = self;
        let format = match &output {
            Some(path) => Some(
                OutputFormat::infer_from_path(path).ok_or(anyhow::anyhow!("无法从路径推断格式"))?,
            ),
            None => None,
        };
        // TODO: early validate

        let data = rt()?.block_on(command.fetch())?;

        match (format, type_hint) {
            (None, None) => {
                pretty_print(data)?;
            }
            // TODO: should split pretty print
            (None, Some(_)) => {
                pretty_print(data)?;
            }
            (Some(format), None) => {
                match format {
                    // TODO: 输出 JSON
                    OutputFormat::Json => {
                        let file = File::create(output.unwrap()).unwrap();
                        let mut writer =
                            arrow::json::WriterBuilder::new().build::<_, JsonArray>(file);
                        writer.write(data.as_arrow().unwrap()).unwrap();
                    }
                    // TODO: 输出 CSV
                    OutputFormat::Csv => {
                        let file = File::create(output.unwrap()).unwrap();
                        let mut writer = arrow::csv::WriterBuilder::new()
                            .with_header(true)
                            .build(file);
                        writer.write(data.as_arrow().unwrap()).unwrap();
                    }
                }
            }
            (Some(_output_format), Some(_data_format)) => {
                // TODO: 验证类型匹配
                std::fs::write(output.unwrap(), &data.as_raw().unwrap())?;
            }
        }
        Ok(())
    }
}

impl Fetch for Command {
    async fn fetch(self) -> anyhow::Result<Data> {
        match self {
            Command::Sse(sse) => sse.fetch().await,
            Command::Eastmoney(eastmoney) => eastmoney.fetch().await,
            Command::Xueqiu(xueqiu) => xueqiu.fetch().await,
        }
    }
}

fn rt() -> tokio::io::Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
}

// TODO: support options like CSV->TSV, json indent and formatting
/// 输出数据格式。
pub enum OutputFormat {
    Json,
    Csv,
    // TODO: excel
    // TODO: parquet
}

impl OutputFormat {
    /// 从文件路径推断数据格式。
    pub fn infer_from_path(path: impl AsRef<Path>) -> Option<Self> {
        match path.as_ref().extension()?.to_str()? {
            "json" => Some(Self::Json),
            "csv" => Some(Self::Csv),
            _ => None,
        }
    }
}
