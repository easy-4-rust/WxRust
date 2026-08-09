//! 订阅消息服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaSubscribeServiceImpl`：
//! 全部方法委托门面默认实现（门面已镜像 Java Impl 的 URL/请求体/响应解析）。

use async_trait::async_trait;
use std::sync::Weak;
use wx_rust_common::bean::subscribemsg::{
    CategoryData, PubTemplateKeyword, PubTemplateTitleListResult, TemplateInfo,
};
use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g2_services::WxMaSubscribeService;
use crate::bean::{
    WxMaGetUserNotifyRequest, WxMaGetUserNotifyResult, WxMaServiceNotifyExtRequest,
    WxMaServiceNotifyRequest, WxMaSubscribeMessage,
};

/// 订阅消息服务实现。
pub struct WxMaSubscribeServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaSubscribeServiceImpl {
    /// 构建订阅消息服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaSubscribeService for WxMaSubscribeServiceImpl {
    /// 对应 Java `WxMaSubscribeServiceImpl.getPubTemplateTitleList`。
    async fn get_pub_template_title_list(
        &self,
        ids: &[&str],
        start: i32,
        limit: i32,
    ) -> Result<PubTemplateTitleListResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.get_pub_template_title_list(ids, start, limit).await
    }

    /// 对应 Java `WxMaSubscribeServiceImpl.getPubTemplateKeyWordsById`。
    async fn get_pub_template_keywords_by_id(
        &self,
        id: &str,
    ) -> Result<Vec<PubTemplateKeyword>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.get_pub_template_keywords_by_id(id).await
    }

    /// 对应 Java `WxMaSubscribeServiceImpl.addTemplate`。
    async fn add_template(
        &self,
        id: &str,
        keyword_id_list: &[i32],
        scene_desc: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.add_template(id, keyword_id_list, scene_desc).await
    }

    /// 对应 Java `WxMaSubscribeServiceImpl.getTemplateList`。
    async fn get_template_list(&self) -> Result<Vec<TemplateInfo>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.get_template_list().await
    }

    /// 对应 Java `WxMaSubscribeServiceImpl.delTemplate`。
    async fn del_template(&self, template_id: &str) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.del_template(template_id).await
    }

    /// 对应 Java `WxMaSubscribeServiceImpl.getCategory`。
    async fn get_category(&self) -> Result<Vec<CategoryData>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.get_category().await
    }

    /// 对应 Java `WxMaSubscribeServiceImpl.sendSubscribeMsg`。
    ///
    /// 请求体为 `WxMaSubscribeMessage` 手写 Serialize 输出
    /// （`touser`/`template_id`/`page`/`miniprogram_state`/`lang`/`data` map
    /// 结构）；Java 的显式 errcode 校验已被执行引擎覆盖（同一语义）。
    async fn send_subscribe_msg(
        &self,
        subscribe_message: &WxMaSubscribeMessage,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.send_subscribe_msg(subscribe_message).await
    }

    /// 对应 Java `WxMaSubscribeServiceImpl.setUserNotify`。
    async fn set_user_notify(
        &self,
        request: &WxMaServiceNotifyRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.set_user_notify(request).await
    }

    /// 对应 Java `WxMaSubscribeServiceImpl.setUserNotifyExt`。
    async fn set_user_notify_ext(
        &self,
        request: &WxMaServiceNotifyExtRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.set_user_notify_ext(request).await
    }

    /// 对应 Java `WxMaSubscribeServiceImpl.getUserNotify`。
    async fn get_user_notify(
        &self,
        request: &WxMaGetUserNotifyRequest,
    ) -> Result<WxMaGetUserNotifyResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.get_user_notify(request).await
    }
}
