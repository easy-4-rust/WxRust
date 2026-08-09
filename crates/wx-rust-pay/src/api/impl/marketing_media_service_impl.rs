//! 对应 Java `service.impl.MarketingMediaServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::MarketingMediaService;
use crate::api::WxPayService;
use crate::bean::*;
use crate::util::crypto::wx_pay_crypto_utils::sha256_hex;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// 微信支付营销媒体服务实现（对应 Java `MarketingMediaServiceImpl`）。
pub struct MarketingMediaServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl MarketingMediaServiceImpl {
    /// 构建实现（对应 Java 构造器 `MarketingMediaServiceImpl(WxPayService)`）。
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
impl MarketingMediaService for MarketingMediaServiceImpl {
    async fn image_upload_v3(
        &self,
        file_name: &str,
        file_data: &[u8],
    ) -> Result<MarketingImageUploadResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/marketing/favor/media/image-upload",
            svc.get_pay_base_url()
        );
        // 对应 Java `DigestUtils.sha256Hex` + `WechatPayUploadHttpPost.Builder`
        let sha256 = sha256_hex(file_data);
        let meta = format!("{{\"filename\":\"{file_name}\",\"sha256\":\"{sha256}\"}}");
        let (content_type, body) = impl_utils::build_multipart_meta_file(
            file_name,
            impl_utils::guess_file_content_type(file_name),
            file_data,
            &meta,
        );
        let result = impl_utils::execute_v3_upload(
            svc.wx_pay_config().as_ref(),
            svc.http_client(),
            &url,
            &content_type,
            &body,
        )
        .await?;
        MarketingImageUploadResult::from_json(&result)
            .map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
