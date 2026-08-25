//! 对应 Java `me.chanjar.weixin.channel.bean.talent.TalentWindowProductDetailResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

use super::talent_window_product_list_response::TalentWindowProductInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentWindowProductDetailResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 橱窗商品详情
    #[serde(rename = "product_detail", default)]
    pub product_detail: TalentWindowProductInfo,
}
