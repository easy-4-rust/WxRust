//! 对应 Java `bean.device.TransMsgResp`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransMsgResp {
    #[serde(rename = "ret", default)]
    pub ret: i32,
    #[serde(rename = "ret_info", default)]
    pub ret_info: String,
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
}

impl TransMsgResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("TransMsgResp 解析失败: {e}"))
    }
}
