//! 对应 Java `com.binarywang.wxjava.store.bean.vip.VipScoreParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 VipScoreParam；对应 Java com.binarywang.wxjava.store.bean.vip.VipScoreParam.java。
pub struct VipScoreParam {
    #[serde(rename = "openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(rename = "score", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
    #[serde(rename = "remark", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(rename = "request_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
