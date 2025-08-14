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

#[cfg(feature = "cli")]
#[derive(clap::Subcommand, Clone, Debug)]
pub enum Raw<T: clap::Args = EmptyArgs> {
    Raw(T),
}

impl<T: Default + clap::Args> Default for Raw<T> {
    fn default() -> Self {
        Self::Raw(T::default())
    }
}

#[cfg(feature = "cli")]
#[derive(clap::Args, Default, Clone, Debug)]
pub struct EmptyArgs;
