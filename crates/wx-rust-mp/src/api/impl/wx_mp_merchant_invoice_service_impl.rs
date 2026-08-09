//! WxMpMerchantInvoiceService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpMerchantInvoiceServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpMerchantInvoiceService, WxMpService};
use crate::bean::invoice::merchant::{
    ClearOutInvoiceRequest, InvoiceAuthDataRequest, InvoiceAuthDataResult, InvoiceAuthPageRequest,
    InvoiceAuthPageResult, InvoiceAuthPageSetting, InvoiceRejectRequest, InvoiceResult,
    MakeOutInvoiceRequest, MerchantContactInfo, MerchantInvoicePlatformInfo,
};
use crate::enums::wx_mp_api_url::merchant_invoice;

/// 公众号MerchantInvoiceService实现。
pub struct WxMpMerchantInvoiceServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpMerchantInvoiceServiceImpl {
    /// 构建 公众号MerchantInvoiceService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpMerchantInvoiceService for WxMpMerchantInvoiceServiceImpl {
    async fn get_auth_page_url(
        &self,
        params: &InvoiceAuthPageRequest,
    ) -> Result<InvoiceAuthPageResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(params).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&merchant_invoice::get_auth_url(config.as_ref()), &body)
            .await?;
        InvoiceAuthPageResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_auth_data(
        &self,
        params: &InvoiceAuthDataRequest,
    ) -> Result<InvoiceAuthDataResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(params).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&merchant_invoice::get_auth_data(config.as_ref()), &body)
            .await?;
        InvoiceAuthDataResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn reject_invoice(&self, params: &InvoiceRejectRequest) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(params).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&merchant_invoice::reject_insert(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn make_out_invoice(
        &self,
        params: &MakeOutInvoiceRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(params).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&merchant_invoice::make_out_invoice(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn clear_out_invoice(
        &self,
        params: &ClearOutInvoiceRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(params).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&merchant_invoice::clear_out_invoice(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn query_invoice_info(
        &self,
        fpqqlsh: &str,
        nsrsbh: &str,
    ) -> Result<InvoiceResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"fpqqlsh": fpqqlsh, "nsrsbh": nsrsbh});
        let response = svc
            .post(
                &merchant_invoice::query_invoice_info(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        InvoiceResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn set_merchant_contact_info(
        &self,
        contact: &MerchantContactInfo,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"contact": contact});
        svc.post(
            &merchant_invoice::set_contact(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_merchant_contact_info(&self) -> Result<MerchantContactInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .post(&merchant_invoice::get_contact(config.as_ref()), "{}")
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let contact = value
            .get("contact")
            .ok_or_else(|| WxErrorException::from_code(-99, "contact 缺失"))?;
        serde_json::from_value(contact.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn set_auth_page_setting(
        &self,
        setting: &InvoiceAuthPageSetting,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"auth_field": setting});
        svc.post(
            &merchant_invoice::set_auth_page(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_auth_page_setting(&self) -> Result<InvoiceAuthPageSetting, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .post(&merchant_invoice::get_auth_page(config.as_ref()), "{}")
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let field = value
            .get("auth_field")
            .ok_or_else(|| WxErrorException::from_code(-99, "auth_field 缺失"))?;
        serde_json::from_value(field.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn set_merchant_invoice_platform(
        &self,
        info: &MerchantInvoicePlatformInfo,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"pay_mch": info});
        svc.post(
            &merchant_invoice::set_platform(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_merchant_invoice_platform(
        &self,
        info: &MerchantInvoicePlatformInfo,
    ) -> Result<MerchantInvoicePlatformInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"pay_mch": info});
        let response = svc
            .post(
                &merchant_invoice::get_platform(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let pay = value
            .get("pay_mch")
            .ok_or_else(|| WxErrorException::from_code(-99, "pay_mch 缺失"))?;
        serde_json::from_value(pay.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
