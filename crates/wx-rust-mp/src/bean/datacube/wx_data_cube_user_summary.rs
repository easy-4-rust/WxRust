//! 对应 Java `bean.datacube.WxDataCubeUserSummary`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDataCubeUserSummary {
    #[serde(rename = "ref_date", default)]
    pub ref_date: String,
    #[serde(rename = "user_source", default)]
    pub user_source: i32,
    #[serde(rename = "new_user", default)]
    pub new_user: i32,
    #[serde(rename = "cancel_user", default)]
    pub cancel_user: i32,
}

impl WxDataCubeUserSummary {
    /// 从 JSON 构建列表（对应 Java `fromJson`：取 `list` 数组）。
    pub fn from_json_list(json: &str) -> Result<Vec<Self>, String> {
        let value: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| format!("WxDataCubeUserSummary 列表解析失败: {e}"))?;
        let list = value
            .get("list")
            .ok_or_else(|| "缺少 list 字段".to_string())?;
        serde_json::from_value(list.clone())
            .map_err(|e| format!("WxDataCubeUserSummary 列表解析失败: {e}"))
    }
}
