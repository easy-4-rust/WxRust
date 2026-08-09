//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.base.WxMaDeliveryBaseResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::delivery::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaDeliveryBaseResponse {
    #[serde(rename = "resultcode", default)]
    pub result_code: i32,
    #[serde(rename = "resultmsg", default)]
    pub result_msg: String,
}

impl WxMaDeliveryBaseResponse {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaDeliveryBaseResponse 解析失败: {e}"))
    }
}
