//! 对应 Java `cn.binarywang.wx.miniapp.bean.intractiy.WxMaAddOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaAddOrderRequest {
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "userName", default)]
    pub user_name: String,
    #[serde(rename = "userPhone", default)]
    pub user_phone: String,
    #[serde(rename = "userLng", default)]
    pub user_lng: f64,
    #[serde(rename = "userLat", default)]
    pub user_lat: f64,
    #[serde(rename = "userAddress", default)]
    pub user_address: String,
    #[serde(rename = "useSandbox", default)]
    pub use_sandbox: i32,
    #[serde(rename = "storeOrderId", default)]
    pub store_order_id: String,
    #[serde(rename = "userOpenid", default)]
    pub user_openid: String,
    #[serde(rename = "orderSeq", default)]
    pub order_seq: String,
    #[serde(rename = "verifyCodeType", default)]
    pub verify_code_type: i32,
    #[serde(rename = "orderDetailPath", default)]
    pub order_detail_path: String,
    #[serde(rename = "callbackUrl", default)]
    pub callback_url: String,
    #[serde(rename = "cargo", default)]
    pub cargo: Cargo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Cargo {
    #[serde(rename = "cargoName", default)]
    pub cargo_name: String,
    #[serde(rename = "cargoWeight", default)]
    pub cargo_weight: i32,
    #[serde(rename = "cargoType", default)]
    pub cargo_type: i32,
    #[serde(rename = "cargoNum", default)]
    pub cargo_num: i32,
    #[serde(rename = "cargoPrice", default)]
    pub cargo_price: i32,
    #[serde(rename = "itemList", default)]
    pub item_list: Vec<ItemDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ItemDetail {
    #[serde(rename = "itemName", default)]
    pub item_name: String,
    #[serde(rename = "itemPicUrl", default)]
    pub item_pic_url: String,
    #[serde(rename = "count", default)]
    pub count: i32,
}
