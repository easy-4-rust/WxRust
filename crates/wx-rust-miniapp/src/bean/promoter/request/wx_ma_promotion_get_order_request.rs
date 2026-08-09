//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.request.WxMaPromotionGetOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionGetOrderRequest {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "mch_id", default)]
    pub mch_id: String,
    #[serde(rename = "trade_no", default)]
    pub trade_no: String,
    #[serde(rename = "out_trade_no", default)]
    pub out_trade_no: String,
    #[serde(rename = "status", default)]
    pub status: i64,
    #[serde(rename = "start_id", default)]
    pub start_id: String,
    #[serde(rename = "need_unionid", default)]
    pub need_unionid: i64,
    #[serde(rename = "date", default)]
    pub date: i64,
}
