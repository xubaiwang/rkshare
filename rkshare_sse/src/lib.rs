use std::{error::Error, io::Cursor};

use either::Either;
use polars::{io::SerReader, prelude::*};
use reqwest::Client;
use serde::Deserialize;
use serde_json::value::RawValue;

pub async fn stock_sse_summary() -> Result<DataFrame, Box<dyn Error>> {
    let client = Client::new();
    let message: Message = client
        .get(url())
        .header("Referer", REFERER)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?
        .json()
        .await?;

    const NAMES: &[(&str, &str)] = &[
        ("NEGO_ISSUE_VOL", "流通股本"),
        ("TOTAL_VALUE", "总市值"),
        ("AVG_PE_RATIO", "平均市盈率"),
        ("LIST_COM_NUM", "上市公司"),
        ("SECURITY_NUM", "上市股票"),
        ("NEGO_VALUE", "流通市值"),
        ("TRADE_DATE", "报告时间"),
        ("TOTAL_ISSUE_VOL", "总股本"),
        // NOTE: 即「主板」等分类
        ("PRODUCT_NAME", "分类名称"),
    ];

    let mut df = JsonReader::new(Cursor::new(message.result.get())).finish()?;

    for (old_name, new_name) in NAMES {
        df.rename(old_name, (*new_name).into())?;
    }

    let df = df
        .select(NAMES.into_iter().map(|x| x.1))?
        .transpose(Some("项目"), Some(Either::Left("分类名称".into())))?;

    Ok(df)
}

const REFERER: &str = "https://www.sse.com.cn";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36";

fn url() -> String {
    let base = "http://query.sse.com.cn/commonQuery.do";
    let sql_id = "COMMON_SSE_SJ_GPSJ_GPSJZM_TJSJ_L";
    let product_name = "股票,主板,科创板";
    let r#type = "inParams";
    let url = format!("{base}?sqlId={sql_id}&PRODUCT_NAME={product_name}&type={type}");
    url
}

/// 解析响应消息
#[derive(Debug, Deserialize)]
struct Message {
    /// 延迟解析，交予 polars
    result: Box<RawValue>,
}
