//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.DropshipDetailResponse.java`。

#[allow(unused_imports)]
use super::dropship_info::DropshipInfo;

/// 微信小店 DropshipDetailResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.supplier.DropshipDetailResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropshipDetailResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 代发单详情
    #[serde(rename = "dropship_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropship_info: Option<DropshipInfo>,
}
