use arrow::{array::RecordBatch, datatypes::FieldRef};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_arrow::schema::SchemaLike;
use url::Url;

use crate::utils::{CommonMessage, configured_client, try_from_str};

pub async fn raw() -> anyhow::Result<Bytes> {
    let url = Url::parse_with_params(
        "http://query.sse.com.cn/commonQuery.do",
        &[
            ("sqlId", "COMMON_SSE_SJ_GPSJ_GPSJZM_TJSJ_L"),
            ("PRODUCT_NAME", "股票,主板,科创板"),
            ("type", "inParams"),
        ],
    )?;

    let bytes = configured_client()?.get(url).send().await?.bytes().await?;
    Ok(bytes)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct Item {
    #[serde(rename(serialize = "数据日期"))]
    trade_date: String,
    #[serde(rename(serialize = "证券类别"))]
    product_name: String,
    #[serde(rename(serialize = "上市公司/家"), deserialize_with = "try_from_str")]
    list_com_num: Option<u32>,
    #[serde(rename(serialize = "上市股票/只"), deserialize_with = "try_from_str")]
    security_num: Option<u32>,
    #[serde(rename(serialize = "总股本/亿股"), deserialize_with = "try_from_str")]
    total_issue_vol: Option<f64>,
    #[serde(rename(serialize = "流通股本/亿股"), deserialize_with = "try_from_str")]
    nego_issue_vol: Option<f64>,
    #[serde(rename(serialize = "总市值/亿元"), deserialize_with = "try_from_str")]
    total_value: Option<f64>,
    #[serde(rename(serialize = "流通市值/亿元"), deserialize_with = "try_from_str")]
    nego_value: Option<f64>,
    #[serde(rename(serialize = "平均市盈率/倍"), deserialize_with = "try_from_str")]
    avg_pe_ratio: Option<f64>,
    // NOTE: 以网页未见不录
    // #[serde(deserialize_with = "parse_f64")]
    // total_trade_amt: f64,
}

pub async fn arrow() -> anyhow::Result<RecordBatch> {
    let raw = raw().await?;
    let items = serde_json::from_slice::<CommonMessage<Vec<Item>>>(&raw)?.result;
    let fields = Vec::<FieldRef>::from_samples(&items, Default::default())?;
    let batch = serde_arrow::to_record_batch(&fields, &items)?;
    Ok(batch)
}
