//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopReturnInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopReturnInfo {
    #[serde(rename = "addressInfo", default)]
    pub address_info: MinishopAddressInfo,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "companyAddress", default)]
    pub company_address: MinishopAddressInfo,
}
