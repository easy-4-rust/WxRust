//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodGetCdnUsageRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodGetCdnUsageRequest {
    #[serde(rename = "data_interval", default)]
    pub data_interval: i32,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
}

impl WxMaVodGetCdnUsageRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaVodGetCdnUsageRequest 序列化失败: {e}"))
    }
}
