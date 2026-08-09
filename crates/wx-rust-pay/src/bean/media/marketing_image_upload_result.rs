//! 对应 Java `com.github.binarywang.wxpay.bean.media.MarketingImageUploadResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarketingImageUploadResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "media_url")]
    pub media_url: Option<String>,
}

impl MarketingImageUploadResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("MarketingImageUploadResult 解析失败: {e}"))
    }
}
