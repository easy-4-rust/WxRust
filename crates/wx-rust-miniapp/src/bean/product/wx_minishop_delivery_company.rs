//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopDeliveryCompany.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopDeliveryCompany {
    #[serde(rename = "deliveryId", default)]
    pub delivery_id: String,
    #[serde(rename = "deliveryName", default)]
    pub delivery_name: String,
}
