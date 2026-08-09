//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopSharerLiveSummaryListResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopSharerLiveSummaryListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "lives", default)]
    pub lives: Vec<LiveSummaryItem>,
    #[serde(rename = "total_num", default)]
    pub total_num: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveSummaryItem {
    #[serde(rename = "live_export_id", default)]
    pub live_export_id: String,
    #[serde(rename = "live_nickname", default)]
    pub live_nickname: String,
    #[serde(rename = "live_start_time", default)]
    pub live_start_time: i64,
    #[serde(rename = "live_end_time", default)]
    pub live_end_time: i64,
    #[serde(rename = "live_status", default)]
    pub live_status: i64,
    #[serde(rename = "gmv", default)]
    pub gmv: i64,
    #[serde(rename = "order_cnt", default)]
    pub order_cnt: i64,
    #[serde(rename = "user_cnt", default)]
    pub user_cnt: i64,
}
