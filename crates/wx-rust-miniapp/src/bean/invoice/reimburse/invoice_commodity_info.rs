//! 对应 Java `cn.binarywang.wx.miniapp.bean.invoice.reimburse.InvoiceCommodityInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::invoice::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceCommodityInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "num", default)]
    pub num: i32,
    #[serde(rename = "unit", default)]
    pub unit: String,
    #[serde(rename = "price", default)]
    pub price: i32,
}
