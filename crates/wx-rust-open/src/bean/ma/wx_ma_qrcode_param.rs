//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxMaQrcodeParam.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaQrcodeParam {
    #[serde(rename = "pagePath", default)]
    pub page_path: String,
    #[serde(rename = "pageParams", default)]
    pub page_params: std::collections::HashMap<String, String>,
}
