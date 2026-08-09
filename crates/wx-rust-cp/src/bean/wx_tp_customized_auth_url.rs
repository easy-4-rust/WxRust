//! 对应 Java `me.chanjar.weixin.cp.bean.WxTpCustomizedAuthUrl.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxTpCustomizedAuthUrl {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "qrcode_url", default)]
    pub qr_code_url: String,
    #[serde(rename = "expires_in", default)]
    pub expires_in: i32,
}

impl WxTpCustomizedAuthUrl {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxTpCustomizedAuthUrl 解析失败: {e}"))
    }
}

impl WxTpCustomizedAuthUrl {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxTpCustomizedAuthUrl 序列化失败: {e}"))
    }
}
