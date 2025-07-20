use arrow::{array::RecordBatch, csv::Writer};
use axum::{
    Router,
    extract::Query,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/api/public", public_api_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on 0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}

macro_rules! router_gets {
    ($($name:ident),*$(,)?) => {
        Router::new()
        $(
            .route(concat!("/", stringify!($name)), get($name))
        )*
    };
}

fn public_api_routes() -> Router {
    router_gets!(
        stock_sse_summary,
        stock_sse_deal_daily,
        stock_szse_summary,
        stock_szse_area_summary,
    )
}

#[derive(Deserialize)]
struct DateQuery {
    date: String,
}

async fn stock_sse_summary() -> impl IntoResponse {
    arrow_to_csv_response(rkshare::sse::stock::summary().await.unwrap())
}

async fn stock_sse_deal_daily(query: Query<DateQuery>) -> impl IntoResponse {
    arrow_to_csv_response(rkshare::sse::stock::deal_daily(&query.date).await.unwrap())
}

async fn stock_szse_summary(query: Query<DateQuery>) -> impl IntoResponse {
    arrow_to_csv_response(rkshare::szse::stock::summary(&query.date).await.unwrap())
}

async fn stock_szse_area_summary(query: Query<DateQuery>) -> impl IntoResponse {
    arrow_to_csv_response(
        rkshare::szse::stock::area_summary(&query.date)
            .await
            .unwrap(),
    )
}

fn arrow_to_csv_response(batch: RecordBatch) -> impl IntoResponse {
    let mut writer = Writer::new(Vec::new());
    writer.write(&batch).unwrap();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/csv; charset=utf-8")],
        writer.into_inner(),
    )
}
