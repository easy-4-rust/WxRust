//! 对应 Java `bean.guide.WxMpGuideMassedBuyerInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideMassedBuyerInfo {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "send_status", default)]
    pub send_status: i32,
}
