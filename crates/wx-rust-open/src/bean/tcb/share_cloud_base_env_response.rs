//! 对应 Java `me.chanjar.weixin.open.bean.tcb.ShareCloudBaseEnvResponse.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShareCloudBaseEnvResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "err_list", default)]
    pub err_list: Vec<ErrListDTO>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ErrListDTO {
    #[serde(rename = "env", default)]
    pub env: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
}
