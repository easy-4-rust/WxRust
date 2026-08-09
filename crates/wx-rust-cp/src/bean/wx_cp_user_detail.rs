//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpUserDetail.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUserDetail {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "mobile", default)]
    pub mobile: String,
    #[serde(rename = "gender", default)]
    pub gender: String,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "avatar", default)]
    pub avatar: String,
    #[serde(rename = "qr_code", default)]
    pub qr_code: String,
    #[serde(rename = "biz_mail", default)]
    pub biz_mail: String,
    #[serde(rename = "address", default)]
    pub address: String,
}
