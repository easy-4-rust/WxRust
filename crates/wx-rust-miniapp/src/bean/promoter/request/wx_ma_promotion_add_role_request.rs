//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.request.WxMaPromotionAddRoleRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionAddRoleRequest {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "desc", default)]
    pub desc: String,
}
