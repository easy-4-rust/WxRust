//! 对应 Java `com.binarywang.wxjava.store.bean.compass.shop.FinderProductSimpleGmvData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 FinderProductSimpleGmvData；对应 Java com.binarywang.wxjava.store.bean.compass.shop.FinderProductSimpleGmvData.java。
pub struct FinderProductSimpleGmvData {
    #[serde(rename = "commission_ratio", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_ratio: Option<f64>,
    #[serde(rename = "pay_gmv", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_gmv: Option<String>,
}
