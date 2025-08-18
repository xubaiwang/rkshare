/// 处理数据到页面名称的映射。
/// 避免 serde verbosity.
// TODO: 展示 verbosity 作为文档用例
#[macro_export]
macro_rules! mapping {
    (
        // 结构体名称
        $name:ident, $(
            // 原始字段名称
            $field_name:ident =>
            // 显示名称（保持与网页一致）
            $serialize_name:literal:
            // 字段类型
            $field_type:tt
            // 解析函数
            $(
                = $deserialize:literal
            )?
            // TODO: 转换函数 deserialize_with
        ),* $(,)?
    ) => {

        #[allow(non_snake_case)]
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub struct $name<Extend = ()> {
            $(
                #[doc = $serialize_name]
                #[serde(rename(serialize = $serialize_name))]
                $(#[serde(deserialize_with = $deserialize)])?
                pub $field_name: Option<$field_type>,
            )*
            #[serde(flatten)]
            extend: Extend,
        }

        impl<Extend> rkshare_shared::FieldsInfo for $name<Extend>
        where
            Extend: rkshare_shared::FieldsInfo
        {
            fn fields() -> Vec<arrow::datatypes::Field> {
                let mut fields = vec![
                    $(
                        arrow::datatypes::Field::new(
                            $serialize_name,
                            $crate::to_arrow_datatype!($field_type),
                            true
                        ),
                    )*
                ];
                let extends_fields = Extend::fields();
                fields.extend(extends_fields);
                fields
            }
        }
    };
}

/// 将 Rust 类型转化为 Arrow 数据类型。
#[macro_export]
macro_rules! to_arrow_datatype {
    (String) => {
        arrow::datatypes::DataType::Utf8
    };
    (f64) => {
        arrow::datatypes::DataType::Float64
    };
    (u64) => {
        arrow::datatypes::DataType::UInt64
    };
    (i64) => {
        arrow::datatypes::DataType::Int64
    };
}
