//! 东方财富接口。

macro_rules! gen_mod_builder {
    ($name:ident) => {
        pub mod $name;
        pub fn $name() -> $name::ArgsBuilder {
            $name::Args::builder()
        }
    };
}

gen_mod_builder!(basic_org_info);

pub mod center_gridlist;

// TODO: 发行相关 (RPT_PCF10_ORG_ISSUEINFO)
// TODO: 参股控股 (RPT_F10_PUBLIC_OP_HOLDINGORG)

#[cfg(feature = "cli")]
pub mod cli;
