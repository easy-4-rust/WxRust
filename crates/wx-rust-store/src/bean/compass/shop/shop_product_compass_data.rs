//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.ShopProductCompassData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ShopProductCompassData；对应 Java com.binarywang.wxjava.store.bean.compass.shop.ShopProductCompassData.java。
pub struct ShopProductCompassData {
    #[serde(rename = "pay_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_gmv: Option<String>,
    #[serde(rename = "create_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_gmv: Option<String>,
    #[serde(rename = "create_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_cnt: Option<String>,
    #[serde(rename = "create_uv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_uv: Option<String>,
    #[serde(rename = "create_product_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_product_cnt: Option<String>,
    #[serde(rename = "pay_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_cnt: Option<String>,
    #[serde(rename = "pay_uv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_uv: Option<String>,
    #[serde(rename = "pay_product_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_product_cnt: Option<String>,
    #[serde(rename = "pure_pay_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pure_pay_gmv: Option<String>,
    #[serde(rename = "pay_gmv_per_uv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_gmv_per_uv: Option<String>,
    #[serde(rename = "seller_actual_settle_amount", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_actual_settle_amount: Option<String>,
    #[serde(rename = "platform_actual_commission", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_actual_commission: Option<String>,
    #[serde(rename = "finderuin_actual_commission", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finderuin_actual_commission: Option<String>,
    #[serde(rename = "captain_actual_commission", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captain_actual_commission: Option<String>,
    #[serde(rename = "seller_predict_settle_amount", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_predict_settle_amount: Option<String>,
    #[serde(rename = "platform_predict_commission", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_predict_commission: Option<String>,
    #[serde(rename = "finderuin_predict_commission", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finderuin_predict_commission: Option<String>,
    #[serde(rename = "captain_predict_commission", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captain_predict_commission: Option<String>,
    #[serde(rename = "product_click_uv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_click_uv: Option<String>,
    #[serde(rename = "product_click_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_click_cnt: Option<String>,
    #[serde(rename = "pay_refund_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_refund_gmv: Option<String>,
    #[serde(rename = "pay_refund_uv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_refund_uv: Option<String>,
    #[serde(rename = "pay_refund_ratio", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_refund_ratio: Option<f64>,
    #[serde(rename = "pay_refund_after_send_ratio", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_refund_after_send_ratio: Option<f64>,
    #[serde(rename = "pay_refund_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_refund_cnt: Option<String>,
    #[serde(rename = "pay_refund_product_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_refund_product_cnt: Option<String>,
    #[serde(rename = "pay_refund_before_send_ratio", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_refund_before_send_ratio: Option<f64>,
    #[serde(rename = "refund_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_gmv: Option<String>,
    #[serde(rename = "refund_product_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_product_cnt: Option<String>,
    #[serde(rename = "refund_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_cnt: Option<String>,
    #[serde(rename = "refund_uv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_uv: Option<String>,
}
