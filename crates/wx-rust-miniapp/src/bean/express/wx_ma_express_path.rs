//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.WxMaExpressPath.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressPath {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "path_item_num", default)]
    pub path_item_num: i32,
    #[serde(rename = "path_item_list", default)]
    pub path_item_list: Vec<PathItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PathItem {
    #[serde(rename = "action_time", default)]
    pub action_time: i64,
    #[serde(rename = "action_type", default)]
    pub action_type: i32,
    #[serde(rename = "action_msg", default)]
    pub action_msg: String,
}

impl WxMaExpressPath {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaExpressPath 解析失败: {e}"))
    }
}
