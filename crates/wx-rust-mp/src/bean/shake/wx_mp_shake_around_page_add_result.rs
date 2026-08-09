//! 对应 Java `bean.shake.WxMpShakeAroundPageAddResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpShakeAroundPageAddResult {
    #[serde(rename = "errorCode", default)]
    pub error_code: i32,
    #[serde(rename = "errorMsg", default)]
    pub error_msg: String,
    #[serde(rename = "pageId", default)]
    pub page_id: i32,
}

impl WxMpShakeAroundPageAddResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxMpShakeAroundPageAddResult 解析失败: {e}"))
    }
}
