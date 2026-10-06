//! 对应 Java `com.binarywang.wxjava.store.bean.after.GuaranteeProofRequest.java`。

/// 商家举证保障单请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeProofRequest {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_order_id: Option<String>,
    /// 举证内容。
    #[serde(rename = "content", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 举证图片 media_id 列表。
    #[serde(rename = "pic_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic_list: Option<Vec<String>>,
}
