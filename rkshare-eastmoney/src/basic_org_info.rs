//! 公司概况 > 基本资料

use arrow::{array::RecordBatch, datatypes::FieldRef};
use rkshare_utils::{Symbol, mapping};
use serde::Deserialize;
use serde_arrow::schema::{SchemaLike, TracingOptions};
use url::Url;

/// Desc.
#[derive(Debug)]
#[cfg_attr(
    feature = "cli",
    derive(argh::FromArgs),
    argh(subcommand, name = "basic_org_info")
)]
pub struct Args {
    /// the symbol
    #[cfg_attr(feature = "cli", argh(positional))]
    pub symbol: Symbol,
}

/// Example: 301232.SZ
pub async fn raw(symbol: &str) -> anyhow::Result<bytes::Bytes> {
    let url = Url::parse_with_params(
        "https://datacenter.eastmoney.com/securities/api/data/v1/get",
        [
            ("reportName", "RPT_F10_BASIC_ORGINFO"),
            ("columns", "ALL"),
            ("quoteColumns", ""),
            ("filter", &format!(r#"(SECUCODE="{symbol}")"#)),
            ("pageNumber", "1"),
            ("pageSize", "1"),
            ("sortTypes", ""),
            ("sortColumns", ""),
            ("source", "HSF10"),
            ("client", "PC"),
            ("v", "08415533882391942"),
        ],
    )?;

    let bytes = reqwest::get(url).await?.bytes().await?;
    Ok(bytes)
}

pub async fn arrow(symbol: &str) -> anyhow::Result<RecordBatch> {
    let raw = raw(symbol).await?;
    let items = serde_json::from_slice::<Message<Item>>(&raw)?.result.data;
    let fields =
        Vec::<FieldRef>::from_samples(&items, TracingOptions::default().allow_null_fields(true))?;
    let batch = serde_arrow::to_record_batch(&fields, &items)?;
    Ok(batch)
}

#[derive(Debug, Deserialize)]
struct Message<T> {
    pub result: MessageResult<T>,
}

#[derive(Debug, Deserialize)]
struct MessageResult<T> {
    pub data: Vec<T>,
}

mapping! { Item,
    ORG_NAME => "公司名称": String,
    ORG_NAME_EN => "英文名称": String,
    STR_CODEA => "A股代码": Option<String>,
    STR_NAMEA => "A股简称": Option<String>,
    EXPAND_NAME_ABBR => "A股扩位简称": Option<String>,
    FORMERNAME => "曾用名": Option<String>,
    STR_CODEB => "B股代码": Option<String>,
    STR_NAMEB => "B股简称": Option<String>,
    STR_CODEH => "H股代码": Option<String>,
    STR_NAMEH => "H股简称": Option<String>,
    SECURITY_TYPE => "证券类别": String,
    EM2016 => "所属东财行业": String,
    TRADE_MARKET => "上市交易所": String,
    INDUSTRYCSRC1 => "所属证监会行业": String,
    PRESIDENT => "总经理": String,
    LEGAL_PERSON => "法人代表": String,
    SECRETARY => "董秘": String,
    CHAIRMAN => "董事长": String,
    SECPRESENT => "证券事物代表": String,
    INDEDIRECTORS => "独立董事": String,
    ORG_TEL => "联系电话": String,
    ORG_EMAIL => "电子信箱": String,
    ORG_FAX => "传真": String,
    ORG_WEB => "公司网址": String,
    ADDRESS => "办公地址": String,
    REG_ADDRESS => "注册地址": String,
    PROVINCE => "区域": String,
    ADDRESS_POSTCODE => "邮政编码": String,
    REG_CAPITAL => "注册资本(元)": f64,
    REG_NUM => "工商登记": String,
    EMP_NUM => "雇员人数": u64,
    TATOLNUMBER => "管理人员人数": u64,
    LAW_FIRM => "律师事务所": String,
    ACCOUNTFIRM_NAME => "会计师事务所": String,
    ORG_PROFILE => "公司简介": String,
    BUSINESS_SCOPE => "经营范围": String,
}
