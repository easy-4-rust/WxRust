//! 对应 Java `cn.binarywang.wx.miniapp.bean.intractiy.WxMaPreAddOrderResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPreAddOrderResponse {
    #[serde(rename = "serviceTransId", default)]
    pub service_trans_id: String,
    #[serde(rename = "distance", default)]
    pub distance: i32,
    #[serde(rename = "estFee", default)]
    pub est_fee: i32,
    #[serde(rename = "expectedFinishedTime", default)]
    pub expected_finished_time: i64,
    #[serde(rename = "promiseDeliveryTime", default)]
    pub promise_delivery_time: i32,
}
