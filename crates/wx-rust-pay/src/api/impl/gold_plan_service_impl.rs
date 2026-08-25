//! 点金计划服务实现。
//!
//! 对应 Java `com.github.binarywang.wxpay.service.impl.GoldPlanServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use serde_json::json;
use wx_rust_common::error::WxErrorException;

use crate::api::{GoldPlanService, WxPayService};
use crate::bean::goldplan::gold_plan_result::GoldPlanResult;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// 操作类型常量（对应 Java `GoldPlanServiceImpl` 的 `OPEN`/`CLOSE`）。
const OPEN: &str = "OPEN";
const CLOSE: &str = "CLOSE";

/// GoldPlanService 实现（对应 Java `GoldPlanServiceImpl`）。
pub struct GoldPlanServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl GoldPlanServiceImpl {
    /// 构建实现（对应 Java 构造器 `GoldPlanServiceImpl(WxPayService)`）。
    pub fn new(pay_service: Weak<dyn WxPayService>) -> Self {
        Self { pay_service }
    }

    /// 升级门面引用（对应 Java `this.payService` 直接使用）。
    fn svc(&self) -> Result<Arc<dyn WxPayService>, WxErrorException> {
        self.pay_service
            .upgrade()
            .ok_or_else(|| impl_utils::runtime("WxPayService 已释放"))
    }

    /// 调用微信支付接口开通或关闭点金计划。
    ///
    /// 对应 Java: `GoldPlanServiceImpl#changeGoldPlanStatus`
    async fn change_gold_plan_status(
        &self,
        sub_mch_id: &str,
        operation_type: &str,
        operation_pay_scene: Option<&str>,
    ) -> Result<GoldPlanResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/goldplan/merchants/changegoldplanstatus",
            svc.get_pay_base_url()
        );
        let mut request = json!({
            "sub_mchid": sub_mch_id,
            "operation_type": operation_type,
        });
        if let Some(scene) = operation_pay_scene {
            request["operation_pay_scene"] = json!(scene);
        }
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    /// 调用微信支付接口开通或关闭商家小票。
    ///
    /// 对应 Java: `GoldPlanServiceImpl#changeCustomPageStatus`
    async fn change_custom_page_status(
        &self,
        sub_mch_id: &str,
        operation_type: &str,
    ) -> Result<GoldPlanResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/goldplan/merchants/changecustompagestatus",
            svc.get_pay_base_url()
        );
        let request = json!({
            "sub_mchid": sub_mch_id,
            "operation_type": operation_type,
        });
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}

#[async_trait]
impl GoldPlanService for GoldPlanServiceImpl {
    async fn open_gold_plan(
        &self,
        sub_mch_id: &str,
        operation_pay_scene: Option<&str>,
    ) -> Result<GoldPlanResult, WxErrorException> {
        self.change_gold_plan_status(sub_mch_id, OPEN, operation_pay_scene)
            .await
    }

    async fn close_gold_plan(
        &self,
        sub_mch_id: &str,
        operation_pay_scene: Option<&str>,
    ) -> Result<GoldPlanResult, WxErrorException> {
        self.change_gold_plan_status(sub_mch_id, CLOSE, operation_pay_scene)
            .await
    }

    async fn open_custom_page(&self, sub_mch_id: &str) -> Result<GoldPlanResult, WxErrorException> {
        self.change_custom_page_status(sub_mch_id, OPEN).await
    }

    async fn close_custom_page(
        &self,
        sub_mch_id: &str,
    ) -> Result<GoldPlanResult, WxErrorException> {
        self.change_custom_page_status(sub_mch_id, CLOSE).await
    }

    async fn set_advertising_industry_filter(
        &self,
        sub_mch_id: &str,
        advertising_industry_filters: &[String],
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/goldplan/merchants/set-advertising-industry-filter",
            svc.get_pay_base_url()
        );
        let request = json!({
            "sub_mchid": sub_mch_id,
            "advertising_industry_filters": advertising_industry_filters,
        });
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3(&url, &body).await?;
        Ok(())
    }

    async fn open_advertising_show(
        &self,
        sub_mch_id: &str,
        advertising_industry_filters: Option<&[String]>,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/goldplan/merchants/open-advertising-show",
            svc.get_pay_base_url()
        );
        let mut request = json!({
            "sub_mchid": sub_mch_id,
        });
        if let Some(filters) = advertising_industry_filters {
            request["advertising_industry_filters"] = json!(filters);
        }
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.patch_v3(&url, &body).await?;
        Ok(())
    }

    async fn close_advertising_show(&self, sub_mch_id: &str) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/goldplan/merchants/close-advertising-show",
            svc.get_pay_base_url()
        );
        let request = json!({
            "sub_mchid": sub_mch_id,
        });
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3(&url, &body).await?;
        Ok(())
    }
}
