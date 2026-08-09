//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopAuditCategoryRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopAuditCategoryRequest {
    #[serde(rename = "audit_req", default)]
    pub audit_req: AuditReqBean,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuditReqBean {
    #[serde(rename = "category_info", default)]
    pub category_info: CategoryInfoBean,
    #[serde(rename = "license", default)]
    pub license: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryInfoBean {
    #[serde(rename = "level1", default)]
    pub level1: i32,
    #[serde(rename = "level2", default)]
    pub level2: i32,
    #[serde(rename = "level3", default)]
    pub level3: i32,
    #[serde(rename = "certificate", default)]
    pub certificate: Vec<String>,
}
