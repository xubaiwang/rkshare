use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use arrow::{array::RecordBatch, datatypes::Schema, json::ReaderBuilder};
use bon::Builder;
use reqwest::header::HeaderMap;
use rkshare_shared::{
    EmptyRaw, FieldsInfo, Symbol,
    data::{Data, Fetch, HasTypeHint, TypeHint, TypedBytes},
    mapping,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use url::Url;

use crate::configured_client;

const XQ_A_TOKEN: &str = "b813a12eb0682b072cce896e4e6c985ecd8efdbd";

pub async fn raw(symbol: impl TryInto<Symbol>) -> Result<TypedBytes> {
    let symbol: Symbol = symbol
        .try_into()
        .map_err(|_| anyhow!("cannot convert to symbol"))?;
    let url = Url::parse_with_params(
        "https://stock.xueqiu.com/v5/stock/f10/cn/company.json",
        [("symbol", symbol.to_prefixed())],
    )?;

    let headers = {
        let mut headers = HeaderMap::new();
        // TODO: optional token
        headers.insert("cookie", format!("xq_a_token={XQ_A_TOKEN}").parse()?);
        headers
    };

    // TODO: timeout
    let raw = configured_client()?
        .get(url)
        .headers(headers)
        .send()
        .await?
        .bytes()
        .await?;

    Ok((raw, TypeHint::Json).into())
}

#[derive(Deserialize)]
struct Message {
    data: MessageData,
}

#[derive(Deserialize)]
struct MessageData {
    company: Company,
}

mapping! { Company,
    org_name_cn => "公司名称": String,
    pre_name_cn => "曾用名": String,
    actual_controller =>"实际控制人": String,
    classi_name => "所有制性质名称": String,
    main_operation_business => "主营业务": String,
    org_cn_introduction => "公司简介": String,
    chairman => "董事长": String,
    legal_representative => "法人代表": String,
    general_manager => "总经理": String,
    secretary => "董秘": String,
    established_date => "成立日期": u64,
    reg_asset => "注册资本": f64,
    currency => "注册货币": String,
    staff_num => "员工人数": u64,
    executives_nums => "管理层人数": u64,
    listed_date => "上市日期": u64,
    actual_issue_vol => "发行量": f64,
    issue_price => "发行价格": f64,
    actual_rc_net_amt => "募集资金": f64,
    pe_after_issuing => "发行市盈率": f64,
    online_success_rate_of_issue => "网上中签率": f64,
    telephone => "联系电话": String,
    postcode => "邮政编码": String,
    fax => "传真": String,
    email => "电子邮箱": String,
    org_website => "公司网址": String,
    reg_address_cn => "注册地址": String,
    office_address_cn => "办公地址": String,
}

pub async fn arrow<Extend>(symbol: impl TryInto<Symbol>) -> Result<RecordBatch>
where
    Extend: DeserializeOwned + Serialize + FieldsInfo + Send,
{
    let raw = raw(symbol).await?;
    let item = serde_json::from_slice::<Message>(&raw)?.data.company;
    let mut decoder =
        ReaderBuilder::new(Arc::new(Schema::new(Company::<Extend>::fields()))).build_decoder()?;
    decoder.serialize(&[item])?;

    Ok(decoder.flush()?.context(anyhow!("no buffered data"))?)
}

#[derive(Builder, Debug, Clone)]
#[cfg_attr(
    feature = "cli",
    derive(argh::FromArgs),
    argh(subcommand, name = "detail")
)]
/// 公司简介
pub struct Args<#[cfg(not(feature = "cli"))] Extra = ()> {
    #[cfg(not(feature = "cli"))]
    #[builder(skip)]
    #[cfg_attr(feature = "cli", arg(skip))]
    _extra: PhantomData<Extra>,

    #[cfg_attr(feature = "cli", argh(subcommand))]
    raw: Option<EmptyRaw>,

    /// 股票代码
    #[argh(positional)]
    symbol: Symbol,
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
    async fn fetch(self) -> Result<Data> {
        Ok(match &self.raw {
            None => self::arrow::<Extend>(self.symbol).await?.into(),
            Some(_) => self::raw(self.symbol).await?.into(),
        })
    }
}

#[cfg(feature = "cli")]
impl Fetch for Args {
    async fn fetch(self) -> Result<Data> {
        Ok(match &self.raw {
            None => self::arrow::<()>(self.symbol).await?.into(),
            Some(_) => self::raw(self.symbol).await?.into(),
        })
    }
}

impl HasTypeHint for Args {
    fn type_hint(&self) -> Option<TypeHint> {
        self.raw.as_ref().map(|_| TypeHint::Json)
    }
}
