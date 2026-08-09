//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopSharerDataSummaryResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopSharerDataSummaryResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "gmv", default)]
    pub gmv: i64,
    #[serde(rename = "order_cnt", default)]
    pub order_cnt: i64,
    #[serde(rename = "user_cnt", default)]
    pub user_cnt: i64,
}
