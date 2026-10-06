//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.AccountInfoResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AccountInfoResponse；对应 Java com.binarywang.wxjava.store.bean.ewaybill.AccountInfoResponse.java。
pub struct AccountInfoResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 账号信息列表
    #[serde(rename = "account_info_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_info_list: Option<Vec<AccountInfo>>,
}

/// 微信小店 AccountInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.ewaybill.AccountInfoResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountInfo {
    /// 快递公司 ID
    #[serde(rename = "delivery_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    /// 网点编码
    #[serde(rename = "branch_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    /// 电子面单账号
    #[serde(rename = "account_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
}
