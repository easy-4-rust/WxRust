//! 对应 Java `com.binarywang.wxjava.store.bean.talent.TalentWindowProductDetailParam.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TalentWindowProductDetailParam；对应 Java com.binarywang.wxjava.store.bean.talent.TalentWindowProductDetailParam.java。
pub struct TalentWindowProductDetailParam {
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}
