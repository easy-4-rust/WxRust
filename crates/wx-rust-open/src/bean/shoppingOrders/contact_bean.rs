//! 对应 Java `me.chanjar.weixin.open.bean.shoppingOrders.ContactBean.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContactBean {
    #[serde(rename = "consignor_contact", default)]
    pub consignor_contact: String,
    #[serde(rename = "receiver_contact", default)]
    pub receiver_contact: String,
}
