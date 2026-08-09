//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopGetPaymentParamsResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopGetPaymentParamsResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "payment_params", default)]
    pub payment_params: PaymentParams,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentParams {
    #[serde(rename = "timeStamp", default)]
    pub time_stamp: String,
    #[serde(rename = "nonceStr", default)]
    pub nonce_str: String,
    #[serde(rename = "package", default)]
    pub package_value: String,
    #[serde(rename = "signType", default)]
    pub sign_type: String,
    #[serde(rename = "paySign", default)]
    pub pay_sign: String,
}
