//! 对应 Java `bean.WxMpUserQuery`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpUserQuery {
    #[serde(rename = "queryParamList", default)]
    pub query_param_list: Vec<WxMpUserQueryParam>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpUserQueryParam {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "lang", default)]
    pub lang: String,
}
