//! 对应 Java `bean.device.WxDeviceOpenIdResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDeviceOpenIdResult {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "open_id", default)]
    pub open_ids: Vec<String>,
    #[serde(rename = "resp_msg", default)]
    pub resp_msg: RespMsg,
}

impl WxDeviceOpenIdResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxDeviceOpenIdResult 解析失败: {e}"))
    }
}
