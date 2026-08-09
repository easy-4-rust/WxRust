//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenMaGetCodePrivacyInfoResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaGetCodePrivacyInfoResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "without_auth_list", default)]
    pub without_auth_list: Vec<String>,
    #[serde(rename = "without_conf_list", default)]
    pub without_conf_list: Vec<String>,
}
