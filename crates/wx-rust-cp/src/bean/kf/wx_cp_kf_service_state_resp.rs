//! 对应 Java `me.chanjar.weixin.cp.bean.kf.WxCpKfServiceStateResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfServiceStateResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "service_state", default)]
    pub service_state: i32,
    #[serde(rename = "servicer_userid", default)]
    pub servicer_user_id: String,
}

impl WxCpKfServiceStateResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpKfServiceStateResp 解析失败: {e}"))
    }
}
