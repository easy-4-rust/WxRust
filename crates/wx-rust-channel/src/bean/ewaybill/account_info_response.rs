//! 对应 Java `me.chanjar.weixin.channel.bean.ewaybill.AccountInfoResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountInfoResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 账号信息列表
    #[serde(rename = "account_info_list", default)]
    pub account_info_list: Vec<AccountInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountInfo {
    /// 快递公司 ID
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    /// 网点编码
    #[serde(rename = "branch_code", default)]
    pub branch_code: String,
    /// 电子面单账号
    #[serde(rename = "account_code", default)]
    pub account_code: String,
}
