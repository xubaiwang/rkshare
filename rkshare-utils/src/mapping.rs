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
            $field_type:ty
            // TODO: 转换函数 deserialize_with
        ),* $(,)?
    ) => {
        #[allow(non_snake_case)]
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            $(
                #[doc = $serialize_name]
                #[serde(rename(serialize = $serialize_name))]
                pub $field_name: $field_type,
            )*
        }
    };
}
