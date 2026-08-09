//! 对应 Java `bean.datacube.WxDataCubeInterfaceResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxDataCubeInterfaceResult {
    #[serde(rename = "ref_hour", default)]
    pub ref_hour: i32,
    #[serde(rename = "callback_count", default)]
    pub callback_count: i32,
    #[serde(rename = "fail_count", default)]
    pub fail_count: i32,
    #[serde(rename = "total_time_cost", default)]
    pub total_time_cost: i32,
    #[serde(rename = "max_time_cost", default)]
    pub max_time_cost: i32,
}
