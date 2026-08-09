//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopGetSpuResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopGetSpuResult {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "head_img", default)]
    pub head_img: Vec<String>,
    #[serde(rename = "qualification_pics", default)]
    pub qualification_pics: Vec<String>,
    #[serde(rename = "desc_info", default)]
    pub desc_info: WxMaShopSpuDescInfo,
    #[serde(rename = "third_cat_id", default)]
    pub third_cat_id: i32,
    #[serde(rename = "brand_id", default)]
    pub brand_id: i32,
    #[serde(rename = "skus", default)]
    pub skus: Vec<WxMaShopSkuInfo>,
    #[serde(rename = "scene_group_list", default)]
    pub scene_group_list: Vec<i32>,
    #[serde(rename = "item_type", default)]
    pub item_type: i32,
    #[serde(rename = "audit_info", default)]
    pub audit_info: WxMaShopSpuAudit,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "edit_status", default)]
    pub edit_status: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "update_time", default)]
    pub update_time: String,
}
