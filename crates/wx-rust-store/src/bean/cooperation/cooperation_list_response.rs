//! 对应 Java `com.binarywang.wxjava.store.bean.cooperation.CooperationListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::CooperationData;

/// 微信小店 CooperationListResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.cooperation.CooperationListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CooperationListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "data_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_list: Option<Vec<CooperationData>>,
}
