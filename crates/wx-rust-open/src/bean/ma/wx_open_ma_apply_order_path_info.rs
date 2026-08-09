//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxOpenMaApplyOrderPathInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaApplyOrderPathInfo {
    #[serde(rename = "batch_req", default)]
    pub batch_req: BatchReqBean,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchReqBean {
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "img_list", default)]
    pub img_list: Vec<String>,
    #[serde(rename = "video", default)]
    pub video: String,
    #[serde(rename = "test_account", default)]
    pub test_account: String,
    #[serde(rename = "test_pwd", default)]
    pub test_pwd: String,
    #[serde(rename = "test_remark", default)]
    pub test_remark: String,
    #[serde(rename = "appid_list", default)]
    pub app_id_list: Vec<String>,
}
