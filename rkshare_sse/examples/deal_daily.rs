use rkshare_sse::stock_sse_deal_daily;

#[tokio::main]
async fn main() {
    let df = stock_sse_deal_daily("20250714").await.unwrap();
    println!("{}", df);
    println!("{:?}", df.get_column_names());
}
