//! 对应 Java `me.chanjar.weixin.channel.bean.talent.TalentWindowProductDetailParam.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentWindowProductDetailParam {
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    pub product_id: String,
}
