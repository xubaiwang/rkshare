use std::{error::Error, io::Cursor};

use calamine::{Reader, Xlsx, open_workbook_from_rs};
use polars::{
    frame::DataFrame,
    prelude::{DataType, IntoColumn, IntoLazy, NamedFrom, lit, nth},
    series::Series,
};
use url::Url;

const BASE: &str = "http://www.szse.cn/api/report/ShowReport";

pub async fn stock_szse_summary(date: &str) -> Result<DataFrame, Box<dyn Error>> {
    let url = Url::parse_with_params(
        BASE,
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
    let df = DataFrame::new(seriess)
        .unwrap()
        .lazy()
        .select([
            nth(0).alias("证券类别").str().strip_chars(lit(" ")),
            nth(1).alias("数量").cast(DataType::UInt32),
            nth(2)
                .alias("成交金额")
                .str()
                .replace_all(lit(","), lit(""), true)
                .cast(DataType::Float32),
            nth(3)
                .alias("总市值")
                .str()
                .replace_all(lit(","), lit(""), true)
                .cast(DataType::Float32),
            nth(4)
                .alias("流通市值")
                .str()
                .replace_all(lit(","), lit(""), true)
                .cast(DataType::Float32),
        ])
        .collect()
        .unwrap();
    Ok(df)
}
