#![warn(missing_docs)]

//! 本模块提供 rkshare 通用实用功能

use std::marker::PhantomData;

use arrow::datatypes::Field;
use serde::{Deserialize, Serialize};
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

/// 标记参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhantomArg<Flatten>(PhantomData<Flatten>);

impl<Flatten> Default for PhantomArg<Flatten> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[cfg(feature = "cli")]
impl<Flatten> argh::FromArgValue for PhantomArg<Flatten> {
    fn from_arg_value(_value: &str) -> std::result::Result<Self, String> {
        Ok(Self::default())
    }
}
