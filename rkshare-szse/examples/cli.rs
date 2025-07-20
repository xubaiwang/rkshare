use argh::FromArgs;
use arrow::{array::RecordBatch, util::pretty::pretty_format_batches};
use bytes::Bytes;
use parse_display::FromStr;
use rkshare_szse::stock;
use serde_json::Value;

#[derive(FromArgs, PartialEq, Debug)]
/// CLI
struct Args {
    /// raw or not
    #[argh(switch)]
    raw: bool,
    #[argh(positional)]
    action: Action,
}

#[derive(FromStr, Debug, PartialEq)]
#[display(style = "kebab-case")]
enum Action {
    StockSummary,
    StockAreaSummary,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    match (&args.action, args.raw) {
        (Action::StockSummary, true) => pretty_print_json(stock::summary::raw("20250715").await?),
        (Action::StockSummary, false) => pretty_print_batch(stock::summary("20250715").await?),
        (Action::StockAreaSummary, true) => {
            pretty_print_json(stock::area_summary::raw("20250715", 0).await?)
        }
        (Action::StockAreaSummary, false) => {
            pretty_print_batch(stock::area_summary("202505").await?)
        }
    }?;

    Ok(())
}

fn pretty_print_json(raw: Bytes) -> anyhow::Result<()> {
    let value: Value = serde_json::from_slice(&raw)?;
    let formatted = serde_json::to_string_pretty(&value)?;
    println!("{formatted}");
    Ok(())
}

fn pretty_print_batch(batch: RecordBatch) -> anyhow::Result<()> {
    println!("{}", pretty_format_batches(&[batch])?);
    Ok(())
}
