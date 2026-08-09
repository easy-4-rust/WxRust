//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopSuperAdministratorInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopSuperAdministratorInfo {
    #[serde(rename = "superAdminInfoId", default)]
    pub super_admin_info_id: i32,
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "idCardNumber", default)]
    pub id_card_number: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
    #[serde(rename = "mail", default)]
    pub mail: String,
}
