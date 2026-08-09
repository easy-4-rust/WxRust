//! WxChannelFreightTemplateServiceImpl（对应 Java
//! `me.chanjar.weixin.channel.api.impl.WxChannelFreightTemplateServiceImpl`）。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_freight_template_service::WxChannelFreightTemplateService;
use crate::bean::freight::{
    FreightTemplate, TemplateAddParam, TemplateIdResponse, TemplateInfoResponse,
    TemplateListResponse,
};
use crate::enums::url_freight as url;

/// 构建 JSON 对象（跳过空值，对应 Java Jackson `JsonInclude.Include.NON_NULL`）。
fn build_json(pairs: &[(&str, serde_json::Value)]) -> String {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        if !value.is_null() {
            map.insert((*key).to_string(), value.clone());
        }
    }
    serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}

/// 运费模板服务实现。
pub struct WxChannelFreightTemplateServiceImpl {
    service: Weak<dyn WxChannelService>,
}

impl WxChannelFreightTemplateServiceImpl {
    /// 构建运费模板服务。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxChannelFreightTemplateService for WxChannelFreightTemplateServiceImpl {
    /// 对应 Java `WxChannelFreightTemplateServiceImpl.listTemplate`：
    /// `TemplateListParam`（空值跳过，Java Jackson `NON_NULL`）后 POST
    /// `LIST_TEMPLATE_URL`。
    async fn list_template(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<TemplateListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[
            (
                "offset",
                offset
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "limit",
                limit
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc.post(url::LIST_TEMPLATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelFreightTemplateServiceImpl.getTemplate`：
    /// `{"template_id": ".."}` 后 POST `GET_TEMPLATE_URL`。
    async fn get_template(
        &self,
        template_id: String,
    ) -> Result<TemplateInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let body = build_json(&[("template_id", serde_json::Value::String(template_id))]);
        let response = svc.post(url::GET_TEMPLATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelFreightTemplateServiceImpl.addTemplate`：
    /// 序列化 `TemplateAddParam`（包裹 `FreightTemplate`，key 为
    /// `freight_template`）后 POST `ADD_TEMPLATE_URL`。
    async fn add_template(
        &self,
        template: FreightTemplate,
    ) -> Result<TemplateIdResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = TemplateAddParam { template };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::ADD_TEMPLATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxChannelFreightTemplateServiceImpl.updateTemplate`：
    /// 序列化 `TemplateAddParam` 后 POST `UPDATE_TEMPLATE_URL`。
    async fn update_template(
        &self,
        template: FreightTemplate,
    ) -> Result<TemplateIdResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "视频号小店服务已释放"))?;
        let param = TemplateAddParam { template };
        let body =
            serde_json::to_string(&param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(url::UPDATE_TEMPLATE_URL, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
