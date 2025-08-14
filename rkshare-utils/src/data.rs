//! 统一数据表示。

use std::ops::Deref;

use arrow::array::RecordBatch;
use bytes::Bytes;
use derive_more::From;

/// 数据。
#[derive(From, Debug, Clone)]
pub enum Data {
    /// 原始字节段。
    Raw(TypedBytes),
    /// 预处理后的数据框。
    Arrow(RecordBatch),
}

impl Data {
    pub fn as_raw(&self) -> Option<&Bytes> {
        match self {
            Data::Raw(typed_bytes) => Some(typed_bytes),
            _ => None,
        }
    }

    pub fn as_arrow(&self) -> Option<&RecordBatch> {
        match self {
            Data::Arrow(record_batch) => Some(record_batch),
            _ => None,
        }
    }
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

/// 数据接口。
pub trait Fetch {
    /// 请求数据。
    // TODO: 未来应该构建自己的错误类型
    fn fetch(self) -> impl std::future::Future<Output = anyhow::Result<Data>> + Send;
}

/// 参数类型提示。
///
/// 用于命令行检测输出格式是否正确。
pub trait HasTypeHint {
    /// Option 表示结果为 `RecordBatch`.
    fn type_hint(&self) -> Option<TypeHint>;
}
