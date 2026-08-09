//! 对应 Java `cn.binarywang.wx.miniapp.bean.cloud.request.WxCloudSendSmsV2Request.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::cloud::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCloudSendSmsV2Request {
    #[serde(rename = "env", default)]
    pub env: String,
    #[serde(rename = "url_link", default)]
    pub url_link: String,
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "template_param_list", default)]
    pub template_param_list: Vec<String>,
    #[serde(rename = "phone_number_list", default)]
    pub phone_number_list: Vec<String>,
    #[serde(rename = "use_short_name", default)]
    pub use_short_name: bool,
    #[serde(rename = "resource_appid", default)]
    pub resource_appid: String,
}
