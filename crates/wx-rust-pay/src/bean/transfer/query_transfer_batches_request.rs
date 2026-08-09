//! 对应 Java `com.github.binarywang.wxpay.bean.transfer.QueryTransferBatchesRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QueryTransferBatchesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batchId")]
    pub batch_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "needQueryDetail"
    )]
    pub need_query_detail: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offset")]
    pub offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limit")]
    pub limit: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "detailStatus"
    )]
    pub detail_status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "outBatchNo"
    )]
    pub out_batch_no: Option<String>,
}
