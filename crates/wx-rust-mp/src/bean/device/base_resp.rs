//! 对应 Java `bean.device.BaseResp`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BaseResp {
    #[serde(rename = "base_info", default)]
    pub base_info: BaseInfo,
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BaseInfo {
    #[serde(rename = "device_type", default)]
    pub device_type: String,
    #[serde(rename = "device_id", default)]
    pub device_id: String,
}
