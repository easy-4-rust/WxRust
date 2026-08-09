//! 小程序推广员服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaPromotionService`
//! （`impl.WxMaPromotionServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::promoter::{
    WxMaPromoterUpdateRoleRequest, WxMaPromotionAddPromoterRequest,
    WxMaPromotionAddPromoterResponse, WxMaPromotionAddRoleRequest, WxMaPromotionAddRoleResponse,
    WxMaPromotionGetInvitationMaterialRequest, WxMaPromotionGetInvitationMaterialResponse,
    WxMaPromotionGetMsgClickDataRequest, WxMaPromotionGetMsgClickDataResponse,
    WxMaPromotionGetMsgRequest, WxMaPromotionGetMsgResponse, WxMaPromotionGetOrderRequest,
    WxMaPromotionGetOrderResponse, WxMaPromotionGetPromoterRequest,
    WxMaPromotionGetPromoterResponse, WxMaPromotionGetRelationRequest,
    WxMaPromotionGetRelationResponse, WxMaPromotionGetRoleRequest, WxMaPromotionGetRoleResponse,
    WxMaPromotionGetShareMaterialRequest, WxMaPromotionGetShareMaterialResponse,
    WxMaPromotionSendMsgRequest, WxMaPromotionSendMsgResponse, WxMaPromotionSingleSendMsgRequest,
    WxMaPromotionSingleSendMsgResponse, WxMaPromotionUpdatePromoterRequest,
    WxMaPromotionUpdatePromoterResponse, WxMaPromotionUpdateRoleResponse,
};

/// 小程序推广员服务。
///
/// 对应 Java `WxMaPromotionService`：角色管理、推广员管理、邀请素材、消息
/// 管理（群发/单发/送达/点击）、推广数据（素材/触达/订单）。
#[async_trait]
pub trait WxMaPromotionService: Send + Sync {
    /// 新增角色（对应 Java `addRole`）。
    async fn add_role(
        &self,
        request: &WxMaPromotionAddRoleRequest,
    ) -> Result<WxMaPromotionAddRoleResponse, WxErrorException>;

    /// 查询角色（对应 Java `getRole`）。
    async fn get_role(
        &self,
        request: &WxMaPromotionGetRoleRequest,
    ) -> Result<WxMaPromotionGetRoleResponse, WxErrorException>;

    /// 修改角色（对应 Java `updateRole`）。
    async fn update_role(
        &self,
        request: &WxMaPromoterUpdateRoleRequest,
    ) -> Result<WxMaPromotionUpdateRoleResponse, WxErrorException>;

    /// 声明推广员身份（对应 Java `addPromoter`）。
    async fn add_promoter(
        &self,
        request: &WxMaPromotionAddPromoterRequest,
    ) -> Result<WxMaPromotionAddPromoterResponse, WxErrorException>;

    /// 查询推广员身份（对应 Java `getPromoter`）。
    async fn get_promoter(
        &self,
        request: &WxMaPromotionGetPromoterRequest,
    ) -> Result<WxMaPromotionGetPromoterResponse, WxErrorException>;

    /// 修改推广员身份（对应 Java `updatePromoter`）。
    async fn update_promoter(
        &self,
        request: &WxMaPromotionUpdatePromoterRequest,
    ) -> Result<WxMaPromotionUpdatePromoterResponse, WxErrorException>;

    /// 获取推广员邀请素材（对应 Java `getInvitationMaterial`）。
    async fn get_invitation_material(
        &self,
        request: &WxMaPromotionGetInvitationMaterialRequest,
    ) -> Result<WxMaPromotionGetInvitationMaterialResponse, WxErrorException>;

    /// 群发消息（对应 Java `sendMsg`）。
    async fn send_msg(
        &self,
        request: &WxMaPromotionSendMsgRequest,
    ) -> Result<WxMaPromotionSendMsgResponse, WxErrorException>;

    /// 单发消息（对应 Java `singleSendMsg`）。
    async fn single_send_msg(
        &self,
        request: &WxMaPromotionSingleSendMsgRequest,
    ) -> Result<WxMaPromotionSingleSendMsgResponse, WxErrorException>;

    /// 查询送达结果（对应 Java `getMsg`）。
    async fn get_msg(
        &self,
        request: &WxMaPromotionGetMsgRequest,
    ) -> Result<WxMaPromotionGetMsgResponse, WxErrorException>;

    /// 分析点击效果（对应 Java `getMsgClickData`）。
    async fn get_msg_click_data(
        &self,
        request: &WxMaPromotionGetMsgClickDataRequest,
    ) -> Result<WxMaPromotionGetMsgClickDataResponse, WxErrorException>;

    /// 生成推广素材（对应 Java `getShareMaterial`）。
    async fn get_share_material(
        &self,
        request: &WxMaPromotionGetShareMaterialRequest,
    ) -> Result<WxMaPromotionGetShareMaterialResponse, WxErrorException>;

    /// 分析触达效果（对应 Java `getRelation`）。
    async fn get_relation(
        &self,
        request: &WxMaPromotionGetRelationRequest,
    ) -> Result<WxMaPromotionGetRelationResponse, WxErrorException>;

    /// 查询推广订单（对应 Java `getOrder`）。
    async fn get_order(
        &self,
        request: &WxMaPromotionGetOrderRequest,
    ) -> Result<WxMaPromotionGetOrderResponse, WxErrorException>;
}
