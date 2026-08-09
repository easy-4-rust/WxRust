//! 对应 Java `cn.binarywang.wx.miniapp.bean.qrcode.WxMaQrcodeJumpRule.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaQrcodeJumpRule {
    #[serde(rename = "prefix", default)]
    pub prefix: String,
    #[serde(rename = "permit_sub_rule", default)]
    pub permit_sub_rule: bool,
    #[serde(rename = "open_version", default)]
    pub open_version: i32,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "debug_wxa_info", default)]
    pub debug_wxa_info: Vec<WxMaQrcodeJumpWxaItem>,
    #[serde(rename = "is_expire", default)]
    pub is_expire: bool,
}
