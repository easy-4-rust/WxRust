//! 对应 Java `com.binarywang.wxjava.store.bean.vip.VipOpenIdParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 VipOpenIdParam；对应 Java com.binarywang.wxjava.store.bean.vip.VipOpenIdParam.java。
pub struct VipOpenIdParam {
    #[serde(rename = "openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
}
