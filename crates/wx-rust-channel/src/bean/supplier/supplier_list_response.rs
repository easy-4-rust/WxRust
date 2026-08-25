//! 对应 Java `me.chanjar.weixin.channel.bean.supplier.SupplierListResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

use super::supplier_info::SupplierInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SupplierListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 供货商列表
    #[serde(rename = "supplier_list", default)]
    pub supplier_list: Vec<SupplierInfo>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}
