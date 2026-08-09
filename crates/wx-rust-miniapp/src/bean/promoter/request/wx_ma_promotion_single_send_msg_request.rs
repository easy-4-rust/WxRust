//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.request.WxMaPromotionSingleSendMsgRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionSingleSendMsgRequest {
    #[serde(rename = "msg_type", default)]
    pub msg_type: i32,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
}
