use std::{marker::PhantomData, sync::Arc};

use anyhow::Context;
use arrow::{array::RecordBatch, datatypes::Schema, json::ReaderBuilder};
use bon::Builder;
use rkshare_utils::{
    FieldsInfo, Raw,
    data::{Data, Fetch, HasTypeHint, TypeHint, TypedBytes},
    mapping,
};
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, DeserializeOwned},
};
use url::Url;

use crate::utils::{CommonMessage, configured_client, try_from_str};

pub async fn raw(date: &str) -> anyhow::Result<TypedBytes> {
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

    Ok((raw_bytes, TypeHint::Json).into())
}

mapping! { Item,
    TRADE_DATE => "数据日期": String,
    PRODUCT_CODE => "证券类别": String = "deserialize_product_code",
    LIST_NUM => "挂牌数": u64 = "try_from_str",
    TOTAL_VALUE => "市场总值(亿元)": f64 = "try_from_str",
    NEGO_VALUE => "流通市值(亿元)": f64 = "try_from_str",
    TRADE_AMT => "成交金额(亿元)": f64 = "try_from_str",
    TRADE_VOL => "成交量(亿股/亿份)": f64 = "try_from_str",
    AVG_PE_RATE => "平均市盈率(倍)": f64 = "try_from_str",
    TOTAL_TO_RATE => "换手率(%)": f64 = "try_from_str",
    NEGO_TO_RATE => "流通换手率(%)": f64 = "try_from_str",
}

pub async fn arrow<Extend>(date: &str) -> anyhow::Result<RecordBatch>
where
    Extend: DeserializeOwned + Serialize + FieldsInfo + Send,
{
    let raw = raw(date).await?;
    let items = serde_json::from_slice::<CommonMessage<Vec<Item<Extend>>>>(&raw)?.result;

    let mut decoder =
        ReaderBuilder::new(Arc::new(Schema::new(Item::<Extend>::fields()))).build_decoder()?;
    decoder.serialize(&items)?;
    Ok(decoder
        .flush()?
        .context(anyhow::anyhow!("no buffered data"))?)
}

pub(crate) fn deserialize_product_code<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(Some(
        match s {
            "01" => "主板A",
            "02" => "主板B",
            "03" => "科创板",
            "11" => "股票回购",
            "07" => "股票",
            _ => return Err(de::Error::custom(format!("product code {s} is unknown"))),
        }
        .to_string(),
    ))
}

#[derive(Builder, Debug, Clone)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
/// 市场总貌
pub struct Args<Extra = ()> {
    #[cfg_attr(feature = "cli", command(subcommand))]
    raw: Option<Raw>,

    date: String,

    #[builder(skip)]
    #[arg(skip)]
    _extra: PhantomData<Extra>,
}

use args_builder::State;

#[allow(deprecated)]
impl<F1, S: State> ArgsBuilder<F1, S> {
    pub fn extra<F2>(self) -> ArgsBuilder<F2, S>
where {
        let ArgsBuilder {
            __unsafe_private_named: unsafe_private_named,
            ..
        } = self;
        ArgsBuilder {
            __unsafe_private_phantom: PhantomData,
            __unsafe_private_named: unsafe_private_named,
        }
    }
}

impl<Extend> Fetch for Args<Extend>
where
    Extend: DeserializeOwned + Serialize + Send + FieldsInfo,
{
    async fn fetch(self) -> anyhow::Result<Data> {
        Ok(match &self.raw {
            None => self::arrow::<Extend>(&self.date).await?.into(),
            Some(_) => self::raw(&self.date).await?.into(),
        })
    }
}

impl HasTypeHint for Args {
    fn type_hint(&self) -> Option<TypeHint> {
        self.raw.as_ref().map(|_| TypeHint::Json)
    }
}
