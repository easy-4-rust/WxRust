//! 对应 Java `cn.binarywang.wx.miniapp.bean.intractiy.WxMaStoreBalance.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaStoreBalance {
    #[serde(rename = "wxStoreId", default)]
    pub wx_store_id: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "allBalance", default)]
    pub all_balance: i32,
    #[serde(rename = "balanceDetail", default)]
    pub balance_detail: Vec<BalanceDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BalanceDetail {
    #[serde(rename = "balance", default)]
    pub balance: i32,
    #[serde(rename = "serviceTransId", default)]
    pub service_trans_id: String,
    #[serde(rename = "serviceTransName", default)]
    pub service_trans_name: String,
    #[serde(rename = "orderList", default)]
    pub order_list: Vec<OrderDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderDetail {
    #[serde(rename = "payorderId", default)]
    pub payorder_id: String,
    #[serde(rename = "chargeAmt", default)]
    pub charge_amt: i32,
    #[serde(rename = "unusedAmt", default)]
    pub unused_amt: i32,
    #[serde(rename = "beginTime", default)]
    pub begin_time: i64,
    #[serde(rename = "endTime", default)]
    pub end_time: i64,
}
