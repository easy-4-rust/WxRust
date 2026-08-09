//! 对应 Java `com.github.binarywang.wxpay.bean.payscore.Device.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Device {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "start_device_id"
    )]
    pub start_device_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "end_device_id"
    )]
    pub end_device_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "materiel_no"
    )]
    pub materiel_no: Option<String>,
}
