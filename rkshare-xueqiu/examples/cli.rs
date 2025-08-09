use argh::FromArgs;
use arrow::{array::RecordBatch, util::pretty::pretty_format_batches};
use bytes::Bytes;
use parse_display::FromStr;
use rkshare_xq as xq;
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
    Detail,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    match (&args.action, args.raw) {
        (Action::Detail, true) => pretty_print_json(xq::detail::raw("SH601127").await?),
        (Action::Detail, false) => pretty_print_batch(xq::detail::arrow("SH601127").await?),
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
