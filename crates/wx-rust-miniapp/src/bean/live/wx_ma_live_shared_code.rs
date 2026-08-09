//! 对应 Java `cn.binarywang.wx.miniapp.bean.live.WxMaLiveSharedCode.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaLiveSharedCode {
    #[serde(rename = "cdnUrl", default)]
    pub cdn_url: String,
    #[serde(rename = "pagePath", default)]
    pub page_path: String,
    #[serde(rename = "posterUrl", default)]
    pub poster_url: String,
}
