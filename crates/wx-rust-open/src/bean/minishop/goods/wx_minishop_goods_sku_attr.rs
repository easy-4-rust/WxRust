//! 对应 Java `me.chanjar.weixin.open.bean.minishop.goods.WxMinishopGoodsSkuAttr.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopGoodsSkuAttr {
    #[serde(rename = "attrKey", default)]
    pub attr_key: String,
    #[serde(rename = "attrValue", default)]
    pub attr_value: String,
}
