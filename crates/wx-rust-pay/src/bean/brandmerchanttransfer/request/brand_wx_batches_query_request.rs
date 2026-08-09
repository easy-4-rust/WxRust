//! 对应 Java `com.github.binarywang.wxpay.bean.brandmerchanttransfer.request.BrandWxBatchesQueryRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandWxBatchesQueryRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batch_no")]
    pub batch_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "need_query_detail"
    )]
    pub need_query_detail: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "detail_state"
    )]
    pub detail_state: Option<String>,
}
