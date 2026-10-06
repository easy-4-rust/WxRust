//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.AutoValidInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AutoValidInfo；对应 Java com.binarywang.wxjava.store.bean.coupon.AutoValidInfo.java。
pub struct AutoValidInfo {
    #[serde(rename = "auto_valid_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_valid_type: Option<i32>,
}
