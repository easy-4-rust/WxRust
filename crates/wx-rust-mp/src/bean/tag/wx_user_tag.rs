//! 对应 Java `bean.tag.WxUserTag`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxUserTag {
    #[serde(rename = "id", default)]
    pub id: i64,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "count", default)]
    pub count: i32,
}

impl WxUserTag {
    /// 从 JSON 构建（对应 Java `fromJson`：取 `tag` 子对象）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("用户标签解析失败: {e}"))?;
        let tag = value
            .get("tag")
            .ok_or_else(|| "缺少 tag 字段".to_string())?;
        serde_json::from_value(tag.clone()).map_err(|e| format!("用户标签解析失败: {e}"))
    }

    /// 从 JSON 构建标签列表（对应 Java `listFromJson`：取 `tags` 数组）。
    pub fn list_from_json(json: &str) -> Result<Vec<Self>, String> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("用户标签列表解析失败: {e}"))?;
        let tags = value
            .get("tags")
            .ok_or_else(|| "缺少 tags 字段".to_string())?;
        serde_json::from_value(tags.clone()).map_err(|e| format!("用户标签列表解析失败: {e}"))
    }
}
