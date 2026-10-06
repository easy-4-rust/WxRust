//! 对应 Java `com.binarywang.wxjava.store.bean.freight.AllConditionFreeDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::ConditionFreeDetail;

/// 微信小店 AllConditionFreeDetail 数据类型；对应 Java com.binarywang.wxjava.store.bean.freight.AllConditionFreeDetail.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllConditionFreeDetail {
    #[serde(rename = "condition_free_detail_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ConditionFreeDetail>>,
}
