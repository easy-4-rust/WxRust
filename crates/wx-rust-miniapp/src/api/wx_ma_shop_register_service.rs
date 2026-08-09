//! 小程序交易组件-申请接入服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopRegisterService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::request::{
    WxMaShopRegisterApplySceneRequest, WxMaShopRegisterFinishAccessInfoRequest,
};
use crate::bean::shop::response::{WxMaShopBaseResponse, WxMaShopRegisterCheckResponse};

/// 小程序交易组件-申请接入服务。
#[async_trait]
pub trait WxMaShopRegisterService: Send + Sync {
    /// 接入申请（对应 Java `registerApply()`）。
    async fn register_apply(&self) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 获取接入状态（对应 Java `registerCheck()`）。
    async fn register_check(&self) -> Result<WxMaShopRegisterCheckResponse, WxErrorException>;

    /// 完成接入任务（对应 Java `registerFinishAccessInfo(WxMaShopRegisterFinishAccessInfoRequest)`）。
    async fn register_finish_access_info(
        &self,
        request: &WxMaShopRegisterFinishAccessInfoRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 场景接入申请（对应 Java `registerApplyScene(WxMaShopRegisterApplySceneRequest)`）。
    async fn register_apply_scene(
        &self,
        request: &WxMaShopRegisterApplySceneRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;
}
