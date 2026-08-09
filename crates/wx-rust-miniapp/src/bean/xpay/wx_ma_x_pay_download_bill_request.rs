//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayDownloadBillRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayDownloadBillRequest {
    #[serde(rename = "begin_ds", default)]
    pub begin_ds: i32,
    #[serde(rename = "end_ds", default)]
    pub end_ds: i32,
}

impl WxMaXPayDownloadBillRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayDownloadBillRequest 序列化失败: {e}"))
    }
}
