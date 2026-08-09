//! 对应 Java `service.impl.MarketingFavorServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{MarketingFavorService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// MarketingFavorService 实现（对应 Java `MarketingFavorServiceImpl`）。
pub struct MarketingFavorServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl MarketingFavorServiceImpl {
    /// 构建实现（对应 Java 构造器 `MarketingFavorServiceImpl(WxPayService)`）。
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
impl MarketingFavorService for MarketingFavorServiceImpl {
    async fn create_favor_stocks_v3(
        &self,
        request: &FavorStocksCreateRequest,
    ) -> Result<FavorStocksCreateResult, WxErrorException> {
        let svc = self.svc()?;
        // 对应 Java `RsaCryptoUtil.encryptFields`：FavorStocksCreateRequest 无
        // @SpecEncrypt 字段（Java 反射遍历无操作）
        let url = format!(
            "{}/v3/marketing/favor/coupon-stocks",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        let mut res: FavorStocksCreateResult =
            serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))?;
        res.raw_json_string = Some(result);
        Ok(res)
    }

    async fn create_favor_coupons_v3(
        &self,
        openid: &str,
        request: &FavorCouponsCreateRequest,
    ) -> Result<FavorCouponsCreateResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/favor/users/{openid}/coupons",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn start_favor_stocks_v3(
        &self,
        stock_id: &str,
        request: &FavorStocksSetRequest,
    ) -> Result<FavorStocksStartResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/favor/stocks/{stock_id}/start",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_favor_stocks_v3(
        &self,
        request: &FavorStocksQueryRequest,
    ) -> Result<FavorStocksQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let mut query = format!(
            "?offset={}&limit={}&stock_creator_mchid={}",
            request.offset.unwrap_or_default(),
            request.limit.unwrap_or_default(),
            request.stock_creator_mchid.as_deref().unwrap_or_default()
        );
        if let Some(v) = request.create_start_time.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&create_start_time={v}"));
            }
        }
        if let Some(v) = request.create_end_time.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&create_end_time={v}"));
            }
        }
        if let Some(v) = request.status.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&status={v}"));
            }
        }
        let url = format!(
            "{}/v3/marketing/favor/stocks{query}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_favor_stocks_v3(
        &self,
        stock_id: &str,
        stock_creator_mchid: &str,
    ) -> Result<FavorStocksGetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/favor/stocks/{stock_id}?stock_creator_mchid={stock_creator_mchid}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        let mut res: FavorStocksGetResult =
            serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))?;
        res.raw_json_string = Some(result);
        Ok(res)
    }

    async fn get_favor_coupons_v3(
        &self,
        coupon_id: &str,
        appid: &str,
        openid: &str,
    ) -> Result<FavorCouponsGetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/favor/users/{openid}/coupons/{coupon_id}?appid={appid}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        let mut res: FavorCouponsGetResult =
            serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))?;
        res.raw_json_string = Some(result);
        Ok(res)
    }

    async fn get_favor_stocks_merchants_v3(
        &self,
        stock_id: &str,
        stock_creator_mchid: &str,
        offset: i32,
        limit: i32,
    ) -> Result<FavorStocksMerchantsGetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/favor/stocks/{stock_id}/merchants?stock_creator_mchid={stock_creator_mchid}&offset={offset}&limit={limit}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_favor_stocks_items_v3(
        &self,
        stock_id: &str,
        stock_creator_mchid: &str,
        offset: i32,
        limit: i32,
    ) -> Result<FavorStocksItemsGetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/favor/stocks/{stock_id}/items?stock_creator_mchid={stock_creator_mchid}&offset={offset}&limit={limit}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_favor_coupons_v3(
        &self,
        request: &FavorCouponsQueryRequest,
    ) -> Result<FavorCouponsQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let mut query = format!("?appid={}", request.appid.as_deref().unwrap_or_default());
        if let Some(v) = request.stock_id.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&stock_id={v}"));
            }
        }
        if let Some(v) = request.status.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&status={v}"));
            }
        }
        if let Some(v) = request.creator_mchid.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&creator_mchid={v}"));
            }
        }
        if let Some(v) = request.sender_mchid.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&sender_mchid={v}"));
            }
        }
        if let Some(v) = request.available_mchid.as_deref() {
            if !v.trim().is_empty() {
                query.push_str(&format!("&available_mchid={v}"));
            }
        }
        if let Some(v) = request.offset {
            query.push_str(&format!("&offset={v}"));
        }
        if let Some(v) = request.limit {
            query.push_str(&format!("&limit={v}"));
        }
        let url = format!(
            "{}/v3/marketing/favor/users/{}/coupons{query}",
            svc.get_pay_base_url(),
            request.openid.as_deref().unwrap_or_default()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_favor_stocks_use_flow_v3(
        &self,
        stock_id: &str,
    ) -> Result<FavorStocksFlowGetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/favor/stocks/{stock_id}/use-flow",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_favor_stocks_refund_flow_v3(
        &self,
        stock_id: &str,
    ) -> Result<FavorStocksFlowGetResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/favor/stocks/{stock_id}/refund-flow",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn save_favor_callbacks_v3(
        &self,
        request: &FavorCallbacksSaveRequest,
    ) -> Result<FavorCallbacksSaveResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!("{}/v3/marketing/favor/callbacks", svc.get_pay_base_url());
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn pause_favor_stocks_v3(
        &self,
        stock_id: &str,
        request: &FavorStocksSetRequest,
    ) -> Result<FavorStocksPauseResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/favor/stocks/{stock_id}/pause",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn restart_favor_stocks_v3(
        &self,
        stock_id: &str,
        request: &FavorStocksSetRequest,
    ) -> Result<FavorStocksRestartResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/favor/stocks/{stock_id}/restart",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn parse_notify_data(
        &self,
        data: &str,
        header: &SignatureHeader,
    ) -> Result<UseNotifyData, WxErrorException> {
        // 对应 Java：验签（探测流量识别）后 GSON 解析
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        crate::util::wx_pay_notify_utils::verify_notify_signature(&public_key, header, data)?;
        serde_json::from_str(data).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn decrypt_notify_data_resource(
        &self,
        data: &UseNotifyData,
    ) -> Result<FavorCouponsUseResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let api_v3_key = config.api_v3_key().unwrap_or_default();
        let resource = data
            .resource
            .as_ref()
            .ok_or_else(|| impl_utils::runtime("解析报文异常！缺少 resource"))?;
        let decrypted = crate::util::crypto::wx_pay_v3_crypto_utils::aes_gcm_decrypt(
            &api_v3_key,
            resource.associated_data.as_deref().unwrap_or_default(),
            resource.nonce.as_deref().unwrap_or_default(),
            resource.cipher_text.as_deref().unwrap_or_default(),
        )
        .map_err(|e| impl_utils::runtime(format!("解析报文异常！: {e}")))?;
        serde_json::from_str(&decrypted).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
