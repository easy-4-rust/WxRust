//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopPayGetOrderResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopPayGetOrderResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "order", default)]
    pub order: OrderBean,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderBean {
    #[serde(rename = "trade_no", default)]
    pub trade_no: String,
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
    #[serde(rename = "combine_trade_no", default)]
    pub combine_trade_no: String,
    #[serde(rename = "mchid", default)]
    pub mchid: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i32,
    #[serde(rename = "update_time", default)]
    pub update_time: i32,
    #[serde(rename = "pay_time", default)]
    pub pay_time: i32,
    #[serde(rename = "expire_time", default)]
    pub expire_time: i32,
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "profit_sharing_delay", default)]
    pub profit_sharing_delay: i32,
    #[serde(rename = "profit_sharing_frozen", default)]
    pub profit_sharing_frozen: i32,
    #[serde(rename = "refund_list", default)]
    pub refund_list: Vec<RefundListBean>,
    #[serde(rename = "profit_sharing_list", default)]
    pub profit_sharing_list: Vec<ProfitSharingListBean>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefundListBean {
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i32,
    #[serde(rename = "finish_time", default)]
    pub finish_time: i32,
    #[serde(rename = "result", default)]
    pub result: String,
    #[serde(rename = "refund_id", default)]
    pub refund_id: String,
    #[serde(rename = "refund_no", default)]
    pub refund_no: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProfitSharingListBean {
    #[serde(rename = "mchid", default)]
    pub mchid: String,
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i32,
    #[serde(rename = "finish_time", default)]
    pub finish_time: i32,
    #[serde(rename = "result", default)]
    pub result: String,
    #[serde(rename = "profit_sharing_id", default)]
    pub profit_sharing_id: String,
    #[serde(rename = "profit_sharing_no", default)]
    pub profit_sharing_no: String,
}
