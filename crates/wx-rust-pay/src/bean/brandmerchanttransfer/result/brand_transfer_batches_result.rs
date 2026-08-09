//! 对应 Java `com.github.binarywang.wxpay.bean.brandmerchanttransfer.result.BrandTransferBatchesResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandTransferBatchesResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_batch_no"
    )]
    pub out_batch_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batch_no")]
    pub batch_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "create_time"
    )]
    pub create_time: Option<String>,
}
