//! 对应 Java `me.chanjar.weixin.open.bean.icp.WxOpenCreateIcpVerifyTaskParam.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenCreateIcpVerifyTaskParam {
    #[serde(rename = "along_with_auth", default)]
    pub along_with_auth: bool,
}
