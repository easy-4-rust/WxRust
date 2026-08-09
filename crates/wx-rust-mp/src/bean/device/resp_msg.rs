//! 对应 Java `bean.device.RespMsg`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RespMsg {
    #[serde(rename = "ret_code", default)]
    pub ret_code: i32,
    #[serde(rename = "error_info", default)]
    pub error_info: String,
}
