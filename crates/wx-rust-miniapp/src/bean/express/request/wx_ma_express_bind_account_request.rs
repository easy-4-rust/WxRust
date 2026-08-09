//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.request.WxMaExpressBindAccountRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressBindAccountRequest {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "biz_id", default)]
    pub biz_id: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "password", default)]
    pub password: String,
    #[serde(rename = "remark_content", default)]
    pub remark_content: String,
}

impl WxMaExpressBindAccountRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaExpressBindAccountRequest 序列化失败: {e}"))
    }
}
