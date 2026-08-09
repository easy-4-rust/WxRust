//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayStartUploadGoodsRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayStartUploadGoodsRequest {
    #[serde(rename = "env", default)]
    pub env: i32,
    #[serde(rename = "upload_item", default)]
    pub upload_item: Vec<UploadItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UploadItem {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "price", default)]
    pub price: i32,
    #[serde(rename = "remark", default)]
    pub remark: String,
    #[serde(rename = "item_url", default)]
    pub item_url: String,
}

impl WxMaXPayStartUploadGoodsRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayStartUploadGoodsRequest 序列化失败: {e}"))
    }
}
