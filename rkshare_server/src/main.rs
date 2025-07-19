use std::io::Cursor;

use axum::{
    Router,
    extract::Query,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use polars::{frame::DataFrame, io::SerWriter, prelude::CsvWriter};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/api/public", public_api_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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
    df_to_csv_response(rkshare::stock_sse_summary().await.unwrap())
}

async fn stock_sse_deal_daily(query: Query<DateQuery>) -> impl IntoResponse {
    df_to_csv_response(rkshare::stock_sse_deal_daily(&query.date).await.unwrap())
}

async fn stock_szse_summary(query: Query<DateQuery>) -> impl IntoResponse {
    df_to_csv_response(rkshare::stock_szse_summary(&query.date).await.unwrap())
}

async fn stock_szse_area_summary(query: Query<DateQuery>) -> impl IntoResponse {
    df_to_csv_response(rkshare::stock_szse_area_summary(&query.date).await.unwrap())
}

fn df_to_csv_response(mut df: DataFrame) -> impl IntoResponse {
    let mut cursor = Cursor::new(Vec::new());
    CsvWriter::new(&mut cursor)
        .include_header(true)
        .finish(&mut df)
        .unwrap();
    let inner = cursor.into_inner();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/csv; charset=utf-8")],
        inner,
    )
}
