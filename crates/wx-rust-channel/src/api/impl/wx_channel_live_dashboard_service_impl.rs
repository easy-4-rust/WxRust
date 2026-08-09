//! 直播大屏数据服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxChannelLiveDashboardServiceImpl`。
//!
//! 注意：微信接口获取直播数据中存在**字符串包裹的非标准 JSON**（Java 注释原文），
//! Java 在反序列化前对 `live_dashboard_data`/`live_comparison_index`/
//! `live_ec_data_summary`/`live_ec_conversion_metric`/`live_ec_profile`/
//! `live_distribution_channel`/`single_live_ec_spu_data_page_v2` 七键做
//! `convertLiveDataResponse` 还原（字符串 → JSON），Rust 以
//! `serde_json::Value` 逐键还原后反序列化，语义一致。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_live_dashboard_service::WxChannelLiveDashboardService;
use crate::bean::live::dashboard::{
    LiveDataParam, LiveDataResponse, LiveListParam, LiveListResponse,
};
use crate::enums::url_live_dashboard::{GET_LIVE_DATA_URL, GET_LIVE_LIST_URL};

/// 直播大屏数据服务实现（对应 Java `WxChannelLiveDashboardServiceImpl`）。
pub struct WxChannelLiveDashboardServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxChannelLiveDashboardServiceImpl {
    /// 构建服务（对应 Java `new WxChannelLiveDashboardServiceImpl(shopService)`）。
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

    /// 还原直播数据中的字符串包裹 JSON（对应 Java `convertLiveDataResponse`）：
    /// 对七个数据键，若存在则将字符串值解析为 JSON 后回填，再整体反序列化。
    fn convert_live_data_response(res_json: &str) -> Result<LiveDataResponse, WxErrorException> {
        const DATA_KEYS: [&str; 7] = [
            "live_dashboard_data",
            "live_comparison_index",
            "live_ec_data_summary",
            "live_ec_conversion_metric",
            "live_ec_profile",
            "live_distribution_channel",
            "single_live_ec_spu_data_page_v2",
        ];
        let mut root: serde_json::Value =
            serde_json::from_str(res_json).map_err(WxErrorException::from)?;
        for key in DATA_KEYS {
            let node = root.get(key);
            if let Some(node) = node {
                if let Some(s) = node.as_str() {
                    let data: serde_json::Value =
                        serde_json::from_str(s).map_err(WxErrorException::from)?;
                    root[key] = data;
                }
            }
        }
        serde_json::from_value(root).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxChannelLiveDashboardService for WxChannelLiveDashboardServiceImpl {
    /// 获取直播大屏直播列表（对应 Java `getLiveList(Long)`，内部构造
    /// `LiveListParam`，请求体 `{"ds":N}`）。
    async fn get_live_list(&self, ds: Option<i64>) -> Result<LiveListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = LiveListParam {
            ds: ds.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_LIVE_LIST_URL, &req_json).await
    }

    /// 获取直播大屏数据（对应 Java `getLiveData(String)`，内部构造
    /// `LiveDataParam`，请求体 `{"export_id":"..."}` + 字符串包裹 JSON 还原）。
    async fn get_live_data(&self, export_id: String) -> Result<LiveDataResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = LiveDataParam { export_id };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        let res_json = svc.post(GET_LIVE_DATA_URL, &req_json).await?;
        Self::convert_live_data_response(&res_json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 获取直播大屏直播列表：请求体 `{"ds":N}` 与响应解析（对应 Java
    /// `getLiveList` + `LiveListParam`）。
    #[tokio::test]
    async fn test_get_live_list() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","live_items":[{"export_id":"exp_1"}],"has_more":false}"#,
        );
        let sub = WxChannelLiveDashboardServiceImpl::new(weak);
        let resp = sub.get_live_list(Some(20240101)).await.unwrap();
        assert_eq!(resp.live_items.len(), 1);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_LIVE_LIST_URL);
        assert_eq!(body, r#"{"ds":20240101}"#);
    }

    /// 获取直播大屏数据：七个数据键的字符串包裹 JSON 还原解析（对应 Java
    /// `convertLiveDataResponse`），以 `live_dashboard_data` 为例。
    #[tokio::test]
    async fn test_get_live_data_convert() {
        // live_dashboard_data 为字符串包裹的 JSON（微信真实返回形态）
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","live_dashboard_data":"{\"live_duration\":3600,\"start_time\":1700000000}","live_comparison_index":"{}"}"#,
        );
        let sub = WxChannelLiveDashboardServiceImpl::new(weak);
        let resp = sub.get_live_data("exp_1".to_string()).await.unwrap();
        assert_eq!(resp.live_dashboard_data.live_duration, 3600);
        assert_eq!(resp.live_dashboard_data.start_time, 1700000000);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_LIVE_DATA_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["export_id"], "exp_1");
    }
}
