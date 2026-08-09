//! 对应 Java `me.chanjar.weixin.open.bean.tcb.ShareCloudBaseEnvRequest.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShareCloudBaseEnvRequest {
    #[serde(rename = "data", default)]
    pub data: Vec<DataDTO>,
    #[serde(rename = "source_type", default)]
    pub source_type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DataDTO {
    #[serde(rename = "env", default)]
    pub env: String,
    #[serde(rename = "appids", default)]
    pub appids: Vec<String>,
}
