//! 对应 Java `me.chanjar.weixin.open.bean.minishop.goods.WxMinishopDeliveryCompany.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopDeliveryCompany {
    #[serde(rename = "deliveryId", default)]
    pub delivery_id: String,
    #[serde(rename = "deliveryName", default)]
    pub delivery_name: String,
}
