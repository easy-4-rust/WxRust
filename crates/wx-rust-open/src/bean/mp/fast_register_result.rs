//! 对应 Java `me.chanjar.weixin.open.bean.mp.FastRegisterResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FastRegisterResult {
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "authorization_code", default)]
    pub authorization_code: String,
    #[serde(rename = "is_wx_verify_succ", default)]
    pub is_wx_verify_succ: bool,
}

impl FastRegisterResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("FastRegisterResult 解析失败: {e}"))
    }
}
