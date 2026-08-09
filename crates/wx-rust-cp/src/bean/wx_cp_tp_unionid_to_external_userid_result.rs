//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpUnionidToExternalUseridResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpUnionidToExternalUseridResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "external_userid", default)]
    pub external_userid: String,
    #[serde(rename = "pending_id", default)]
    pub pending_id: String,
}

impl WxCpTpUnionidToExternalUseridResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpTpUnionidToExternalUseridResult 解析失败: {e}"))
    }
}
