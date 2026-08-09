//! 对应 Java `cn.binarywang.wx.miniapp.bean.express.request.WxMaExpressOrderShopDetail.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::express::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaExpressOrderShopDetail {
    #[serde(rename = "goods_name", default)]
    pub goods_name: String,
    #[serde(rename = "goods_img_url", default)]
    pub goods_img_url: String,
    #[serde(rename = "goods_desc", default)]
    pub goods_desc: String,
}
