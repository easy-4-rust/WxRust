//! 对应 Java `me.chanjar.weixin.open.bean.shoppingOrders.PayerBean.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PayerBean {
    #[serde(rename = "openid", default)]
    pub openid: String,
}
