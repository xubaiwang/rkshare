use arrow::{array::RecordBatch, datatypes::FieldRef};
use bytes::Bytes;
use serde::{Deserialize, Serialize, de::IgnoredAny};
use serde_arrow::schema::SchemaLike;
use url::Url;

use crate::utils::{CommonMessage, str_no_nbsp, try_from_str, try_from_str_comma};

pub async fn raw(date: &str) -> anyhow::Result<Bytes> {
    let url = Url::parse_with_params(
        "http://www.szse.cn/api/report/ShowReport/data",
        [
            ("SHOWTYPE", "JSON"),
            ("CATALOGID", "1803_sczm"),
            ("TABKEY", "tab1"),
            (
                "txtQueryDate",
                &[&date[..4], &date[4..6], &date[6..]].join("-"),
            ),
            ("random", "0.33601988010392003"),
        ],
    )?;

    let bytes = reqwest::get(url).await?.bytes().await?;
    Ok(bytes)
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    #[serde(rename(serialize = "证券类别"), deserialize_with = "str_no_nbsp")]
    lbmc: String,
    #[serde(rename(serialize = "数量(只)"), deserialize_with = "try_from_str")]
    zqsl: Option<u32>,
    #[serde(
        rename(serialize = "成交金额(亿元)"),
        deserialize_with = "try_from_str_comma"
    )]
    cjje: Option<f64>,
    #[serde(
        rename(serialize = "总市值(亿元)"),
        deserialize_with = "try_from_str_comma"
    )]
    sjzz: Option<f64>,
    #[serde(
        rename(serialize = "流通市值(亿元)"),
        deserialize_with = "try_from_str_comma"
    )]
    ltsz: Option<f64>,
}

pub async fn arrow(date: &str) -> anyhow::Result<RecordBatch> {
    let raw = raw(date).await?;
    let items = serde_json::from_slice::<(CommonMessage<Vec<Item>>, IgnoredAny)>(&raw)?
        .0
        .data;
    let fields = Vec::<FieldRef>::from_samples(&items, Default::default())?;
    let batch = serde_arrow::to_record_batch(&fields, &items)?;
    Ok(batch)
}
