//! 对应 Java `me.chanjar.weixin.channel.bean.talent.TalentOrderDetailParam.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderDetailParam {
    /// 佣金单号
    #[serde(rename = "order_id", default)]
    pub order_id: String,
}
