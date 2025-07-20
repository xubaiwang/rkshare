use std::error::Error;

use rkshare_sse::stock_sse_summary;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let df = stock_sse_summary().await?;
    println!("{df}");

    Ok(())
}
