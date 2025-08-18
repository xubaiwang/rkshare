use bon::Builder;
use rkshare_shared::data::{Data, Fetch, HasTypeHint, TypeHint};

pub mod fetch {
    use std::sync::Arc;

    use anyhow::Context;
    use arrow::{array::RecordBatch, datatypes::Schema, json::ReaderBuilder};
    use rkshare_shared::{
        FieldsInfo,
        data::{TypeHint, TypedBytes},
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
}

#[derive(Builder, Debug, Clone)]
#[cfg_attr(
    feature = "cli",
    derive(argh::FromArgs),
    argh(subcommand, name = "summary")
)]
/// 市场总貌
pub struct Args {
    #[cfg_attr(feature = "cli", argh(subcommand))]
    raw: Option<rkshare_shared::EmptyRaw>,
}

impl Fetch for Args {
    async fn fetch(self) -> anyhow::Result<Data> {
        Ok(match &self.raw {
            None => fetch::arrow::<()>().await?.into(),
            Some(_) => fetch::raw().await?.into(),
        })
    }
}

impl HasTypeHint for Args {
    fn type_hint(&self) -> Option<TypeHint> {
        self.raw.as_ref().map(|_| TypeHint::Json)
    }
}
