use arrow::{array::RecordBatch, datatypes::FieldRef};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_arrow::schema::{SchemaLike, TracingOptions};
use url::Url;

use crate::configured_client;

const XQ_A_TOKEN: &str = "78591293b0e2907176f7f507563a6b9ac189af0b";

pub async fn raw(symbol: &str) -> anyhow::Result<bytes::Bytes> {
    let url = Url::parse_with_params(
        "https://stock.xueqiu.com/v5/stock/f10/cn/company.json",
        [("symbol", symbol)],
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

    Ok(raw)
}

#[derive(Deserialize)]
struct Message {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    company: Company,
}

#[derive(Deserialize, Serialize)]
struct Company {
    #[serde(rename(serialize = "公司名称"))]
    org_name_cn: String,
    #[serde(rename(serialize = "曾用名"))]
    pre_name_cn: String,
    #[serde(rename(serialize = "实际控制人"))]
    actual_controller: String,
    #[serde(rename(serialize = "所有制性质名称"))]
    classi_name: String,
    #[serde(rename(serialize = "主营业务"))]
    main_operation_business: String,
    #[serde(rename(serialize = "公司简介"))]
    org_cn_introduction: String,
    #[serde(rename(serialize = "董事长"))]
    chairman: String,
    #[serde(rename(serialize = "法人代表"))]
    legal_representative: String,
    #[serde(rename(serialize = "总经理"))]
    general_manager: String,
    #[serde(rename(serialize = "董秘"))]
    secretary: String,
    #[serde(rename(serialize = "成立日期"))]
    established_date: u64,
    // NOTE: 似乎不同 currency 会不同
    #[serde(rename(serialize = "注册资本"))]
    reg_asset: f64,
    #[serde(rename(serialize = "注册货币"))]
    currency: String,
    #[serde(rename(serialize = "员工人数"))]
    staff_num: u64,
    #[serde(rename(serialize = "管理层人数"))]
    executives_nums: u64,
    #[serde(rename(serialize = "上市日期"))]
    listed_date: u64,
    #[serde(rename(serialize = "发行量"))]
    actual_issue_vol: f64,
    #[serde(rename(serialize = "发行价格"))]
    issue_price: f64,
    #[serde(rename(serialize = "募集资金"))]
    actual_rc_net_amt: f64,
    #[serde(rename(serialize = "发行市盈率"))]
    pe_after_issuing: f64,
    #[serde(rename(serialize = "网上中签率"))]
    online_success_rate_of_issue: f64,
    #[serde(rename(serialize = "联系电话"))]
    telephone: String,
    #[serde(rename(serialize = "邮政编码"))]
    postcode: String,
    #[serde(rename(serialize = "传真"))]
    fax: String,
    #[serde(rename(serialize = "电子邮箱"))]
    email: String,
    #[serde(rename(serialize = "公司网址"))]
    org_website: String,
    #[serde(rename(serialize = "注册地址"))]
    reg_address_cn: String,
    #[serde(rename(serialize = "办公地址"))]
    office_address_cn: String,
}

pub async fn arrow(symbol: &str) -> anyhow::Result<RecordBatch> {
    let raw = raw(symbol).await?;
    let item = serde_json::from_slice::<Message>(&raw)?.data.company;
    let fiedls = Vec::<FieldRef>::from_samples(&[&item], TracingOptions::default())?;
    let batch = serde_arrow::to_record_batch(&fiedls, &[item])?;
    Ok(batch)
}
