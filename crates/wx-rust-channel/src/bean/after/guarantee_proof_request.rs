//! 对应 Java `me.chanjar.weixin.channel.bean.after.GuaranteeProofRequest.java`。

#[allow(unused_imports)]
use super::*;

/// 商家举证保障单请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeProofRequest {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    pub guarantee_order_id: String,
    /// 举证内容。
    #[serde(rename = "content", default)]
    pub content: String,
    /// 举证图片 media_id 列表。
    #[serde(rename = "pic_list", default)]
    pub pic_list: Vec<String>,
}
