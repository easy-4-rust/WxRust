//! 对应 Java `me.chanjar.weixin.channel.bean.product.assistant.CategoryPreCheckParam.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryPreCheckParam {
    /// 类目 ID
    #[serde(rename = "category_id", default)]
    pub category_id: String,
}
