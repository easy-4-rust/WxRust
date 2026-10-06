//! 对应 Java `com.binarywang.wxjava.store.bean.talent.TalentOrderDetailParam.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TalentOrderDetailParam；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderDetailParam.java。
pub struct TalentOrderDetailParam {
    /// 佣金单号
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 上游字段 sku_id。
    #[serde(rename = "sku_id", skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    /// 上游字段 special_id。
    #[serde(
        rename = "special_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_id: Option<String>,
}
