//! 对应 Java `me.chanjar.weixin.cp.bean.kf.msg.WxCpKfChannelsShopOrderMsg.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::kf::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfChannelsShopOrderMsg {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "product_titles", default)]
    pub product_titles: String,
    #[serde(rename = "price_wording", default)]
    pub price_wording: String,
    #[serde(rename = "state", default)]
    pub state: String,
    #[serde(rename = "image_url", default)]
    pub image_url: String,
    #[serde(rename = "shop_nickname", default)]
    pub shop_nickname: String,
}
