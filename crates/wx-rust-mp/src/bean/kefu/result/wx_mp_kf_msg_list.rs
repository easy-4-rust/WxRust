//! 对应 Java `bean.kefu.result.WxMpKfMsgList`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpKfMsgList {
    #[serde(rename = "recordlist", default)]
    pub records: Vec<WxMpKfMsgRecord>,
    #[serde(rename = "number", default)]
    pub number: i32,
    #[serde(rename = "msgid", default)]
    pub msg_id: i64,
}

impl WxMpKfMsgList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpKfMsgList 解析失败: {e}"))
    }
}
