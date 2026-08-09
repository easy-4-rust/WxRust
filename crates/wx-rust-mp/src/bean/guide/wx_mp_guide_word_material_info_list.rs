//! 对应 Java `bean.guide.WxMpGuideWordMaterialInfoList`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideWordMaterialInfoList {
    #[serde(rename = "total_num", default)]
    pub total_num: i32,
    #[serde(rename = "word_list", default)]
    pub list: Vec<WxMpGuideWordMaterialInfo>,
}

impl WxMpGuideWordMaterialInfoList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxMpGuideWordMaterialInfoList 解析失败: {e}"))
    }
}
