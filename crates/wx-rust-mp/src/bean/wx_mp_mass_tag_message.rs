//! 对应 Java `bean.WxMpMassTagMessage`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMassTagMessage {
    #[serde(rename = "tagId", default)]
    pub tag_id: i64,
    #[serde(rename = "msgType", default)]
    pub msg_type: String,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "mediaId", default)]
    pub media_id: String,
    #[serde(rename = "mediaIds", default)]
    pub media_ids: Vec<String>,
    #[serde(rename = "isSendAll", default)]
    pub is_send_all: bool,
    #[serde(rename = "sendIgnoreReprint", default)]
    pub send_ignore_reprint: bool,
    #[serde(rename = "clientMsgId", default)]
    pub client_msg_id: String,
}
