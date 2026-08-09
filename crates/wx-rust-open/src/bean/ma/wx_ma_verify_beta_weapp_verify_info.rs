//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxMaVerifyBetaWeappVerifyInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVerifyBetaWeappVerifyInfo {
    #[serde(rename = "enterprise_name", default)]
    pub enterprise_name: String,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "code_type", default)]
    pub code_type: i32,
    #[serde(rename = "legal_persona_wechat", default)]
    pub legal_persona_wechat: String,
    #[serde(rename = "legal_persona_name", default)]
    pub legal_persona_name: String,
    #[serde(rename = "component_phone", default)]
    pub component_phone: String,
    #[serde(rename = "legal_persona_idcard", default)]
    pub legal_persona_idcard: String,
}
