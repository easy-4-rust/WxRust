//! 对应 Java `cn.binarywang.wx.miniapp.bean.customservice.WxMaCustomserviceResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCustomserviceResult {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "entityName", default)]
    pub entity_name: String,
    #[serde(rename = "corpid", default)]
    pub corpid: String,
    #[serde(rename = "bindTime", default)]
    pub bind_time: i64,
}

impl WxMaCustomserviceResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaCustomserviceResult 解析失败: {e}"))
    }
}
