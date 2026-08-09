//! 对应 Java `bean.kefu.result.WxMpKfSessionWaitCaseList`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::kefu::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpKfSessionWaitCaseList {
    #[serde(rename = "count", default)]
    pub count: i64,
    #[serde(rename = "waitcaselist", default)]
    pub kf_session_wait_case_list: Vec<WxMpKfSession>,
}

impl WxMpKfSessionWaitCaseList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpKfSessionWaitCaseList 解析失败: {e}"))
    }
}
