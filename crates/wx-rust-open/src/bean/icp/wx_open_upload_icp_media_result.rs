//! 对应 Java `me.chanjar.weixin.open.bean.icp.WxOpenUploadIcpMediaResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenUploadIcpMediaResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "media_id", default)]
    pub media_id: String,
    #[serde(rename = "created_at", default)]
    pub created_at: String,
}
