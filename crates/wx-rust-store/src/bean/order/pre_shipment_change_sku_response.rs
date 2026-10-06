//! 对应 Java `com.binarywang.wxjava.store.bean.order.PreShipmentChangeSkuResponse.java`。

use super::ChangeSkuInfo;

/// 获取待发货前更换 SKU 待处理请求响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreShipmentChangeSkuResponse {
    /// 错误码。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 更换 SKU 信息。
    #[serde(rename = "change_sku_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_sku_info: Option<ChangeSkuInfo>,
}
