//! 团长商品变更信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.supplier.SupplierItemInfo.java`。

use serde::{Deserialize, Serialize};

/// 团长商品变更信息（对应 Java `SupplierItemInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupplierItemInfo {
    /// 商品变更类型，1：新增商品；2：更新商品（对应 Java `eventType`）。
    #[serde(
        rename = "event_type",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub event_type: Option<i32>,
    /// 团长商品所属小店appid（对应 Java `appid`）。
    #[serde(rename = "appid", default)]
    pub appid: Option<String>,
    /// 商品id（对应 Java `productId`）。
    #[serde(rename = "product_id", default)]
    pub product_id: Option<String>,
    /// 商品版本号（对应 Java `version`）。
    #[serde(rename = "version", default)]
    pub version: Option<String>,
    /// 商品更新字段，当event_type = 2时有值。commission_ratio、service_ratio、
    /// status、active_time分别表示佣金、服务费、商品状态和合作生效时间有变更
    /// （对应 Java `updateFields`；JSON 为数组）。
    #[serde(rename = "update_fields", default)]
    pub update_fields: Option<Vec<String>>,
}
