use std::sync::Arc;

use anyhow::Context;
use arrow::{array::RecordBatch, datatypes::Schema, json::ReaderBuilder};
use bon::Builder;
use rkshare_shared::{
    EmptyRaw, FieldsInfo,
    data::{Data, Fetch, HasTypeHint, TypeHint, TypedBytes},
    mapping,
};
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use crate::utils::{CommonMessage, configured_client, try_from_str};

pub async fn raw() -> anyhow::Result<TypedBytes> {
    let url = Url::parse_with_params(
        "http://query.sse.com.cn/commonQuery.do",
        &[
            ("sqlId", "COMMON_SSE_SJ_GPSJ_GPSJZM_TJSJ_L"),
            ("PRODUCT_NAME", "股票,主板,科创板"),
            ("type", "inParams"),
        ],
    )?;

    let bytes = configured_client()?.get(url).send().await?.bytes().await?;
    Ok((bytes, TypeHint::Json).into())
}

mapping! { Item,
    TRADE_DATE => "数据日期": String,
    PRODUCT_NAME => "证券类别": String,
    LIST_COM_NUM => "上市公司/家": u64 = "try_from_str",
    SECURITY_NUM => "上市股票/只": u64 = "try_from_str",
    TOTAL_ISSUE_VOL => "总股本/亿股": f64 = "try_from_str",
    NEGO_ISSUE_VOL => "流通股本/亿股": f64 = "try_from_str",
    TOTAL_VALUE => "总市值/亿元": f64 = "try_from_str",
    NEGO_VALUE => "流通市值/亿元": f64 = "try_from_str",
    AVG_PE_RATIO => "平均市盈率/倍": f64 = "try_from_str",
}

pub async fn arrow<Extend>() -> anyhow::Result<RecordBatch>
where
    Extend: DeserializeOwned + Serialize + FieldsInfo,
{
    let raw = raw().await?;
    let items = serde_json::from_slice::<CommonMessage<Vec<Item<Extend>>>>(&raw)?.result;

    let mut decoder =
        ReaderBuilder::new(Arc::new(Schema::new(Item::<Extend>::fields()))).build_decoder()?;
    decoder.serialize(&items)?;
    Ok(decoder
        .flush()?
        .context(anyhow::anyhow!("no buffered data"))?)
}

#[derive(Builder, Debug, Clone)]
#[cfg_attr(
    feature = "cli",
    derive(argh::FromArgs),
    argh(subcommand, name = "summary")
)]
/// 市场总貌
pub struct Args<#[cfg(not(feature = "cli"))] Extra = ()> {
    #[cfg_attr(feature = "cli", argh(subcommand))]
    raw: Option<EmptyRaw>,

    #[cfg(not(feature = "cli"))]
    #[builder(skip)]
    #[cfg_attr(feature = "cli", arg(skip))]
    _extra: PhantomData<Extra>,
}

#[cfg(not(feature = "cli"))]
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

#[cfg(not(feature = "cli"))]
impl<Extend> Fetch for Args<Extend>
where
    Extend: DeserializeOwned + Serialize + Send + FieldsInfo,
{
    async fn fetch(self) -> anyhow::Result<Data> {
        Ok(match &self.raw {
            None => self::arrow::<Extend>().await?.into(),
            Some(_) => self::raw().await?.into(),
        })
    }
}

#[cfg(feature = "cli")]
impl Fetch for Args {
    async fn fetch(self) -> anyhow::Result<Data> {
        Ok(match &self.raw {
            None => self::arrow::<()>().await?.into(),
            Some(_) => self::raw().await?.into(),
        })
    }
}

#[cfg(not(feature = "cli"))]
impl<E> HasTypeHint for Args<E> {
    fn type_hint(&self) -> Option<TypeHint> {
        self.raw.as_ref().map(|_| TypeHint::Json)
    }
}

#[cfg(feature = "cli")]
impl HasTypeHint for Args {
    fn type_hint(&self) -> Option<TypeHint> {
        self.raw.as_ref().map(|_| TypeHint::Json)
    }
}
