//! 对应 Java `bean.card.WxMpCardResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpCardResult {
    #[serde(rename = "errorCode", default)]
    pub error_code: String,
    #[serde(rename = "errorMsg", default)]
    pub error_msg: String,
    #[serde(rename = "openId", default)]
    pub open_id: String,
    #[serde(rename = "card", default)]
    pub card: WxMpCard,
    #[serde(rename = "userCardStatus", default)]
    pub user_card_status: String,
    #[serde(rename = "canConsume", default)]
    pub can_consume: bool,
    #[serde(rename = "outStr", default)]
    pub out_str: String,
    #[serde(rename = "backgroundPicUrl", default)]
    pub background_pic_url: String,
    #[serde(rename = "unionid", default)]
    pub unionid: String,
}

impl WxMpCardResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpCardResult 解析失败: {e}"))
    }
}
