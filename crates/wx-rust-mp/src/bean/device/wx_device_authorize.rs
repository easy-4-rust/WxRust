//! 对应 Java `bean.device.WxDeviceAuthorize`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDeviceAuthorize {
    #[serde(rename = "device_num", default)]
    pub device_num: String,
    #[serde(rename = "op_type", default)]
    pub op_type: String,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "device_list", default)]
    pub device_list: Vec<WxDevice>,
}
