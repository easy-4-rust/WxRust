//! 对应 Java `service.impl.MerchantMediaServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{MerchantMediaService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// MerchantMediaService 实现（对应 Java `MerchantMediaServiceImpl`）。
pub struct MerchantMediaServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl MerchantMediaServiceImpl {
    /// 构建实现（对应 Java 构造器 `MerchantMediaServiceImpl(WxPayService)`）。
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
impl MerchantMediaService for MerchantMediaServiceImpl {
    async fn image_upload_v3(
        &self,
        file_name: &str,
        file_data: &[u8],
    ) -> Result<ImageUploadResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!("{}/v3/merchant/media/upload", svc.get_pay_base_url());
        // 对应 Java `DigestUtils.sha256Hex` + `WechatPayUploadHttpPost.Builder.withImage`
        let sha256 = crate::util::crypto::wx_pay_crypto_utils::sha256_hex(file_data);
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
        ImageUploadResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn video_upload_v3(
        &self,
        file_name: &str,
        file_data: &[u8],
    ) -> Result<VideoUploadResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!("{}/v3/merchant/media/video_upload", svc.get_pay_base_url());
        // 对应 Java `WechatPayUploadHttpPost.Builder.withVideo`（视频走
        // octet-stream，meta 同图片格式）
        let sha256 = crate::util::crypto::wx_pay_crypto_utils::sha256_hex(file_data);
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
        VideoUploadResult::from_json(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
