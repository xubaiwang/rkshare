use rkshare_szse::*;

#[tokio::main]
async fn main() {
    let df = stock_szse_area_summary("20240830").await.unwrap();
    dbg!(df);
}
