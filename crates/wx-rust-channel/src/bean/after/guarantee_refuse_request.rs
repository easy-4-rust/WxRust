//! 对应 Java `me.chanjar.weixin.channel.bean.after.GuaranteeRefuseRequest.java`。

#[allow(unused_imports)]
use super::*;

/// 商家拒绝保障单请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeRefuseRequest {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    pub guarantee_order_id: String,
    /// 拒绝原因。
    #[serde(rename = "reason", default)]
    pub reason: String,
    /// 拒绝凭证图片 media_id 列表。
    #[serde(rename = "pic_list", default)]
    pub pic_list: Vec<String>,
}
