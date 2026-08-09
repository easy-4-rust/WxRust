//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpPreauthCode.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName
//! 覆盖保留）。Wave 5 C5 修正：补齐 Java 的 `pre_auth_code`/`expires_in`
//! 字段（生成脚本遗漏）。

#[allow(unused_imports)]
use super::*;

/// 预授权码返回（对应 Java `WxCpTpPreauthCode`，继承 `WxCpBaseResp`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpPreauthCode {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    /// 预授权码（对应 Java `preAuthCode`）。
    #[serde(rename = "pre_auth_code", default)]
    pub pre_auth_code: String,
    /// 有效期（秒，对应 Java `expiresIn`）。
    #[serde(rename = "expires_in", default)]
    pub expires_in: i64,
}

impl WxCpTpPreauthCode {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpPreauthCode 解析失败: {e}"))
    }
}
