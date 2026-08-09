//! 优选联盟达人服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxLeaguePromoterServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_league_promoter_service::WxLeaguePromoterService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::league::promoter::{
    PromoterInfoResponse, PromoterListParam, PromoterListResponse,
};
use crate::enums::url_league::{
    ADD_PROMOTER_URL, DELETE_PROMOTER_URL, EDIT_PROMOTER_URL, GET_PROMOTER_LIST_URL,
    GET_PROMOTER_URL,
};

/// 优选联盟达人服务实现（对应 Java `WxLeaguePromoterServiceImpl`）。
pub struct WxLeaguePromoterServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxLeaguePromoterServiceImpl {
    /// 构建服务（对应 Java `new WxLeaguePromoterServiceImpl(shopService)`）。
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
impl WxLeaguePromoterService for WxLeaguePromoterServiceImpl {
    /// 新增达人（对应 Java `addPromoter`，请求体 `{"finder_id":"..."}`；
    /// Java 已废弃，保留镜像）。
    async fn add_promoter(
        &self,
        finder_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"finder_id\":\"{finder_id}\"}}");
        Self::post_as(svc.as_ref(), ADD_PROMOTER_URL, &req_json).await
    }

    /// 编辑达人（对应 Java `updatePromoter`，请求体
    /// `{"finder_id":"...","type":N}`；`type`：1 取消邀请 / 2 结束合作）。
    async fn update_promoter(
        &self,
        finder_id: String,
        r#type: i32,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"finder_id\":\"{finder_id}\",\"type\":{}}}", r#type);
        Self::post_as(svc.as_ref(), EDIT_PROMOTER_URL, &req_json).await
    }

    /// 删除达人（对应 Java `deletePromoter`，请求体 `{"finder_id":"..."}`）。
    async fn delete_promoter(
        &self,
        finder_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"finder_id\":\"{finder_id}\"}}");
        Self::post_as(svc.as_ref(), DELETE_PROMOTER_URL, &req_json).await
    }

    /// 获取达人详情信息（对应 Java `getPromoterInfo`，请求体 `{"finder_id":"..."}`）。
    async fn get_promoter_info(
        &self,
        finder_id: String,
    ) -> Result<PromoterInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"finder_id\":\"{finder_id}\"}}");
        Self::post_as(svc.as_ref(), GET_PROMOTER_URL, &req_json).await
    }

    /// 新增达人（对应 Java `addPromoterV2`，请求体 `{"promoter_id":"..."}`）。
    async fn add_promoter_v2(
        &self,
        promoter_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"promoter_id\":\"{promoter_id}\"}}");
        Self::post_as(svc.as_ref(), ADD_PROMOTER_URL, &req_json).await
    }

    /// 编辑达人（对应 Java `updatePromoterV2`，请求体
    /// `{"promoter_id":"...","type":N}`）。
    async fn update_promoter_v2(
        &self,
        promoter_id: String,
        r#type: i32,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"promoter_id\":\"{promoter_id}\",\"type\":{}}}", r#type);
        Self::post_as(svc.as_ref(), EDIT_PROMOTER_URL, &req_json).await
    }

    /// 删除达人（对应 Java `deletePromoterV2`，请求体 `{"promoter_id":"..."}`）。
    async fn delete_promoter_v2(
        &self,
        promoter_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"promoter_id\":\"{promoter_id}\"}}");
        Self::post_as(svc.as_ref(), DELETE_PROMOTER_URL, &req_json).await
    }

    /// 获取达人详情信息（对应 Java `getPromoterInfoV2`，请求体 `{"promoter_id":"..."}`）。
    async fn get_promoter_info_v2(
        &self,
        promoter_id: String,
    ) -> Result<PromoterInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"promoter_id\":\"{promoter_id}\"}}");
        Self::post_as(svc.as_ref(), GET_PROMOTER_URL, &req_json).await
    }

    /// 获取达人列表（对应 Java `listPromoter(Integer, Integer, Integer)`，内部构造
    /// `PromoterListParam`，请求体 `{"page_index":N,"page_size":N,"status":N}`）。
    async fn list_promoter(
        &self,
        page_index: Option<i32>,
        page_size: Option<i32>,
        status: Option<i32>,
    ) -> Result<PromoterListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = PromoterListParam {
            page_index: page_index.unwrap_or(1),
            page_size: page_size.unwrap_or(0),
            status: status.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_PROMOTER_LIST_URL, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 编辑达人：字面量请求体 `{"finder_id":"...","type":N}` 逐字对齐 Java
    /// `updatePromoter`。
    #[tokio::test]
    async fn test_update_promoter() {
        let (svc, weak) = test_support::build_service(r#"{"errcode":0,"errmsg":"ok"}"#);
        let sub = WxLeaguePromoterServiceImpl::new(weak);
        let resp = sub
            .update_promoter("sph_finder_1".to_string(), 2)
            .await
            .unwrap();
        assert_eq!(resp.err_code, 0);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, EDIT_PROMOTER_URL);
        assert_eq!(body, r#"{"finder_id":"sph_finder_1","type":2}"#);
    }

    /// 获取达人列表：`PromoterListParam` 请求体与响应解析（对应 Java
    /// `listPromoter` + `PromoterListParam`；`page_index` 默认 1 对齐 Java
    /// 参数默认值）。
    #[tokio::test]
    async fn test_list_promoter() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","finder_ids":["f1","f2"],"total_num":2,"continue_flag":false}"#,
        );
        let sub = WxLeaguePromoterServiceImpl::new(weak);
        let resp = sub.list_promoter(None, Some(10), Some(1)).await.unwrap();
        assert_eq!(resp.finder_ids, vec!["f1".to_string(), "f2".to_string()]);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_PROMOTER_LIST_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["page_index"], 1);
        assert_eq!(json["page_size"], 10);
        assert_eq!(json["status"], 1);
    }

    /// V2 达人详情：请求体字段 `promoter_id`（对应 Java `getPromoterInfoV2`）。
    #[tokio::test]
    async fn test_get_promoter_info_v2() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","promoter":{"finder_id":"sph_f","status":1}}"#,
        );
        let sub = WxLeaguePromoterServiceImpl::new(weak);
        let resp = sub.get_promoter_info_v2("p1".to_string()).await.unwrap();
        assert_eq!(resp.promoter.finder_id, "sph_f");
        assert_eq!(resp.promoter.status, 1);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_PROMOTER_URL);
        assert_eq!(body, r#"{"promoter_id":"p1"}"#);
    }
}
