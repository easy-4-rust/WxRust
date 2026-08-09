//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.request.WxMaExpressOrderShop.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressOrderShop {
    #[serde(rename = "wxa_path", default)]
    pub wxa_path: String,
    #[serde(rename = "img_url", default)]
    pub img_url: String,
    #[serde(rename = "goods_name", default)]
    pub goods_name: String,
    #[serde(rename = "goods_count", default)]
    pub goods_count: i32,
    #[serde(rename = "detail_list", default)]
    pub detail_list: Vec<WxMaExpressOrderShopDetail>,
}
