macro_rules! mod_reexport {
    ($name:ident) => {
        pub mod $name;
        pub use $name::arrow as $name;
    };
}

mod_reexport!(basic_org_info);

// TODO: 发行相关 (RPT_PCF10_ORG_ISSUEINFO)
// TODO: 参股控股 (RPT_F10_PUBLIC_OP_HOLDINGORG)
