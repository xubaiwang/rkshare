use std::{error::Error, io::Cursor};

use bytes::Bytes;
use calamine::{Reader, Xlsx, open_workbook_from_rs};
use polars::{
    frame::DataFrame,
    prelude::{DataType, IntoColumn, IntoLazy, NamedFrom, lit, nth},
    series::Series,
};
use url::Url;

fn read_excel(bytes: Bytes) -> Result<DataFrame, Box<dyn Error>> {
    let cursor = Cursor::new(bytes);
    let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)?;

    let (_, range) = workbook.worksheets().remove(0);
    let (height, width) = range.get_size();

    let seriess = (0..width)
        .map(|col| {
            let name = range.get((0, col)).unwrap().to_string();
            let data: Vec<_> = (1..height)
                .map(|row| range.get((row, col)).unwrap().to_string())
                .collect();
            let series = Series::new(name.into(), data);
            series.into_column()
        })
        .collect();
    let df = DataFrame::new(seriess)?;
    Ok(df)
}

pub async fn stock_szse_summary(date: &str) -> Result<DataFrame, Box<dyn Error>> {
    let url = Url::parse_with_params(
        "http://www.szse.cn/api/report/ShowReport",
        [
            ("SHOWTYPE", "xlsx"),
            ("CATALOGID", "1803_sczm"),
            ("TABKEY", "tab1"),
            (
                "txtQueryDate",
                &[&date[..4], &date[4..6], &date[6..]].join("-"),
            ),
            ("random", "0.39339437497296137"),
        ],
    )?;

    let bytes = reqwest::get(url).await?.bytes().await?;
    let df = read_excel(bytes)?
        .lazy()
        .select([
            nth(0).alias("证券类别").str().strip_chars(lit(" ")),
            nth(1).alias("数量").cast(DataType::Int64),
            nth(2)
                .alias("成交金额")
                .str()
                .replace_all(lit(","), lit(""), true)
                .cast(DataType::Float64),
            nth(3)
                .alias("总市值")
                .str()
                .replace_all(lit(","), lit(""), true)
                .cast(DataType::Float64),
            nth(4)
                .alias("流通市值")
                .str()
                .replace_all(lit(","), lit(""), true)
                .cast(DataType::Float64),
        ])
        .collect()?;
    Ok(df)
}

pub async fn stock_szse_area_summary(date: &str) -> Result<DataFrame, Box<dyn Error>> {
    let url = Url::parse_with_params(
        "http://www.szse.cn/api/report/ShowReport",
        [
            ("SHOWTYPE", "xlsx"),
            ("CATALOGID", "1803_sczm"),
            ("TABKEY", "tab2"),
            ("DATETIME", &[&date[..4], &date[4..6]].join("-")),
            ("random", "0.39339437497296137"),
        ],
    )?;

    let bytes = reqwest::get(url).await?.bytes().await?;
    let df = read_excel(bytes)?
        .lazy()
        .select([
            nth(0).alias("序号").cast(DataType::Int64),
            nth(1).alias("地区"),
            nth(2)
                .alias("总交易额")
                .str()
                .replace_all(lit(","), lit(""), true)
                .cast(DataType::Float64),
            nth(3)
                .alias("占市场")
                .str()
                .replace_all(lit(","), lit(""), true)
                .cast(DataType::Float64),
            nth(4)
                .alias("股票交易额")
                .str()
                .replace_all(lit(","), lit(""), true)
                .cast(DataType::Float64),
            nth(5)
                .alias("基金交易额")
                .str()
                .replace_all(lit(","), lit(""), true)
                .cast(DataType::Float64),
            nth(6)
                .alias("债券交易额")
                .str()
                .replace_all(lit(","), lit(""), true)
                .cast(DataType::Float64),
        ])
        .collect()?;
    Ok(df)
}

// TODO: stock_szse_sector_summary, need bs4
