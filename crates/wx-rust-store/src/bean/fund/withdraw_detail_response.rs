//! 对应 Java `com.binarywang.wxjava.store.bean.fund.WithdrawDetailResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 WithdrawDetailResponse；对应 Java com.binarywang.wxjava.store.bean.fund.WithdrawDetailResponse.java。
pub struct WithdrawDetailResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "amount", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "update_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(rename = "reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "remark", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(rename = "bank_memo", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_memo: Option<String>,
    #[serde(rename = "bank_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(rename = "bank_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_num: Option<String>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
