//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpMaJsCode2SessionResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpMaJsCode2SessionResult {
    #[serde(rename = "session_key", default)]
    pub session_key: String,
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
}

impl WxCpMaJsCode2SessionResult {
    /// 构建登录凭证校验结果。
    pub fn new(
        session_key: Option<String>,
        user_id: Option<String>,
        corp_id: Option<String>,
    ) -> Self {
        Self {
            session_key: session_key.unwrap_or_default(),
            user_id: user_id.unwrap_or_default(),
            corp_id: corp_id.unwrap_or_default(),
        }
    }
}

impl WxCpMaJsCode2SessionResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpMaJsCode2SessionResult 解析失败: {e}"))
    }
}
