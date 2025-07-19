use core::f32;
use std::io::Cursor;

use either::Either;
use polars::{io::SerReader, prelude::*};
use reqwest::{Client, header::HeaderMap};
use serde::Deserialize;
use serde_json::value::RawValue;
use url::Url;

const BASE: &str = "http://query.sse.com.cn/commonQuery.do";

fn configured_client() -> anyhow::Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert("Referer", "https://www.sse.com.cn".parse()?);
    headers.insert(
        "User-Agent",
        concat!(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
            "AppleWebKit/537.36 (KHTML, like Gecko)",
            "Chrome/89.0.4389.90",
            "Safari/537.36"
        )
        .parse()?,
    );
    let client = Client::builder().default_headers(headers).build()?;
    Ok(client)
}

/// SSE 响应通用消息格式。
#[derive(Debug, Deserialize)]
struct SseMessage {
    /// 延迟解析，交予 polars.
    result: Box<RawValue>,
}

pub async fn stock_sse_summary() -> anyhow::Result<DataFrame> {
    let url = Url::parse_with_params(
        BASE,
        &[
            ("sqlId", "COMMON_SSE_SJ_GPSJ_GPSJZM_TJSJ_L"),
            ("PRODUCT_NAME", "股票,主板,科创板"),
            ("type", "inParams"),
        ],
    )?;

    let message = configured_client()?
        .get(url)
        .send()
        .await?
        .json::<SseMessage>()
        .await?;

    let mut df = JsonReader::new(Cursor::new(message.result.get())).finish()?;

    // 预处理

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
    for (old_name, new_name) in NAMES {
        df.rename(old_name, (*new_name).into())?;
    }
    let df = df
        .select(NAMES.into_iter().map(|x| x.1))?
        .transpose(Some("项目"), Some(Either::Left("分类名称".into())))?;

    let df = df
        .lazy()
        .select([
            col("项目"),
            col("股票").cast(DataType::Float64),
            col("主板").cast(DataType::Float64),
            col("科创板").cast(DataType::Float64),
        ])
        .collect()?;

    Ok(df)
}

pub async fn stock_sse_deal_daily(date: &str) -> anyhow::Result<DataFrame> {
    let url = Url::parse_with_params(
        BASE,
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

    let message = configured_client()?
        .get(url)
        .send()
        .await?
        .json::<SseMessage>()
        .await?;

    let mut df = JsonReader::new(Cursor::new(message.result.get())).finish()?;

    df = df.drop_many(["TRADE_NUM", "PRODUCT_CODE"]);
    df.set_column_names([
        "市场总值",
        "成交量",
        "平均市盈率",
        "换手率",
        "成交金额",
        "流通市值",
        "流通换手率",
        "报告日期",
        "挂牌数",
    ])?;

    df = df.transpose(Some("单日情况"), None)?;

    let width = df.width();
    let df = df
        .lazy()
        .select([
            nth(0).alias("单日情况"),
            match width {
                5 => nth(4),
                4 => lit(f32::NAN),
                _ => nth(5),
            }
            .cast(DataType::Float64)
            .alias("股票"),
            nth(1).alias("主板A").cast(DataType::Float64),
            nth(2).alias("主板B").cast(DataType::Float64),
            nth(3).alias("科创板").cast(DataType::Float64),
            if width >= 6 { nth(5) } else { lit(f32::NAN) }
                .alias("股票回购")
                .cast(DataType::Float64),
        ])
        .collect()?;

    Ok(df)
}
