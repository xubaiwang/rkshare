use rkshare::{
    eastmoney::basic_org_info,
    utils::{Symbol, data::Fetch, pretty::pretty_print},
};
use serde::{Deserialize, Serialize};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let symbol: Symbol = "301232".try_into().unwrap();

    // raw
    let raw = basic_org_info()
        .symbol(symbol.clone())
        .raw()
        .build()
        .fetch()
        .await
        .unwrap();

    pretty_print(raw).unwrap();

    // arrow with custom field
    let batch = basic_org_info()
        .symbol(symbol.clone())
        .extra::<ExtraFields>()
        .build()
        .fetch()
        .await
        .unwrap();

    pretty_print(batch).unwrap();
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct ExtraFields {
    #[serde(rename(serialize = "组织代码"))]
    ORG_CODE: String,
}
