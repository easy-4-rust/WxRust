//! 对应 Java `bean.guide.WxMpGuideAcctConfig`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideAcctConfig {
    #[serde(rename = "black_keyword", default)]
    pub guide_sensitive_words: WxMpGuideSensitiveWords,
    #[serde(rename = "guide_auto_reply", default)]
    pub guide_off_line_reply: WxMpGuideOffLineReply,
}

impl WxMpGuideAcctConfig {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpGuideAcctConfig 解析失败: {e}"))
    }
}
