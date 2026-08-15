//! 留资组件管理服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxLeadComponentServiceImpl`。
//!
//! 注意：微信返回的 `user_data` 与其中 `leads_data` 均为**字符串包裹的非标准
//! JSON**（Java 注释原文），Java 在反序列化前先做 `convertLeadInfoResponse`
//! 解析（`user_data` 每项是 JSON 字符串，内部 `leads_data` 又是 JSON 字符串），
//! Rust 以 `serde_json::Value` 逐层还原后反序列化，语义一致。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_lead_component_service::WxLeadComponentService;
use crate::bean::lead::component::request::{
    GetLeadInfoByComponentRequest, GetLeadsComponentIdRequest,
    GetLeadsComponentPromoteRecordRequest, GetLeadsInfoByRequestIdRequest,
    GetLeadsRequestIdRequest,
};
use crate::bean::lead::component::response::{
    GetLeadsComponentIdResponse, GetLeadsComponentPromoteRecordResponse, GetLeadsRequestIdResponse,
    LeadInfoResponse,
};
use crate::enums::url_lead_component::{
    GET_LEADS_COMPONENT_ID, GET_LEADS_COMPONENT_PROMOTE_RECORD, GET_LEADS_INFO_BY_COMPONENT_ID,
    GET_LEADS_INFO_BY_REQUEST_ID, GET_LEADS_REQUEST_ID,
};

/// 留资组件管理服务实现（对应 Java `WxLeadComponentServiceImpl`）。
pub struct WxLeadComponentServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxLeadComponentServiceImpl {
    /// 构建服务（对应 Java `new WxLeadComponentServiceImpl(shopService)`）。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }

    /// 发送 POST 请求并解析响应（对应 Java `shopService.post` +
    /// `ResponseUtils.decode`；errcode 校验由执行引擎完成，同 Java 语义）。
    async fn post_as<T>(
        svc: &dyn WxChannelService,
        url: &str,
        post_data: &str,
    ) -> Result<T, WxErrorException>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = svc.post(url, post_data).await?;
        serde_json::from_str(&response).map_err(WxErrorException::from)
    }

    /// 还原微信返回的字符串包裹非标准 JSON（对应 Java
    /// `convertLeadInfoResponse`）：
    /// - `user_data` 为数组，每项是 JSON 字符串 → 解析为对象；
    /// - 对象内 `leads_data` 为 JSON 字符串 → 解析为数组；
    ///
    /// 还原后整体反序列化为 [`LeadInfoResponse`]。
    fn convert_lead_info_response(res_json: &str) -> Result<LeadInfoResponse, WxErrorException> {
        let mut root: serde_json::Value =
            serde_json::from_str(res_json).map_err(WxErrorException::from)?;
        let user_data = root
            .get("user_data")
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        let mut converted = Vec::new();
        if let Some(array) = user_data.as_array() {
            for ele in array {
                // 每项是字符串包裹的 JSON（Java `objectMapper.readTree(userDataEle.asText())`）
                let item_str = ele.as_str().unwrap_or("{}");
                let mut item: serde_json::Value =
                    serde_json::from_str(item_str).map_err(WxErrorException::from)?;
                let leads_str = item
                    .get("leads_data")
                    .and_then(|v| v.as_str())
                    .unwrap_or("[]");
                let leads_array: serde_json::Value =
                    serde_json::from_str(leads_str).map_err(WxErrorException::from)?;
                item["leads_data"] = leads_array;
                converted.push(item);
            }
        }
        root["user_data"] = serde_json::Value::Array(converted);
        serde_json::from_value(root).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxLeadComponentService for WxLeadComponentServiceImpl {
    /// 按时间获取留资信息详情（对应 Java `getLeadsInfoByComponentId`；
    /// Java `ObjectUtils.defaultIfNull(version, 1)`：Rust 以 `version == 0`
    /// 表达 null，默认置 1）。
    async fn get_leads_info_by_component_id(
        &self,
        mut req: GetLeadInfoByComponentRequest,
    ) -> Result<LeadInfoResponse, WxErrorException> {
        if req.version == 0 {
            req.version = 1;
        }
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&req).map_err(WxErrorException::from)?;
        let res_json = svc.post(GET_LEADS_INFO_BY_COMPONENT_ID, &req_json).await?;
        Self::convert_lead_info_response(&res_json)
    }

    /// 按直播场次获取留资信息详情（对应 Java `getLeadsInfoByRequestId`；
    /// version 默认 1 同上）。
    async fn get_leads_info_by_request_id(
        &self,
        mut req: GetLeadsInfoByRequestIdRequest,
    ) -> Result<LeadInfoResponse, WxErrorException> {
        if req.version == 0 {
            req.version = 1;
        }
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&req).map_err(WxErrorException::from)?;
        let res_json = svc.post(GET_LEADS_INFO_BY_REQUEST_ID, &req_json).await?;
        Self::convert_lead_info_response(&res_json)
    }

    /// 获取留资 request_id 列表详情（对应 Java `getLeadsRequestId`）。
    async fn get_leads_request_id(
        &self,
        req: GetLeadsRequestIdRequest,
    ) -> Result<GetLeadsRequestIdResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&req).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_LEADS_REQUEST_ID, &req_json).await
    }

    /// 获取留资组件直播推广记录信息详情（对应 Java `getLeadsComponentPromoteRecord`）。
    async fn get_leads_component_promote_record(
        &self,
        req: GetLeadsComponentPromoteRecordRequest,
    ) -> Result<GetLeadsComponentPromoteRecordResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&req).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_LEADS_COMPONENT_PROMOTE_RECORD, &req_json).await
    }

    /// 获取留资组件 Id 列表详情（对应 Java `getLeadsComponentId`）。
    async fn get_leads_component_id(
        &self,
        req: GetLeadsComponentIdRequest,
    ) -> Result<GetLeadsComponentIdResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&req).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_LEADS_COMPONENT_ID, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 按时间获取留资信息详情：version 默认置 1（对应 Java
    /// `ObjectUtils.defaultIfNull(version, 1)`）+ 字符串包裹 JSON 还原解析
    /// （对应 Java `convertLeadInfoResponse`）。
    #[tokio::test]
    async fn test_get_leads_info_by_component_id() {
        // user_data 每项是 JSON 字符串，内部 leads_data 又是 JSON 字符串
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","user_data":["{\"anchor_nickname\":\"主播A\",\"leads_data\":\"[{\\\"title\\\":\\\"手机号\\\",\\\"value\\\":\\\"13800000000\\\"}]\"}"],"last_buffer":"","continue_flag":false}"#,
        );
        let sub = WxLeadComponentServiceImpl::new(weak);
        let req = GetLeadInfoByComponentRequest {
            leads_component_id: "comp_1".to_string(),
            start_time: 1700000000,
            end_time: 1700000100,
            last_buffer: String::new(),
            version: 0,
        };
        let resp = sub.get_leads_info_by_component_id(req).await.unwrap();
        assert_eq!(resp.user_data.len(), 1);
        assert_eq!(resp.user_data[0].anchor_nickname, "主播A");
        assert_eq!(resp.user_data[0].leads_data.len(), 1);
        assert_eq!(resp.user_data[0].leads_data[0].title, "手机号");
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_LEADS_INFO_BY_COMPONENT_ID);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["leads_component_id"], "comp_1");
        assert_eq!(json["version"], 1);
    }

    /// 获取留资组件 Id 列表详情：请求体透传与响应解析（对应 Java
    /// `getLeadsComponentId`）。
    #[tokio::test]
    async fn test_get_leads_component_id() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","item":[{"leads_component_id":"comp_1","status":1}],"last_buffer":""}"#,
        );
        let sub = WxLeadComponentServiceImpl::new(weak);
        let req = GetLeadsComponentIdRequest {
            last_buffer: String::new(),
        };
        let resp = sub.get_leads_component_id(req).await.unwrap();
        assert_eq!(resp.item.len(), 1);
        assert_eq!(resp.item[0].leads_component_id, "comp_1");
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_LEADS_COMPONENT_ID);
        assert_eq!(body, r#"{"last_buffer":""}"#);
    }
}
