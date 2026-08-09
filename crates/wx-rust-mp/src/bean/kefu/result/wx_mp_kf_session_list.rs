//! 对应 Java `bean.kefu.result.WxMpKfSessionList`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::kefu::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpKfSessionList {
    #[serde(rename = "sessionlist", default)]
    pub kf_session_list: Vec<WxMpKfSession>,
}

impl WxMpKfSessionList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpKfSessionList 解析失败: {e}"))
    }
}
