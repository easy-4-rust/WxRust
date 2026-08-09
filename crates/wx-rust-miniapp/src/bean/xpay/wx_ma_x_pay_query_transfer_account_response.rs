//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayQueryTransferAccountResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayQueryTransferAccountResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "acct_list", default)]
    pub acct_list: Vec<AcctList>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AcctList {
    #[serde(rename = "transfer_account_name", default)]
    pub transfer_account_name: String,
    #[serde(rename = "transfer_account_uid", default)]
    pub transfer_account_uid: i64,
    #[serde(rename = "transfer_account_agency_id", default)]
    pub transfer_account_agency_id: i64,
    #[serde(rename = "transfer_account_agency_name", default)]
    pub transfer_account_agency_name: String,
    #[serde(rename = "state", default)]
    pub state: i32,
    #[serde(rename = "bind_result", default)]
    pub bind_result: i32,
    #[serde(rename = "error_msg", default)]
    pub error_msg: String,
}

impl WxMaXPayQueryTransferAccountResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayQueryTransferAccountResponse 序列化失败: {e}"))
    }
}
