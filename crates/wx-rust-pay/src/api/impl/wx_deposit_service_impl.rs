//! 对应 Java `service.impl.WxDepositServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{WxDepositService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;
use crate::util::wx_pay_service_impl_utils::V2Request;

/// WxDepositService 实现（对应 Java `WxDepositServiceImpl`）。
pub struct WxDepositServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl WxDepositServiceImpl {
    /// 构建实现（对应 Java 构造器 `WxDepositServiceImpl(WxPayService)`）。
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
impl WxDepositService for WxDepositServiceImpl {
    async fn unified_order(
        &self,
        request: &WxDepositUnifiedOrderRequest,
    ) -> Result<WxDepositUnifiedOrderResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/pay/depositpay", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, false)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            WxDepositUnifiedOrderResult::from_xml,
        )
    }

    async fn query_order(
        &self,
        request: &WxDepositOrderQueryRequest,
    ) -> Result<WxDepositOrderQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/pay/depositorderquery", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, false)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            WxDepositOrderQueryResult::from_xml,
        )
    }

    async fn consume(
        &self,
        request: &WxDepositConsumeRequest,
    ) -> Result<WxDepositConsumeResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/pay/depositconsume", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, false)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            WxDepositConsumeResult::from_xml,
        )
    }

    async fn unfreeze(
        &self,
        request: &WxDepositUnfreezeRequest,
    ) -> Result<WxDepositUnfreezeResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/pay/depositreverse", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, false)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            WxDepositUnfreezeResult::from_xml,
        )
    }

    async fn refund(
        &self,
        request: &WxDepositRefundRequest,
    ) -> Result<WxDepositRefundResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/pay/depositrefund", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            WxDepositRefundResult::from_xml,
        )
    }
}
