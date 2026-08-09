//! 对应 Java `me.chanjar.weixin.open.bean.tcbComponent.GetShareCloudBaseEnvResponse.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetShareCloudBaseEnvResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "relation_data", default)]
    pub relation_data: Vec<RelationDataDTO>,
    #[serde(rename = "err_list", default)]
    pub err_list: Vec<ErrListDTO>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RelationDataDTO {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "env_list", default)]
    pub env_list: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ErrListDTO {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
}
