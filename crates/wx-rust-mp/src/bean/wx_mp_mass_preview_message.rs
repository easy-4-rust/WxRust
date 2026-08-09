//! 对应 Java `bean.WxMpMassPreviewMessage`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMassPreviewMessage {
    #[serde(rename = "toWxUserName", default)]
    pub to_wx_user_name: String,
    #[serde(rename = "toWxUserOpenid", default)]
    pub to_wx_user_openid: String,
    #[serde(rename = "msgType", default)]
    pub msg_type: String,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "mediaId", default)]
    pub media_id: String,
}
