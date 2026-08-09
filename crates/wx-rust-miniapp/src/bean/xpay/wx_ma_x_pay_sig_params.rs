//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPaySigParams.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPaySigParams {
    #[serde(rename = "sessionKey", default)]
    pub session_key: String,
    #[serde(rename = "appKey", default)]
    pub app_key: String,
}
