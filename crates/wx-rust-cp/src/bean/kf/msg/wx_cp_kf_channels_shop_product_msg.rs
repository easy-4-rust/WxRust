//! 对应 Java `me.chanjar.weixin.cp.bean.kf.msg.WxCpKfChannelsShopProductMsg.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::kf::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfChannelsShopProductMsg {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "head_img", default)]
    pub head_img: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "sales_price", default)]
    pub sales_price: String,
    #[serde(rename = "shop_nickname", default)]
    pub shop_nickname: String,
    #[serde(rename = "shop_head_img", default)]
    pub shop_head_img: String,
}
