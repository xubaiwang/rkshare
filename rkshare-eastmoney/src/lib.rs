//! 东方财富接口。

#[crabtime::function]
fn gen_mod_builder(pattern!($name:ident, $arg:ident): _) {
    let name = stringify!($name);
    let arg = stringify!($arg);

    crabtime::output! {
        pub mod {{name}};
        pub fn {{name}}() -> {{name}}::{{arg}}Builder {
            {{name}}::{{arg}}::builder()
        }
    }
}

gen_mod_builder!(basic_org_info, Args);

// TODO: 发行相关 (RPT_PCF10_ORG_ISSUEINFO)
// TODO: 参股控股 (RPT_F10_PUBLIC_OP_HOLDINGORG)

#[cfg(feature = "cli")]
pub mod cli;
