//! 对应 Java `com.binarywang.wxjava.store.bean.after.GuaranteeRefuseRequest.java`。

/// 商家拒绝保障单请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeRefuseRequest {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_order_id: Option<String>,
    /// 拒绝原因。
    #[serde(rename = "reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 拒绝凭证图片 media_id 列表。
    #[serde(rename = "pic_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic_list: Option<Vec<String>>,
}
