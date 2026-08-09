//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.CancelOrderResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelOrderResponse {
    #[serde(rename = "resultcode", default)]
    pub result_code: i32,
    #[serde(rename = "resultmsg", default)]
    pub result_msg: String,
    #[serde(
        rename = "deduct_fee",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub deduct_fee: String,
    #[serde(rename = "desc", default)]
    pub desc: String,
}
