//! 对应 Java `bean.card.BaseInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BaseInfo {
    #[serde(rename = "logo_url", default)]
    pub logo_url: String,
    #[serde(rename = "code_type", default)]
    pub code_type: String,
    #[serde(rename = "pay_info", default)]
    pub pay_info: PayInfo,
    #[serde(rename = "is_pay_and_qrcode", default)]
    pub is_pay_and_qrcode: bool,
    #[serde(rename = "brand_name", default)]
    pub brand_name: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "color", default)]
    pub color: String,
    #[serde(rename = "notice", default)]
    pub notice: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "sku", default)]
    pub sku: Sku,
    #[serde(rename = "date_info", default)]
    pub date_info: DateInfo,
    #[serde(rename = "use_custom_code", default)]
    pub use_custom_code: bool,
    #[serde(rename = "bind_openid", default)]
    pub bind_openid: bool,
    #[serde(rename = "service_phone", default)]
    pub service_phone: String,
    #[serde(rename = "location_id_list", default)]
    pub location_id_list: Vec<String>,
    #[serde(rename = "use_all_locations", default)]
    pub use_all_locations: bool,
    #[serde(rename = "center_title", default)]
    pub center_title: String,
    #[serde(rename = "center_sub_title", default)]
    pub center_sub_title: String,
    #[serde(rename = "center_url", default)]
    pub center_url: String,
    #[serde(rename = "custom_url_name", default)]
    pub custom_url_name: String,
    #[serde(rename = "custom_url", default)]
    pub custom_url: String,
    #[serde(rename = "custom_url_sub_title", default)]
    pub custom_url_sub_title: String,
    #[serde(rename = "promotion_url_name", default)]
    pub promotion_url_name: String,
    #[serde(rename = "promotion_url", default)]
    pub promotion_url: String,
    #[serde(rename = "promotion_url_sub_title", default)]
    pub promotion_url_sub_title: String,
    #[serde(rename = "get_limit", default)]
    pub get_limit: i32,
    #[serde(rename = "use_limit", default)]
    pub use_limit: i32,
    #[serde(rename = "can_share", default)]
    pub can_share: bool,
    #[serde(rename = "can_give_friend", default)]
    pub can_give_friend: bool,
    #[serde(rename = "need_push_on_view", default)]
    pub need_push_on_view: bool,
    #[serde(rename = "custom_app_brand_user_name", default)]
    pub custom_app_brand_user_name: String,
    #[serde(rename = "custom_app_brand_pass", default)]
    pub custom_app_brand_pass: String,
    #[serde(rename = "center_app_brand_user_name", default)]
    pub center_app_brand_user_name: String,
    #[serde(rename = "center_app_brand_pass", default)]
    pub center_app_brand_pass: String,
    #[serde(rename = "promotion_app_brand_user_name", default)]
    pub promotion_app_brand_user_name: String,
    #[serde(rename = "promotion_app_brand_pass", default)]
    pub promotion_app_brand_pass: String,
    #[serde(rename = "activate_app_brand_user_name", default)]
    pub activate_app_brand_user_name: String,
    #[serde(rename = "activate_app_brand_pass", default)]
    pub activate_app_brand_pass: String,
    #[serde(rename = "status", default)]
    pub status: String,
}
