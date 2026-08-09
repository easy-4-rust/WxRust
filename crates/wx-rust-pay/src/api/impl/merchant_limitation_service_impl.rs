//! 对应 Java `service.impl.MerchantLimitationServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::MerchantLimitationService;
use crate::api::WxPayService;
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// 商户被管控能力及原因查询服务实现（对应 Java `MerchantLimitationServiceImpl`）。
pub struct MerchantLimitationServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl MerchantLimitationServiceImpl {
    /// 构建实现（对应 Java 构造器 `MerchantLimitationServiceImpl(WxPayService)`）。
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
impl MerchantLimitationService for MerchantLimitationServiceImpl {
    async fn fetch_limitations(
        &self,
        sub_mch_id: &str,
    ) -> Result<MerchantLimitationResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/mch-operation-manage/merchant-limitations/sub-mchid/{}",
            svc.get_pay_base_url(),
            sub_mch_id
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
