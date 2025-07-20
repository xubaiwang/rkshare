use arrow::{array::RecordBatch, datatypes::FieldRef};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_arrow::schema::{SchemaLike, TracingOptions};
use url::Url;

use crate::utils::{CommonMessage, configured_client, try_from_str};

pub async fn raw(date: &str) -> anyhow::Result<Bytes> {
    let url = Url::parse_with_params(
        "http://query.sse.com.cn/commonQuery.do",
        &[
            ("sqlId", "COMMON_SSE_SJ_GPSJ_CJGK_MRGK_C"),
            ("PRODUCT_CODE", "01,02,03,11,17"),
            ("type", "inParams"),
            (
                "SEARCH_DATE",
                &[&date[..4], &date[4..6], &date[6..]].join("-"),
            ),
        ],
    )?;

    let raw_bytes = configured_client()?.get(url).send().await?.bytes().await?;

    Ok(raw_bytes)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ProductCode {
    #[serde(rename(serialize = "主板A", deserialize = "01"))]
    MainA,
    #[serde(rename(serialize = "主板B", deserialize = "02"))]
    MainB,
    #[serde(rename(serialize = "科创版", deserialize = "03"))]
    Star,
    #[serde(rename(serialize = "股票回购", deserialize = "11"))]
    StockRepurchase,
    #[serde(rename(serialize = "股票", deserialize = "17"))]
    Stock,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
struct Item {
    #[serde(rename(serialize = "数据日期"))]
    trade_date: String,
    #[serde(rename(serialize = "证券类别"))]
    product_code: ProductCode,
    #[serde(rename(serialize = "挂牌数"), deserialize_with = "try_from_str")]
    list_num: Option<u32>,
    #[serde(
        rename(serialize = "市场总值(亿元)"),
        deserialize_with = "try_from_str"
    )]
    total_value: Option<f64>,
    #[serde(
        rename(serialize = "流通市值(亿元)"),
        deserialize_with = "try_from_str"
    )]
    nego_value: Option<f64>,
    #[serde(
        rename(serialize = "成交金额(亿元)"),
        deserialize_with = "try_from_str"
    )]
    trade_amt: Option<f64>,
    #[serde(
        rename(serialize = "成交量(亿股/亿份)"),
        deserialize_with = "try_from_str"
    )]
    trade_vol: Option<f64>,
    #[serde(
        rename(serialize = "平均市盈率(倍)"),
        deserialize_with = "try_from_str"
    )]
    avg_pe_rate: Option<f64>,
    #[serde(rename(serialize = "换手率(%)"), deserialize_with = "try_from_str")]
    total_to_rate: Option<f64>,
    #[serde(rename(serialize = "流通换手率(%)"), deserialize_with = "try_from_str")]
    nego_to_rate: Option<f64>,
    // NOTE: 以网页未见不录
    // #[serde_as(as = "DisplayFromStr")]
    // trade_num: f64,
}

pub async fn arrow(date: &str) -> anyhow::Result<RecordBatch> {
    let raw = raw(date).await?;
    let items = serde_json::from_slice::<CommonMessage<Vec<Item>>>(&raw)?.result;
    let fields = Vec::<FieldRef>::from_samples(
        &items,
        TracingOptions::default().enums_without_data_as_strings(true),
    )?;
    // TODO: 是否需要确保「股票」和「股票回购」数据项存在？
    let batch = serde_arrow::to_record_batch(&fields, &items)?;
    Ok(batch)
}
