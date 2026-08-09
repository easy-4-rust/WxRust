//! 留资服务直播数据服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxFinderLiveServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_finder_live_service::WxFinderLiveService;
use crate::bean::lead::component::request::{
    GetFinderLiveDataListRequest, GetFinderLiveLeadsDataRequest,
};
use crate::bean::lead::component::response::{
    FinderAttrResponse, GetFinderLiveDataListResponse, GetFinderLiveLeadsDataResponse,
};
use crate::enums::url_finder_live::{
    GET_FINDER_ATTR_BY_APPID, GET_FINDER_LIVE_DATA_LIST, GET_FINDER_LIVE_LEADS_DATA,
};

/// 留资服务直播数据服务实现（对应 Java `WxFinderLiveServiceImpl`）。
pub struct WxFinderLiveServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxFinderLiveServiceImpl {
    /// 构建服务（对应 Java `new WxFinderLiveServiceImpl(shopService)`）。
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
}

#[async_trait]
impl WxFinderLiveService for WxFinderLiveServiceImpl {
    /// 获取视频号账号信息（对应 Java `getFinderAttrByAppid`，POST 空对象 `{}`）。
    async fn get_finder_attr_by_appid(&self) -> Result<FinderAttrResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        Self::post_as(svc.as_ref(), GET_FINDER_ATTR_BY_APPID, "{}").await
    }

    /// 获取留资直播间数据详情（对应 Java `getFinderLiveDataList(GetFinderLiveDataListRequest)`）。
    async fn get_finder_live_data_list(
        &self,
        req: GetFinderLiveDataListRequest,
    ) -> Result<GetFinderLiveDataListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&req).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_FINDER_LIVE_DATA_LIST, &req_json).await
    }

    /// 获取账号收集的留资数量（对应 Java `getFinderLiveLeadsData`；
    /// 该接口只统计 2023.9.13 起的数据，start_time 应大于等于 1694534400）。
    async fn get_finder_live_leads_data(
        &self,
        req: GetFinderLiveLeadsDataRequest,
    ) -> Result<GetFinderLiveLeadsDataResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&req).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_FINDER_LIVE_LEADS_DATA, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 获取视频号账号信息：POST 空对象 `{}` 与响应解析（对应 Java
    /// `getFinderAttrByAppid`）。
    #[tokio::test]
    async fn test_get_finder_attr_by_appid() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","finder_attr":{"uniq_id":"sph_x","nickname":"视频号"}}"#,
        );
        let sub = WxFinderLiveServiceImpl::new(weak);
        let resp = sub.get_finder_attr_by_appid().await.unwrap();
        assert_eq!(resp.finder_attr.nickname, "视频号");
        assert_eq!(resp.finder_attr.uniq_id, "sph_x");
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_FINDER_ATTR_BY_APPID);
        assert_eq!(body, "{}");
    }

    /// 获取留资直播间数据详情：请求体透传与响应解析（对应 Java
    /// `getFinderLiveDataList`）。
    #[tokio::test]
    async fn test_get_finder_live_data_list() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","item":[{"export_id":"exp_1","live_start_time":1694534400}],"continue_flag":false}"#,
        );
        let sub = WxFinderLiveServiceImpl::new(weak);
        let req = GetFinderLiveDataListRequest {
            start_time: 1694534400,
            end_time: 1694538000,
            last_buffer: String::new(),
        };
        let resp = sub.get_finder_live_data_list(req).await.unwrap();
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].export_id, "exp_1");
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_FINDER_LIVE_DATA_LIST);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["start_time"], 1694534400);
        assert_eq!(json["end_time"], 1694538000);
    }
}
