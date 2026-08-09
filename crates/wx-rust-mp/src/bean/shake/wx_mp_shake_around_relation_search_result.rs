//! 对应 Java `bean.shake.WxMpShakeAroundRelationSearchResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpShakeAroundRelationSearchResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "data", default)]
    pub data: WxMpShakeAcoundRelationSearch,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpShakeAcoundRelationSearch {
    #[serde(rename = "relations", default)]
    pub relations: Vec<WxMpDeviceIdentifier>,
    #[serde(rename = "total_count", default)]
    pub total_count: i32,
}

impl WxMpShakeAroundRelationSearchResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxMpShakeAroundRelationSearchResult 解析失败: {e}"))
    }
}
