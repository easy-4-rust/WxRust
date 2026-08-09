//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMiniOrderAfterSaleDetail.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMiniOrderAfterSaleDetail {
    #[serde(rename = "aftersale_order_list", default)]
    pub aftersale_order_list: Vec<AfterSaleOrder>,
    #[serde(rename = "on_aftersale_order_cnt", default)]
    pub on_aftersale_order_cnt: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleOrder {
    #[serde(rename = "aftersale_order_id", default)]
    pub aftersale_order_id: i64,
    #[serde(rename = "status", default)]
    pub status: i32,
}
