//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaRunStepInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaRunStepInfo {
    #[serde(rename = "timestamp", default)]
    pub timestamp: i64,
    #[serde(rename = "step", default)]
    pub step: i32,
}

impl WxMaRunStepInfo {
    /// 从 JSON 构建列表（对应 Java `fromJson`：取 `stepInfoList` 数组）。
    pub fn from_json(json: &str) -> Result<Vec<Self>, String> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("WxMaRunStepInfo 列表解析失败: {e}"))?;
        let list = value
            .get("stepInfoList")
            .ok_or_else(|| "缺少 stepInfoList 字段".to_string())?;
        serde_json::from_value(list.clone())
            .map_err(|e| format!("WxMaRunStepInfo 列表解析失败: {e}"))
    }
}
