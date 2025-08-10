//! 统一数据表示。

use std::ops::Deref;

use arrow::array::RecordBatch;
use derive_more::From;

/// 数据。
#[derive(From, Debug, Clone)]
pub enum Data {
    /// 原始字节段。
    Raw(TypedBytes),
    /// 预处理后的数据框。
    Arrow(RecordBatch),
}

// impl From<RecordBatch> f
/// 类型注释
#[derive(Debug, Clone)]
pub enum TypeHint {
    /// JSON 类型
    Json,
    // TODO: 其他类型
}

/// 带有类型注释的 `bytes::Bytes`
#[derive(Debug, Clone)]
pub struct TypedBytes {
    /// 实际内容
    pub bytes: bytes::Bytes,
    /// 类型注释
    pub hint: TypeHint,
}

impl From<(bytes::Bytes, TypeHint)> for TypedBytes {
    fn from(value: (bytes::Bytes, TypeHint)) -> Self {
        Self {
            bytes: value.0,
            hint: value.1,
        }
    }
}

impl From<TypedBytes> for bytes::Bytes {
    fn from(value: TypedBytes) -> Self {
        value.bytes
    }
}

impl Deref for TypedBytes {
    type Target = bytes::Bytes;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
