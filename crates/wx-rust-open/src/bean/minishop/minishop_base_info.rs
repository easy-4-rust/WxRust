//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopBaseInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopBaseInfo {
    #[serde(rename = "miniShopId", default)]
    pub mini_shop_id: i64,
    #[serde(rename = "appId", default)]
    pub app_id: String,
    #[serde(rename = "nickName", default)]
    pub nick_name: String,
    #[serde(rename = "abbr", default)]
    pub abbr: String,
    #[serde(rename = "introduction", default)]
    pub introduction: String,
    #[serde(rename = "namingOtherStuff", default)]
    pub naming_other_stuff: String,
    #[serde(rename = "mail", default)]
    pub mail: String,
    #[serde(rename = "returnAddressId", default)]
    pub return_address_id: i32,
    #[serde(rename = "companyAddressId", default)]
    pub company_address_id: i32,
}
