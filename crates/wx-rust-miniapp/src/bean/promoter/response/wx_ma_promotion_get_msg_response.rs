//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.response.WxMaPromotionGetMsgResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionGetMsgResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "send_cnt", default)]
    pub send_cnt: i64,
    #[serde(rename = "percent", default)]
    pub percent: i64,
    #[serde(rename = "fail_cnt", default)]
    pub fail_cnt: i64,
    #[serde(rename = "fail_info", default)]
    pub fail_info: Vec<FailInfo>,
    #[serde(rename = "fail_info_url", default)]
    pub fail_info_url: String,
    #[serde(rename = "msg_type", default)]
    pub msg_type: i64,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "list_type", default)]
    pub list_type: i64,
    #[serde(rename = "role_id", default)]
    pub role_id: Vec<i64>,
    #[serde(rename = "retail_id", default)]
    pub retail_id: Vec<String>,
    #[serde(rename = "id", default)]
    pub id: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FailInfo {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "errorcode", default)]
    pub errorcode: i64,
}
