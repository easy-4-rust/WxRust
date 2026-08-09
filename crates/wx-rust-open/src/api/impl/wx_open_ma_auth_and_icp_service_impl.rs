//! 小程序认证及备案服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMaAuthAndIcpServiceImpl`
//! （持有 `WxMaService`——代 ma 桥接服务，构造入参
//! `new WxOpenMaAuthAndIcpServiceImpl(this)`）。
//!
//! Rust 以 `Weak<dyn WxOpenService>` + `app_id` 表达同一依赖：请求时经
//! 组件服务 `get_wx_ma_service_by_appid(app_id)` 取回代 ma 桥接服务
//! （[`crate::api::r#impl::WxOpenMaService`]）下转后调用，语义与 Java
//! 直接持有的 `WxMaService` 一致（ADAPTED：弱引用 + 按 appid 查找打破
//! 构造环）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;
use wx_rust_miniapp::api::WxMaService;

use crate::api::WxOpenMaAuthAndIcpService;
use crate::api::WxOpenService;
use crate::api::r#impl::WxOpenMaService;
use crate::api::r#impl::base_wx_open_service_impl;
use crate::bean::WxOpenQueryAuthAndIcpResult;
use crate::bean::WxOpenSubmitAuthAndIcpParam;
use crate::bean::WxOpenSubmitAuthAndIcpResult;
use crate::enums::url_ma_domain::{ma_auth_and_icp_query_url, ma_auth_and_icp_submit_url};

/// 小程序认证及备案服务实现（对应 Java `WxOpenMaAuthAndIcpServiceImpl`）。
pub struct WxOpenMaAuthAndIcpServiceImpl {
    /// 门面服务弱引用（对应 Java 强持有 `WxMaService`；Rust 以弱引用 +
    /// 按 appid 查找表达，打破「open service → component service →
    /// 桥接服务 → 子服务」环）。
    wx_open_service: Weak<dyn WxOpenService>,
    /// 授权方 appid（代运营目标账号）。
    app_id: String,
}

impl WxOpenMaAuthAndIcpServiceImpl {
    /// 构建服务（对应 Java
    /// `new WxOpenMaAuthAndIcpServiceImpl(WxMaService)`，其中
    /// `WxMaService` 为代 ma 桥接服务自身）。
    ///
    /// # 参数
    /// - `wx_open_service`：门面服务强引用（内部降级为弱引用）
    /// - `app_id`：授权方 appid
    pub fn new(wx_open_service: Arc<dyn WxOpenService>, app_id: String) -> Self {
        Self {
            wx_open_service: Arc::downgrade(&wx_open_service),
            app_id,
        }
    }

    /// 升级门面服务引用；门面已释放时返回业务错误。
    /// 授权方 appid（Java 构造入参，代运营目标账号）。
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    fn svc(&self) -> Result<Arc<dyn WxOpenService>, WxErrorException> {
        self.wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))
    }

    /// 取代 ma 桥接服务（对应 Java 构造持有的 `WxMaService` 字段）。
    ///
    /// 经组件服务 `get_wx_ma_service_by_appid(app_id)` 取回后按具体类型
    /// `WxOpenMaService` 下转再上转为 `Arc<dyn WxMaService>`（std downcast
    /// 无法直接到 trait 对象，ADAPTED）。
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
impl WxOpenMaAuthAndIcpService for WxOpenMaAuthAndIcpServiceImpl {
    /// 查询小程序认证及备案进度（对应 Java
    /// `queryAuthAndIcp(String procedureId)`：POST `{"procedure_id": ...}`
    /// → `WxOpenQueryAuthAndIcpResult`）。
    async fn query_auth_and_icp(
        &self,
        procedure_id: &str,
    ) -> Result<WxOpenQueryAuthAndIcpResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "procedure_id": procedure_id });
        let response = ma
            .post(
                &ma_auth_and_icp_query_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 提交小程序认证及备案信息（对应 Java
    /// `submitAuthAndIcp(WxOpenSubmitAuthAndIcpParam param)`：POST 参数
    /// 序列化 → `WxOpenSubmitAuthAndIcpResult`）。
    async fn submit_auth_and_icp(
        &self,
        param: &WxOpenSubmitAuthAndIcpParam,
    ) -> Result<WxOpenSubmitAuthAndIcpResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = ma
            .post(&ma_auth_and_icp_submit_url(config.as_ref()), &body)
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
