//! 对应 Java `cn.binarywang.wx.miniapp.bean.qrcode.WxMaQrcodeJumpWxaItem.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaQrcodeJumpWxaItem {
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "path", default)]
    pub path: String,
}
