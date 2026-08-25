//! 对应 Java `me.chanjar.weixin.channel.bean.talent.TalentWindowProductListResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentWindowProductListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 橱窗商品列表
    #[serde(rename = "product_list", default)]
    pub product_list: Vec<TalentWindowProductInfo>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentWindowProductInfo {
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// 商品名称
    #[serde(rename = "product_name", default)]
    pub product_name: String,
}
