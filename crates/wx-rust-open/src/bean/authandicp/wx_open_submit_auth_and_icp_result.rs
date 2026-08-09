//! 对应 Java `me.chanjar.weixin.open.bean.authandicp.WxOpenSubmitAuthAndIcpResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenSubmitAuthAndIcpResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "hints", default)]
    pub hints: Vec<Hint>,
    #[serde(rename = "procedure_id", default)]
    pub procedure_id: String,
    #[serde(rename = "pay_url", default)]
    pub pay_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Hint {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "err_field", default)]
    pub err_field: String,
}
