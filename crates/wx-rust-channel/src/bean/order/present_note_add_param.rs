//! 对应 Java `me.chanjar.weixin.channel.bean.order.PresentNoteAddParam.java`。

#[allow(unused_imports)]
use super::*;

/// 礼物订单新增备注信息请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PresentNoteAddParam {
    /// 礼物订单 ID。
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// 备注内容。
    #[serde(rename = "note", default)]
    pub note: String,
}
