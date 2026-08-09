//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpContactWayResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpContactWayResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "config_id", default)]
    pub config_id: String,
    #[serde(rename = "qr_code", default)]
    pub qr_code: String,
}

impl WxCpContactWayResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpContactWayResult 解析失败: {e}"))
    }
}
