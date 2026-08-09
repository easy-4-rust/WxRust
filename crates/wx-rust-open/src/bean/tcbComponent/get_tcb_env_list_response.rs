//! 对应 Java `me.chanjar.weixin.open.bean.tcbComponent.GetTcbEnvListResponse.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetTcbEnvListResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "info_list", default)]
    pub info_list: Vec<InfoListDTO>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InfoListDTO {
    #[serde(rename = "env", default)]
    pub env: String,
    #[serde(rename = "alias", default)]
    pub alias: String,
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "update_time", default)]
    pub update_time: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "package_id", default)]
    pub package_id: String,
    #[serde(rename = "package_name", default)]
    pub package_name: String,
    #[serde(rename = "dbinstance_id", default)]
    pub dbinstance_id: String,
    #[serde(rename = "bucket_id", default)]
    pub bucket_id: String,
}
