//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.FinderOverallData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 FinderOverallData；对应 Java com.binarywang.wxjava.store.bean.compass.shop.FinderOverallData.java。
pub struct FinderOverallData {
    #[serde(rename = "pay_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_gmv: Option<String>,
    #[serde(rename = "pay_sales_finder_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_sales_finder_cnt: Option<String>,
    #[serde(rename = "pay_product_id_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_product_id_cnt: Option<String>,
    #[serde(rename = "click_to_pay_uv_ratio", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_to_pay_uv_ratio: Option<f64>,
}
