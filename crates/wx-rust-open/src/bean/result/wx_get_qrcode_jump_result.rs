//! 对应 Java `me.chanjar.weixin.open.bean.result.WxGetQrcodeJumpResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxGetQrcodeJumpResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "rule_list", default)]
    pub rule_list: Vec<WxQrcodeJumpRule>,
    #[serde(rename = "qrcodejump_open", default)]
    pub qrcodejump_open: String,
    #[serde(rename = "qrcodejump_pub_quota", default)]
    pub qrcodejump_pub_quota: i32,
    #[serde(rename = "list_size", default)]
    pub list_size: i32,
}
