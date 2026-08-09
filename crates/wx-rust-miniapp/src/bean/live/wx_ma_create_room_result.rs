//! 对应 Java `cn.binarywang.wx.miniapp.bean.live.WxMaCreateRoomResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCreateRoomResult {
    #[serde(rename = "qrcode_url", default)]
    pub qrcode_url: String,
    #[serde(rename = "roomId", default)]
    pub room_id: i32,
}
