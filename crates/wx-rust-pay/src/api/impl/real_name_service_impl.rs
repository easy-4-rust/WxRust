//! 对应 Java `service.impl.RealNameServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::RealNameService;
use crate::api::WxPayService;
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;
use crate::util::wx_pay_service_impl_utils::V2Request;

/// 实名服务实现（对应 Java `RealNameServiceImpl`）。
pub struct RealNameServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl RealNameServiceImpl {
    /// 构建实现（对应 Java 构造器 `RealNameServiceImpl(WxPayService)`）。
    pub fn new(pay_service: Weak<dyn WxPayService>) -> Self {
        Self { pay_service }
    }

    /// 升级门面引用（对应 Java `this.payService` 直接使用）。
    fn svc(&self) -> Result<Arc<dyn WxPayService>, WxErrorException> {
        self.pay_service
            .upgrade()
            .ok_or_else(|| impl_utils::runtime("WxPayService 已释放"))
    }
}

#[async_trait]
impl RealNameService for RealNameServiceImpl {
    async fn query_real_name(
        &self,
        request: &RealNameRequest,
    ) -> Result<RealNameResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/userinfo/realnameauth/query", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type().as_deref(),
            true,
            RealNameResult::from_xml,
        )
    }

    async fn query_real_name_with_openid(
        &self,
        openid: &str,
    ) -> Result<RealNameResult, WxErrorException> {
        let mut request = RealNameRequest::default();
        request.openid = Some(openid.to_string());
        self.query_real_name(&request).await
    }
}
