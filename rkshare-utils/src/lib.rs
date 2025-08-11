#![warn(missing_docs)]

//! 本模块提供 rkshare 通用实用功能

pub use symbol::*;

mod mapping;
pub mod symbol;

#[cfg(feature = "cli")]
#[doc(hidden)]
pub mod cli;

pub mod data;

#[cfg(feature = "pretty")]
pub mod pretty;
