//! 对应 Java `com.binarywang.wxjava.store.bean.vip.VipListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::VipInfo;

/// 微信小店 VipListResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.vip.VipListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VipListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_infos: Option<Vec<VipInfo>>,
    #[serde(rename = "total_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num: Option<i64>,
}
