//! 使用授权码换取授权信息的结果。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenQueryAuthResult`。
//! 由 `WxOpenQueryAuthResultGsonAdapter` 驱动解析（`authorization_info`
//! 键），与字段名直映不同，故人工迁移。

use crate::bean::auth::WxOpenAuthorizationInfo;

/// 使用授权码换取授权信息的结果。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenQueryAuthResult {
    /// 授权信息。
    #[serde(rename = "authorization_info", default)]
    pub authorization_info: Option<WxOpenAuthorizationInfo>,
}
