//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopSharerUnbindResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopSharerUnbindResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "success_list", default)]
    pub success_list: Vec<String>,
    #[serde(rename = "fail_list", default)]
    pub fail_list: Vec<String>,
    #[serde(rename = "refuse_list", default)]
    pub refuse_list: Vec<String>,
}
