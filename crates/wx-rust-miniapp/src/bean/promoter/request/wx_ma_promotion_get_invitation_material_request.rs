//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.request.WxMaPromotionGetInvitationMaterialRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionGetInvitationMaterialRequest {
    #[serde(rename = "role_id", default)]
    pub role_id: i64,
    #[serde(rename = "invitation_type", default)]
    pub invitation_type: i64,
}
