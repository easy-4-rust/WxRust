//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.DropshipListResponse.java`。

#[allow(unused_imports)]
use super::dropship_info::DropshipInfo;

/// 微信小店 DropshipListResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.supplier.DropshipListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropshipListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 代发单列表
    #[serde(rename = "dropship_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropship_list: Option<Vec<DropshipInfo>>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 has_more。
    #[serde(rename = "has_more", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}
