//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.BindAccountResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BindAccountResponse {
    #[serde(rename = "resultcode", default)]
    pub result_code: i32,
    #[serde(rename = "resultmsg", default)]
    pub result_msg: String,
    #[serde(rename = "shop_list", default)]
    pub shop_list: Vec<Shop>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Shop {
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "shopid", default)]
    pub shop_id: String,
    #[serde(rename = "audit_result", default)]
    pub audit_result: String,
}
