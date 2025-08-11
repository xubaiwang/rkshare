#![warn(missing_docs)]

//! 本模块提供 rkshare 通用实用功能

use arrow::datatypes::Field;
pub use symbol::*;

mod mapping;
pub mod symbol;

#[cfg(feature = "cli")]
#[doc(hidden)]
pub mod cli;

pub mod data;

#[cfg(feature = "pretty")]
pub mod pretty;

/// 有哪些数据列。
pub trait FieldsInfo {
    /// 列出数据列。
    fn fields() -> Vec<Field>;
}

impl FieldsInfo for () {
    fn fields() -> Vec<Field> {
        vec![]
    }
}
