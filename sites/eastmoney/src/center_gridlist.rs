/// 核心实现。
pub mod fetch {
    use std::sync::Arc;

    use anyhow::{Context, Result};
    use arrow::{array::RecordBatch, datatypes::Schema};
    use arrow_json::ReaderBuilder;
    use rkshare_shared::{
        FieldsInfo,
        data::{TypeHint, TypedBytes},
        mapping,
    };
    use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
    use url::Url;

    mapping! { Item,
        f12 => "代码": String,
        f14 => "名称": String,
        f2 => "最新价": f64 = "deserialize_div_100",
        f3 => "涨跌幅": f64 = "deserialize_div_100",
        f4 => "涨跌额": f64 = "deserialize_div_100",
        f5 => "成交量": u64 = "hyphen_missing",
        f6 => "成交额": f64 = "hyphen_missing",
        f7 => "振幅": f64 = "deserialize_div_100",
        f15 => "最高": f64 = "deserialize_div_100",
        f16 => "最低": f64 = "deserialize_div_100",
        f17 => "今开": f64 = "deserialize_div_100",
        f18 => "昨收": f64 = "deserialize_div_100",
        f10 => "量比": f64 = "deserialize_div_100",
        f8 => "换手率": f64 = "deserialize_div_100",
        f9 => "市盈率(动态)": f64 = "deserialize_div_100",
        f23 => "市净率": f64 = "deserialize_div_100",
        f20 => "总市值": u64 = "hyphen_missing",
        f21 => "流通市值": u64 = "hyphen_missing",
        f24 => "60日涨跌幅": f64 = "deserialize_div_100",
        f25 => "年初至今涨跌幅": f64 = "deserialize_div_100",
        f22 => "涨速": f64 = "deserialize_div_100",
        f11 => "5分钟涨跌": f64 = "deserialize_div_100",
    }

    /// 实际数据需要除以 100.
    fn deserialize_div_100<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match i64::deserialize(deserializer) {
            Ok(value) => Ok(Some(value as f64 / 100.)),
            Err(_) => Ok(None),
        }
    }

    /// 处理横杠作为缺失值
    fn hyphen_missing<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        Ok(T::deserialize(deserializer).ok())
    }

    #[derive(Deserialize, Debug)]
    struct Message {
        data: MessageData,
    }

    #[derive(Deserialize, Debug)]
    struct MessageData {
        total: u32,
        diff: Vec<Item>,
    }

    /// # Parameters
    /// - `page_number` 页码，从 1 开始
    pub async fn raw(page_number: u32, page_size: u32) -> Result<TypedBytes> {
        let url = Url::parse_with_params(
            "https://push2.eastmoney.com/api/qt/clist/get",
            [
                ("np", "1"),
                ("fltt", "1"),
                ("invt", "2"),
                ("fs", "m:0+t:6,m:0+t:80,m:1+t:2,m:1+t:23,m:0+t:81+s:2048"),
                (
                    "fields",
                    "f12,f13,f14,f1,f2,f4,f3,f152,f5,f6,f7,f15,f18,f16,f17,f10,f8,f9,f23,f20,f21,f24,f25,f22,f11",
                ),
                ("fid", "f3"),
                ("pn", &page_number.to_string()),
                ("pz", &page_size.to_string()),
                ("po", "1"),
                ("dect", "1"),
                // `ut` user token removed here
                ("wbp2u", "|0|0|0|web"),
                ("_", "1755013626214"),
            ],
        )?;

        let bytes = reqwest::get(url).await?.bytes().await?;
        Ok((bytes, TypeHint::Json).into())
    }

    pub async fn arrow<Extend>() -> Result<RecordBatch>
    where
        Extend: DeserializeOwned + Serialize + FieldsInfo,
    {
        let mut decoder =
            ReaderBuilder::new(Arc::new(Schema::new(Item::<Extend>::fields()))).build_decoder()?;

        // first
        let first_raw = raw(1, 100).await?;
        let first_data = serde_json::from_slice::<Message>(&first_raw)?.data;
        let total = first_data.total;
        let first_items = first_data.diff;
        decoder.serialize(&first_items)?;

        // rest
        // TODO: 是否应该并发，并发策略
        for page in 2..=(total / 100 + 1) {
            let page_raw = raw(page, 100).await?;
            let page_items = serde_json::from_slice::<Message>(&page_raw)?.data.diff;
            decoder.serialize(&page_items)?;
        }

        Ok(decoder
            .flush()?
            .context(anyhow::anyhow!("no buffered data"))?)
    }
}

/// 行情中心
#[derive(Builder, Debug, Clone)]
#[cfg_attr(
    feature = "cli",
    derive(argh::FromArgs),
    argh(subcommand, name = "center_gridlist")
)]
pub struct Args {
    #[cfg_attr(feature = "cli", argh(subcommand))]
    #[builder(into)]
    raw: Option<Raw>,
}

impl HasTypeHint for Args {
    fn type_hint(&self) -> Option<TypeHint> {
        self.raw.as_ref().map(|_| TypeHint::Json)
    }
}

/// 获取原始数据
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "cli",
    derive(argh::FromArgs),
    argh(subcommand, name = "raw")
)]
pub struct Raw {
    /// 页码
    #[cfg_attr(feature = "cli", argh(option, default = "1"))]
    page: u32,
    /// 页面大小
    #[cfg_attr(feature = "cli", argh(option, default = "100"))]
    size: u32,
}

use bon::Builder;
use rkshare_shared::data::{Data, Fetch, HasTypeHint, TypeHint};

impl Fetch for Args {
    async fn fetch(self) -> anyhow::Result<Data> {
        Ok(match &self.raw {
            None => fetch::arrow::<()>().await?.into(),
            Some(Raw {
                page: page_number,
                size: page_size,
            }) => fetch::raw(*page_number, *page_size).await?.into(),
        })
    }
}
