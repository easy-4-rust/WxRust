//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopSpuAudit.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopSpuAudit {
    #[serde(rename = "audit_time", default)]
    pub audit_time: String,
    #[serde(rename = "reject_reason", default)]
    pub reject_reason: String,
}
