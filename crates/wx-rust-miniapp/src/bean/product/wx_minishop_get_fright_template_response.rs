//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopGetFrightTemplateResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopGetFrightTemplateResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "template_list", default)]
    pub template_list: Vec<MinishopFeightTemplateItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopFeightTemplateItem {
    #[serde(rename = "template_id", default)]
    pub template_id: i64,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "valuation_type", default)]
    pub valuation_type: i32,
}
