//! 对应 Java `cn.binarywang.wx.miniapp.bean.complaint.WxMaComplaintNotifyUrlResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaComplaintNotifyUrlResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
}
