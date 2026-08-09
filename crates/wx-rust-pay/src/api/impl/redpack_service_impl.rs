//! 对应 Java `service.impl.RedpackServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::RedpackService;
use crate::api::WxPayService;
use crate::bean::*;
use crate::constant::wx_pay_constants::bill_type as bill_type_const;
use crate::util::wx_pay_service_impl_utils as impl_utils;
use crate::util::wx_pay_service_impl_utils::V2Request;

/// 红包服务实现（对应 Java `RedpackServiceImpl`）。
pub struct RedpackServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl RedpackServiceImpl {
    /// 构建实现（对应 Java 构造器 `RedpackServiceImpl(WxPayService)`）。
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
impl RedpackService for RedpackServiceImpl {
    async fn send_mini_program_redpack(
        &self,
        request: &WxPaySendMiniProgramRedpackRequest,
    ) -> Result<WxPaySendMiniProgramRedpackResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!(
            "{}/mmpaymkttransfers/sendminiprogramhb",
            svc.get_pay_base_url()
        );
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type().as_deref(),
            true,
            WxPaySendMiniProgramRedpackResult::from_xml,
        )
    }

    async fn send_redpack(
        &self,
        request: &WxPaySendRedpackRequest,
    ) -> Result<WxPaySendRedpackResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        // 对应 Java：amt_type 非空时走群红包接口 sendgroupredpack
        let url = if request.amt_type.is_some() {
            format!(
                "{}/mmpaymkttransfers/sendgroupredpack",
                svc.get_pay_base_url()
            )
        } else {
            format!("{}/mmpaymkttransfers/sendredpack", svc.get_pay_base_url())
        };
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type().as_deref(),
            true,
            WxPaySendRedpackResult::from_xml,
        )
    }

    async fn query_redpack(
        &self,
        mch_bill_no: &str,
    ) -> Result<WxPayRedpackQueryResult, WxErrorException> {
        let mut request = WxPayRedpackQueryRequest::default();
        request.mch_bill_no = Some(mch_bill_no.to_string());
        self.query_redpack_with_request(&request).await
    }

    async fn query_redpack_with_request(
        &self,
        request: &WxPayRedpackQueryRequest,
    ) -> Result<WxPayRedpackQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java：bill_type 固定为 MCHT
        request.bill_type = Some(bill_type_const::MCHT.to_string());
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/mmpaymkttransfers/gethbinfo", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type().as_deref(),
            true,
            WxPayRedpackQueryResult::from_xml,
        )
    }
}
