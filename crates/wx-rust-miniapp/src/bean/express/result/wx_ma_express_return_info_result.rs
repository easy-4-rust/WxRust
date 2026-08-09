//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.result.WxMaExpressReturnInfoResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressReturnInfoResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "return_id", default)]
    pub return_id: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "order_status", default)]
    pub order_status: String,
    #[serde(rename = "delivery_name", default)]
    pub delivery_name: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
}

impl WxMaExpressReturnInfoResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaExpressReturnInfoResult 解析失败: {e}"))
    }
}
