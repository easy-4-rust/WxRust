//! 对应 Java `service.impl.MarketingBusiFavorServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{MarketingBusiFavorService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// MarketingBusiFavorService 实现（对应 Java `MarketingBusiFavorServiceImpl`）。
pub struct MarketingBusiFavorServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl MarketingBusiFavorServiceImpl {
    /// 构建实现（对应 Java 构造器 `MarketingBusiFavorServiceImpl(WxPayService)`）。
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
impl MarketingBusiFavorService for MarketingBusiFavorServiceImpl {
    async fn create_busi_favor_stocks_v3(
        &self,
        request: &BusiFavorStocksCreateRequest,
    ) -> Result<BusiFavorStocksCreateResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!("{}/v3/marketing/busifavor/stocks", svc.get_pay_base_url());
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_busi_favor_stocks_v3(
        &self,
        stock_id: &str,
    ) -> Result<BusiFavorStocksGetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/stocks/{stock_id}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn verify_busi_favor_coupons_use_v3(
        &self,
        request: &BusiFavorCouponsUseRequest,
    ) -> Result<BusiFavorCouponsUseResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/coupons/use",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn build_busi_favor_couponinfo_url(
        &self,
        request: &BusiFavorCouponsUrlRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        // 对应 Java：HMAC-SHA256 签名（mchKey）
        let mut map = std::collections::HashMap::new();
        map.insert(
            "out_request_no".to_string(),
            request.out_request_no.clone().unwrap_or_default(),
        );
        map.insert(
            "stock_id".to_string(),
            request.stock_id.clone().unwrap_or_default(),
        );
        map.insert(
            "send_coupon_merchant".to_string(),
            request.send_coupon_merchant.clone().unwrap_or_default(),
        );
        map.insert(
            "open_id".to_string(),
            request.openid.clone().unwrap_or_default(),
        );
        let sign = crate::util::sign_utils::SignUtils::create_sign(
            &map,
            Some(crate::constant::wx_pay_constants::sign_type::HMAC_SHA256),
            config.mch_key().unwrap_or_default(),
            &[],
        )?;
        Ok(format!(
            "https://action.weixin.qq.com/busifavor/getcouponinfo?stock_id={}&out_request_no={}&sign={}&send_coupon_merchant={}&open_id={}#wechat_redirect",
            request.stock_id.as_deref().unwrap_or_default(),
            request.out_request_no.as_deref().unwrap_or_default(),
            sign,
            request.send_coupon_merchant.as_deref().unwrap_or_default(),
            request.openid.as_deref().unwrap_or_default()
        ))
    }

    async fn query_busi_favor_users_coupons(
        &self,
        request: &BusiFavorQueryUserCouponsRequest,
    ) -> Result<BusiFavorQueryUserCouponsResult, WxErrorException> {
        let svc = self.svc()?;
        let mut request = request.clone();
        // 对应 Java：offset/limit 默认 0/20
        if request.offset.is_none() {
            request.offset = Some(0);
        }
        if request.limit.is_none() || request.limit.unwrap_or_default() <= 0 {
            request.limit = Some(20);
        }
        let mut query = format!(
            "?appid={}&offset={}&limit={}",
            request.appid.as_deref().unwrap_or_default(),
            request.offset.unwrap_or_default(),
            request.limit.unwrap_or_default()
        );
        if let Some(v) = request.stock_id.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&stock_id={v}"));
            }
        }
        if let Some(v) = request.coupon_state.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&coupon_state={v}"));
            }
        }
        if let Some(v) = request.creator_merchant.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&creator_merchant={v}"));
            }
        }
        if let Some(v) = request.belong_merchant.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&belong_merchant={v}"));
            }
        }
        if let Some(v) = request.sender_merchant.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&sender_merchant={v}"));
            }
        }
        let url = format!(
            "{}/v3/marketing/busifavor/users/{}/coupons{query}",
            svc.get_pay_base_url(),
            request.openid.as_deref().unwrap_or_default()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_one_busi_favor_users_coupons(
        &self,
        request: &BusiFavorQueryOneUserCouponsRequest,
    ) -> Result<BusiFavorQueryOneUserCouponsResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/users/{}/coupons/{}/appids/{}",
            svc.get_pay_base_url(),
            request.openid.as_deref().unwrap_or_default(),
            request.coupon_code.as_deref().unwrap_or_default(),
            request.appid.as_deref().unwrap_or_default()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn upload_busi_favor_coupon_codes(
        &self,
        stock_id: &str,
        request: &BusiFavorCouponCodeRequest,
    ) -> Result<BusiFavorCouponCodeResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/stocks/{stock_id}/couponcodes",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn create_busi_favor_callbacks(
        &self,
        request: &BusiFavorCallbacksRequest,
    ) -> Result<BusiFavorCallbacksResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/callbacks",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_busi_favor_callbacks(
        &self,
        request: &BusiFavorCallbacksRequest,
    ) -> Result<BusiFavorCallbacksResult, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}/v3/marketing/busifavor/callbacks",
            svc.get_pay_base_url()
        );
        if let Some(v) = request.mchid.as_deref() {
            if !v.trim().is_empty() {
                url.push_str(&format!("?mchid={v}"));
            }
        }
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_busi_favor_coupons_associate(
        &self,
        request: &BusiFavorCouponsAssociateRequest,
    ) -> Result<BusiFavorCouponsAssociateResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/coupons/associate",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_busi_favor_coupons_dis_associate(
        &self,
        request: &BusiFavorCouponsAssociateRequest,
    ) -> Result<BusiFavorCouponsAssociateResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/coupons/disassociate",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn update_busi_favor_stocks_budget(
        &self,
        stock_id: &str,
        request: &BusiFavorStocksBudgetRequest,
    ) -> Result<BusiFavorStocksBudgetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/stocks/{stock_id}/budget",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.patch_v3(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn update_busi_favor_stocks_v3(
        &self,
        stock_id: &str,
        request: &BusiFavorStocksCreateRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/stocks/{stock_id}",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.patch_v3(&url, &body).await
    }

    async fn return_busi_favor_coupons(
        &self,
        request: &BusiFavorCouponsReturnRequest,
    ) -> Result<BusiFavorCouponsReturnResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/coupons/return",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn deactive_busi_favor_coupons(
        &self,
        request: &BusiFavorCouponsDeactivateRequest,
    ) -> Result<BusiFavorCouponsDeactivateResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/coupons/deactivate",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn subsidy_busi_favor_pay_receipts(
        &self,
        request: &BusiFavorSubsidyRequest,
    ) -> Result<BusiFavorSubsidyResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/subsidy/pay-receipts",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_busi_favor_subsidy_pay_receipts(
        &self,
        subsidy_receipt_id: &str,
    ) -> Result<BusiFavorSubsidyResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/busifavor/subsidy/pay-receipts/{subsidy_receipt_id}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn notify_busi_favor(
        &self,
        url: &str,
        request: &BusiFavorNotifyRequest,
    ) -> Result<BusiFavorNotifyResult, WxErrorException> {
        let svc = self.svc()?;
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
