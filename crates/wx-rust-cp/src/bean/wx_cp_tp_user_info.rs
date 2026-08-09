//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpUserInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpUserInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "user_ticket", default)]
    pub user_ticket: String,
    #[serde(rename = "expires_in", default)]
    pub expires_in: String,
    #[serde(rename = "open_userid", default)]
    pub open_user_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
}

impl WxCpTpUserInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpUserInfo 解析失败: {e}"))
    }
}
