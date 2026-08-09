//! 对应 Java `me.chanjar.weixin.open.bean.icp.WxOpenOnlineIcpOrderResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenOnlineIcpOrderResult {
    #[serde(rename = "icp_subject", default)]
    pub icp_subject: Subject,
    #[serde(rename = "icp_applets", default)]
    pub icp_applets: Applets,
}
