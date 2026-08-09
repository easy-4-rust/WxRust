//! 对应 Java `cn.binarywang.wx.miniapp.bean.internet.WxMaInternetUserKeyInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaInternetUserKeyInfo {
    #[serde(rename = "encrypt_key", default)]
    pub encrypt_key: String,
    #[serde(rename = "version", default)]
    pub version: i32,
    #[serde(rename = "expire_in", default)]
    pub expire_in: i64,
    #[serde(rename = "iv", default)]
    pub iv: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
}
