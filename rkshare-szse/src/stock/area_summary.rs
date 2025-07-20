use arrow::{array::RecordBatch, datatypes::FieldRef};
use bytes::Bytes;
use serde::{Deserialize, Serialize, de::IgnoredAny};
use serde_arrow::schema::SchemaLike;
use url::Url;

use crate::utils::{CommonMessage, try_from_str, try_from_str_comma};

pub async fn raw(date: &str, page: u8) -> anyhow::Result<Bytes> {
    let url = Url::parse_with_params(
        "http://www.szse.cn/api/report/ShowReport/data",
        [
            ("SHOWTYPE", "JSON"),
            ("CATALOGID", "1803_sczm"),
            ("TABKEY", "tab2"),
            ("PAGENO", &format!("{page}")),
            ("DATETIME", &[&date[..4], &date[4..6]].join("-")),
            ("random", "0.33601988010392003"),
        ],
    )?;

    let bytes = reqwest::get(url).await?.bytes().await?;
    Ok(bytes)
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    #[serde(rename(serialize = "序号"), deserialize_with = "try_from_str")]
    rowid: Option<u8>,
    #[serde(rename(serialize = "地区"))]
    dq: String,
    #[serde(
        rename(serialize = "总交易额(亿元)"),
        deserialize_with = "try_from_str_comma"
    )]
    jyje: Option<f64>,
    #[serde(rename(serialize = "占市场%"), deserialize_with = "try_from_str_comma")]
    zsc: Option<f64>,
    #[serde(
        rename(serialize = "股票交易额(亿元)"),
        deserialize_with = "try_from_str_comma"
    )]
    gbjye: Option<f64>,
    #[serde(
        rename(serialize = "基金交易额(亿元)"),
        deserialize_with = "try_from_str_comma"
    )]
    jjjye: Option<f64>,
    #[serde(
        rename(serialize = "债券交易额(亿元)"),
        deserialize_with = "try_from_str_comma"
    )]
    zqjye: Option<f64>,
    #[serde(
        rename(serialize = "优先股交易额(亿元)"),
        deserialize_with = "try_from_str_comma"
    )]
    yxgjye: Option<f64>,
    #[serde(
        rename(serialize = "期权交易额(亿元)"),
        deserialize_with = "try_from_str_comma"
    )]
    qqjye: Option<f64>,
}

pub async fn arrow(date: &str) -> anyhow::Result<RecordBatch> {
    // deserialize
    let raw_to_items = |raw: Bytes| -> anyhow::Result<Vec<Item>> {
        Ok(
            serde_json::from_slice::<(IgnoredAny, CommonMessage<Vec<Item>>)>(&raw)?
                .1
                .data,
        )
    };

    // page 1
    let raw_1 = raw(date, 1).await?;
    let mut items = raw_to_items(raw_1)?;

    // page 2
    let raw_2 = raw(date, 2).await?;
    items.extend(raw_to_items(raw_2)?);

    let fields = Vec::<FieldRef>::from_samples(&items, Default::default())?;
    let batch = serde_arrow::to_record_batch(&fields, &items)?;

    Ok(batch)
}
