//! 小程序推广员服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaPromotionServiceImpl`。
//! 说明：Java `getPromoter`/`getRelation`/`getOrder` 的错误判断写为
//! `errCode != 0 || errCode != ERR_CODE_OF_EMPTY_LIST(103006)`——该条件恒真
//! （Java 笔误，本意应为 `&&`，允许空列表错误码 103006 通过），照搬会导致
//! 方法必然抛错；本实现按**本意语义**（`&&`：errcode 为 0 或 103006 均视为
//! 成功）实现，并绕过执行引擎的 errcode 预检以放行 103006（ADAPTED，注释
//! 标注）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::enums::WxType;
use wx_rust_common::error::{WxError, WxErrorException};

use crate::api::WxMaService;
use crate::api::g4_services::WxMaPromotionService;
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
use crate::config::DEFAULT_API_HOST_URL;
use crate::enums::g4_urls::url_g4_ability::promotion as promotion_url;

/// 小程序推广员服务实现。
pub struct WxMaPromotionServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaPromotionServiceImpl {
    /// 空列表错误码（对应 Java `ERR_CODE_OF_EMPTY_LIST = 103006`）。
    const ERR_CODE_OF_EMPTY_LIST: i32 = 103006;

    /// 构建推广员服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 序列化请求对象为 JSON（对应 Java 直接 post 请求对象）。
    fn to_json<T: serde::Serialize>(request: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(request).map_err(WxErrorException::from)
    }

    /// POST 请求并解析响应（errcode!=0 由执行引擎抛错，Java 在此再校验
    /// 一次，语义一致）。
    async fn post_as<T>(
        svc: &dyn WxMaService,
        url: &str,
        post_body: &str,
    ) -> Result<T, WxErrorException>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = svc.post(url, post_body).await?;
        serde_json::from_str(&response).map_err(WxErrorException::from)
    }

    /// 允许空列表错误码的 POST（对应 Java `getPromoter`/`getRelation`/
    /// `getOrder` 的本意语义：errcode 为 0 或 103006 均不抛错）。
    ///
    /// 由于执行引擎对 errcode!=0 一律抛错，此处直连 http_client 自行校验
    /// （token 注入 + 自定义域名替换与执行引擎一致）。
    async fn post_allow_empty_list<T>(
        svc: &dyn WxMaService,
        url: &str,
        post_body: &str,
    ) -> Result<T, WxErrorException>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let config = svc.wx_ma_config();
        let access_token = svc.get_access_token().await?;
        let effective_host = config.effective_api_host_url();
        let url = if effective_host != DEFAULT_API_HOST_URL {
            url.replace(DEFAULT_API_HOST_URL, &effective_host)
        } else {
            url.to_string()
        };
        let url_with_token = format!("{url}?access_token={access_token}");
        let text = svc
            .http_client()
            .post(&url_with_token)
            .body(post_body.to_string())
            .send()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("请求失败: {e}")))?
            .text()
            .await
            .map_err(|e| WxErrorException::from_code(-99, format!("请求失败: {e}")))?;
        let error = WxError::from_json_with_type(&text, Some(WxType::MiniApp));
        if error.error_code != 0 && error.error_code != Self::ERR_CODE_OF_EMPTY_LIST {
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        serde_json::from_str(&text).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxMaPromotionService for WxMaPromotionServiceImpl {
    /// 新增角色（对应 Java `WxMaPromotionServiceImpl.addRole`）。
    async fn add_role(
        &self,
        request: &WxMaPromotionAddRoleRequest,
    ) -> Result<WxMaPromotionAddRoleResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &promotion_url::add_role_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 查询角色（对应 Java `WxMaPromotionServiceImpl.getRole`）。
    async fn get_role(
        &self,
        request: &WxMaPromotionGetRoleRequest,
    ) -> Result<WxMaPromotionGetRoleResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &promotion_url::get_role_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 修改角色（对应 Java `WxMaPromotionServiceImpl.updateRole`）。
    async fn update_role(
        &self,
        request: &WxMaPromoterUpdateRoleRequest,
    ) -> Result<WxMaPromotionUpdateRoleResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &promotion_url::update_role_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 声明推广员身份（对应 Java `WxMaPromotionServiceImpl.addPromoter`）。
    async fn add_promoter(
        &self,
        request: &WxMaPromotionAddPromoterRequest,
    ) -> Result<WxMaPromotionAddPromoterResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &promotion_url::add_promoter_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 查询推广员身份（对应 Java `WxMaPromotionServiceImpl.getPromoter`）。
    async fn get_promoter(
        &self,
        request: &WxMaPromotionGetPromoterRequest,
    ) -> Result<WxMaPromotionGetPromoterResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_allow_empty_list(
            svc.as_ref(),
            &promotion_url::get_promoter_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 修改推广员身份（对应 Java `WxMaPromotionServiceImpl.updatePromoter`）。
    async fn update_promoter(
        &self,
        request: &WxMaPromotionUpdatePromoterRequest,
    ) -> Result<WxMaPromotionUpdatePromoterResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &promotion_url::update_promoter_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 获取推广员邀请素材（对应 Java
    /// `WxMaPromotionServiceImpl.getInvitationMaterial`）。
    async fn get_invitation_material(
        &self,
        request: &WxMaPromotionGetInvitationMaterialRequest,
    ) -> Result<WxMaPromotionGetInvitationMaterialResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &promotion_url::get_invitation_material_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 群发消息（对应 Java `WxMaPromotionServiceImpl.sendMsg`）。
    async fn send_msg(
        &self,
        request: &WxMaPromotionSendMsgRequest,
    ) -> Result<WxMaPromotionSendMsgResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &promotion_url::send_msg_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 单发消息（对应 Java `WxMaPromotionServiceImpl.singleSendMsg`）。
    async fn single_send_msg(
        &self,
        request: &WxMaPromotionSingleSendMsgRequest,
    ) -> Result<WxMaPromotionSingleSendMsgResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &promotion_url::single_send_msg_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 查询送达结果（对应 Java `WxMaPromotionServiceImpl.getMsg`）。
    async fn get_msg(
        &self,
        request: &WxMaPromotionGetMsgRequest,
    ) -> Result<WxMaPromotionGetMsgResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &promotion_url::get_msg_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 分析点击效果（对应 Java `WxMaPromotionServiceImpl.getMsgClickData`）。
    async fn get_msg_click_data(
        &self,
        request: &WxMaPromotionGetMsgClickDataRequest,
    ) -> Result<WxMaPromotionGetMsgClickDataResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &promotion_url::get_msg_click_data_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 生成推广素材（对应 Java `WxMaPromotionServiceImpl.getShareMaterial`）。
    async fn get_share_material(
        &self,
        request: &WxMaPromotionGetShareMaterialRequest,
    ) -> Result<WxMaPromotionGetShareMaterialResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &promotion_url::get_share_material_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 分析触达效果（对应 Java `WxMaPromotionServiceImpl.getRelation`）。
    async fn get_relation(
        &self,
        request: &WxMaPromotionGetRelationRequest,
    ) -> Result<WxMaPromotionGetRelationResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_allow_empty_list(
            svc.as_ref(),
            &promotion_url::get_relation_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }

    /// 查询推广订单（对应 Java `WxMaPromotionServiceImpl.getOrder`）。
    async fn get_order(
        &self,
        request: &WxMaPromotionGetOrderRequest,
    ) -> Result<WxMaPromotionGetOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_allow_empty_list(
            svc.as_ref(),
            &promotion_url::get_order_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await
    }
}
