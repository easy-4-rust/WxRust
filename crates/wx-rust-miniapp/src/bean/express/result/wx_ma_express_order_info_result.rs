//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.result.WxMaExpressOrderInfoResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressOrderInfoResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "print_html", default)]
    pub print_html: String,
    #[serde(rename = "waybill_data", default)]
    pub waybill_data: Vec<std::collections::HashMap<String, String>>,
    #[serde(rename = "order_status", default)]
    pub order_status: i32,
}

impl WxMaExpressOrderInfoResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaExpressOrderInfoResult 解析失败: {e}"))
    }
}
