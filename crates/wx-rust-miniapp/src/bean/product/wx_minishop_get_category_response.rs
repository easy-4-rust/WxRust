//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopGetCategoryResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopGetCategoryResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "cat_list", default)]
    pub cat_list: Vec<MinishopCatItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopCatItem {
    #[serde(rename = "cat_id", default)]
    pub cat_id: i32,
    #[serde(rename = "f_cat_id", default)]
    pub f_cat_id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
}
