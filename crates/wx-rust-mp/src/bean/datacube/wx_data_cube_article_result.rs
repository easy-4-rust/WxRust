//! 对应 Java `bean.datacube.WxDataCubeArticleResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDataCubeArticleResult {
    #[serde(rename = "ref_hour", default)]
    pub ref_hour: i32,
    #[serde(rename = "msgid", default)]
    pub msg_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "int_page_read_user", default)]
    pub int_page_read_user: i32,
    #[serde(rename = "int_page_read_count", default)]
    pub int_page_read_count: i32,
    #[serde(rename = "ori_page_read_user", default)]
    pub ori_page_read_user: i32,
    #[serde(rename = "ori_page_read_count", default)]
    pub ori_page_read_count: i32,
    #[serde(rename = "share_scene", default)]
    pub share_scene: i32,
    #[serde(rename = "share_user", default)]
    pub share_user: i32,
    #[serde(rename = "share_count", default)]
    pub share_count: i32,
    #[serde(rename = "add_to_fav_user", default)]
    pub add_to_fav_user: i32,
    #[serde(rename = "add_to_fav_count", default)]
    pub add_to_fav_count: i32,
    #[serde(rename = "user_source", default)]
    pub user_source: i32,
}

impl WxDataCubeArticleResult {
    /// 从 JSON 构建列表（对应 Java `fromJson`：取 `list` 数组）。
    pub fn from_json_list(json: &str) -> Result<Vec<Self>, String> {
        let value: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| format!("WxDataCubeArticleResult 列表解析失败: {e}"))?;
        let list = value
            .get("list")
            .ok_or_else(|| "缺少 list 字段".to_string())?;
        serde_json::from_value(list.clone())
            .map_err(|e| format!("WxDataCubeArticleResult 列表解析失败: {e}"))
    }
}
