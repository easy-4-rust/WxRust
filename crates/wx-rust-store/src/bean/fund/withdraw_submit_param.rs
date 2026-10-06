//! 对应 Java `com.binarywang.wxjava.store.bean.fund.WithdrawSubmitParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 WithdrawSubmitParam；对应 Java com.binarywang.wxjava.store.bean.fund.WithdrawSubmitParam.java。
pub struct WithdrawSubmitParam {
    #[serde(rename = "amount", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "remark", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(rename = "bank_memo", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_memo: Option<String>,
}
