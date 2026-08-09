//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpMessageSendResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpMessageSendResult {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "invaliduser", default)]
    pub invalid_user: String,
    #[serde(rename = "invalidparty", default)]
    pub invalid_party: String,
    #[serde(rename = "invalidtag", default)]
    pub invalid_tag: String,
    #[serde(rename = "unlicenseduser", default)]
    pub unlicensed_user: String,
    #[serde(rename = "msgid", default)]
    pub msg_id: String,
    #[serde(rename = "response_code", default)]
    pub response_code: String,
}

impl WxCpMessageSendResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpMessageSendResult 解析失败: {e}"))
    }
}
