//! 该模块用于美化数据内容输出。

use arrow::util::pretty::pretty_format_batches;
use serde_json::Value;

use crate::data::{Data, TypeHint};

/// 美化数据输出。
pub fn pretty_print(data: impl Into<Data>) -> anyhow::Result<()> {
    let data = data.into();

    match data {
        Data::Arrow(record_batch) => {
            // TODO: 当期使用 arrow 自带美化，对长行支持不好，
            // 后续应该修改读取终端宽度等。
            let formatted = pretty_format_batches(&[record_batch.clone()])?;
            println!("{}", formatted);
        }
        Data::Raw(typed_bytes) => {
            match typed_bytes.hint {
                TypeHint::Json => {
                    let value: Value = serde_json::from_slice(&typed_bytes)?;
                    // TODO: 缺少颜色美化
                    let formatted = serde_json::to_string_pretty(&value)?;
                    println!("{formatted}");
                }
            }
        }
    }

    Ok(())
}
