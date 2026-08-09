//! 对应 Java `me.chanjar.weixin.open.bean.result.WxFastMaQueryNicknameStatusResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxFastMaQueryNicknameStatusResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "audit_stat", default)]
    pub audit_stat: i32,
    #[serde(rename = "fail_reason", default)]
    pub fail_reason: String,
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "audit_time", default)]
    pub audit_time: String,
}
