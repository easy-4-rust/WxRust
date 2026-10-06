//! 对应 Java `com.binarywang.wxjava.store.bean.fund.FundsFlow.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::FlowRelatedInfo;

/// 微信小店 FundsFlow 数据类型；对应 Java com.binarywang.wxjava.store.bean.fund.FundsFlow.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundsFlow {
    #[serde(rename = "flow_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(rename = "funds_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds_type: Option<i32>,
    #[serde(rename = "flow_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<i32>,
    #[serde(rename = "amount", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "balance", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i32>,
    #[serde(rename = "related_info_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_infos: Option<Vec<FlowRelatedInfo>>,
    #[serde(rename = "bookkeeping_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookkeeping_time: Option<String>,
    #[serde(rename = "remark", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}
