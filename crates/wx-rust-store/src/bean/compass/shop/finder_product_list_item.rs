//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.FinderProductListItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 FinderProductListItem；对应 Java com.binarywang.wxjava.store.bean.compass.shop.FinderProductListItem.java。
pub struct FinderProductListItem {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "head_img_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_img_url: Option<String>,
    #[serde(rename = "title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "first_category_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_category_id: Option<String>,
    #[serde(rename = "second_category_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_category_id: Option<String>,
    #[serde(rename = "third_category_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_category_id: Option<String>,
    #[serde(rename = "data", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GmvData>,
}

/// 微信小店 GmvData 数据类型；对应 Java com.binarywang.wxjava.store.bean.compass.shop.FinderProductListItem.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GmvData {
    #[serde(rename = "commission_ratio", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_ratio: Option<f64>,
    #[serde(rename = "pay_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_gmv: Option<String>,
}
