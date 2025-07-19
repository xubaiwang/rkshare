use rkshare_szse::stock_szse_summary;

#[tokio::main]
async fn main() {
    let df = stock_szse_summary("20240830").await.unwrap();
    dbg!(df);
}
