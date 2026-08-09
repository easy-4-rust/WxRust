//! 小程序认证（年审）服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMaAuthServiceImpl`
//! （持有 `WxMaService`——代 ma 桥接服务，构造入参
//! `new WxOpenMaAuthServiceImpl(this)`）。
//!
//! 依赖表达与 [`crate::api::r#impl::WxOpenMaAuthAndIcpServiceImpl`] 相同
//! （`Weak<dyn WxOpenService>` + 按 appid 取回代 ma 桥接服务，ADAPTED）。
//! `upload` 对应 Java `wxMaService.upload(url, new CommonUploadParam(
//! "media", data))`：Rust 以
//! [`MediaUploadRequestExecutor`] + 代 ma 执行引擎（`execute_with_retry`，
//! authorizer access_token 注入 + 自动刷新）承载。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::MediaUploadRequestExecutor;
use wx_rust_miniapp::api::WxMaService;

use crate::api::WxOpenMaAuthService;
use crate::api::WxOpenService;
use crate::api::r#impl::WxOpenMaService;
use crate::api::r#impl::base_wx_open_service_impl;
use crate::bean::MaAuthQueryIdentityTreeResult;
use crate::bean::MaAuthQueryResult;
use crate::bean::MaAuthResubmitParam;
use crate::bean::MaAuthSubmitParam;
use crate::bean::MaAuthSubmitResult;
use crate::bean::MaAuthUploadResult;
use crate::enums::url_ma_domain::{
    ma_auth_identity_url, ma_auth_query_url, ma_auth_resubmit_url, ma_auth_submit_url,
    ma_auth_upload_url,
};

/// 小程序认证服务实现（对应 Java `WxOpenMaAuthServiceImpl`）。
pub struct WxOpenMaAuthServiceImpl {
    wx_open_service: Weak<dyn WxOpenService>,
    app_id: String,
}

impl WxOpenMaAuthServiceImpl {
    /// 构建服务（对应 Java
    /// `new WxOpenMaAuthServiceImpl(WxMaService)`，`WxMaService` 为代 ma
    /// 桥接服务自身）。
    pub fn new(wx_open_service: Arc<dyn WxOpenService>, app_id: String) -> Self {
        Self {
            wx_open_service: Arc::downgrade(&wx_open_service),
            app_id,
        }
    }

    /// 授权方 appid（Java 构造入参，代运营目标账号）。
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    fn svc(&self) -> Result<Arc<dyn WxOpenService>, WxErrorException> {
        self.wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))
    }

    /// 取代 ma 桥接服务（同
    /// [`WxOpenMaAuthAndIcpServiceImpl::ma_service`]）。
    fn ma_service(&self) -> Result<Arc<dyn WxMaService>, WxErrorException> {
        let svc = self.svc()?;
        let component = svc.wx_open_component_service().ok_or_else(|| {
            WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )
        })?;
        let any = component
            .get_wx_ma_service_by_appid(&self.app_id)
            .ok_or_else(|| WxErrorException::from_code(-99, "getWxMaServiceByAppid 返回 None"))?;
        let ma = any.downcast::<WxOpenMaService>().map_err(|_| {
            WxErrorException::from_code(-99, "代 ma 服务 downcast 失败（缓存类型不匹配）")
        })?;
        Ok(ma as Arc<dyn WxMaService>)
    }
}

#[async_trait]
impl WxOpenMaAuthService for WxOpenMaAuthServiceImpl {
    /// 小程序认证（提审）（对应 Java `submit(MaAuthSubmitParam param)`：
    /// POST 参数序列化 → `MaAuthSubmitResult`）。
    async fn submit(
        &self,
        param: &MaAuthSubmitParam,
    ) -> Result<MaAuthSubmitResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = ma.post(&ma_auth_submit_url(config.as_ref()), &body).await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 进度查询（对应 Java `query(String taskId)`：POST
    /// `{"taskid": ...}` → `MaAuthQueryResult`）。
    async fn query(&self, task_id: &str) -> Result<MaAuthQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "taskid": task_id });
        let response = ma
            .post(&ma_auth_query_url(config.as_ref()), &body.to_string())
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 上传补充材料（对应 Java `upload(CommonUploadData data)`：
    /// multipart 字段 `media` 上传 → `MaAuthUploadResult`）。
    async fn upload(
        &self,
        data: &CommonUploadData,
    ) -> Result<MaAuthUploadResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let executor = MediaUploadRequestExecutor::new(ma.http_client().clone());
        let param = CommonUploadParam::new("media", data.clone());
        let response = wx_rust_miniapp::api::r#impl::base_wx_ma_service_impl::execute_with_retry(
            ma.as_ref(),
            &executor,
            &ma_auth_upload_url(config.as_ref()),
            param,
        )
        .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 重新提审（对应 Java `resubmit(MaAuthResubmitParam param)`：POST
    /// 参数序列化 → `MaAuthSubmitResult`）。
    async fn resubmit(
        &self,
        param: &MaAuthResubmitParam,
    ) -> Result<MaAuthSubmitResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = ma
            .post(&ma_auth_resubmit_url(config.as_ref()), &body)
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 查询个人认证身份选项列表（对应 Java `queryIdentityTree()`：
    /// GET `null` 参数 → `MaAuthQueryIdentityTreeResult`）。
    async fn query_identity_tree(&self) -> Result<MaAuthQueryIdentityTreeResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma.get(&ma_auth_identity_url(config.as_ref()), "").await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
