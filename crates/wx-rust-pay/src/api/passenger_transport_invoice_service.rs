use crate::api::WxPayService;
use crate::bean::invoice::PassengerTransportInvoiceRequest;
use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

/// 服务商旅客运输行业发票扩展；保留既有自定义发票服务实现的兼容性。
/// 对应 Java: com.github.binarywang.wxpay.service.PartnerInvoiceService（旅客开票方法）
#[async_trait]
pub trait PassengerTransportInvoiceService: WxPayService {
    /// 提交旅客运输开票申请，成功只表示已受理；最终结果通过查询或通知获取。
    /// 参数 request 中的手机号、邮箱和证件号必须由调用方按当前平台密钥预加密。
    /// 返回受理结果或支付错误。对应 Java: PartnerInvoiceService#issuePassengerTransportInvoice
    async fn issue_passenger_transport_invoice(
        &self,
        request: &PassengerTransportInvoiceRequest,
    ) -> Result<(), WxErrorException> {
        let url = format!(
            "{}/v3/new-tax-control-fapiao/fapiao-applications/issue-passenger-transport",
            self.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        // 使用现有 V3 签名和平台密钥标识；密文字段原样透传，202 空响应无需 JSON 解析。
        self.post_v3_with_wechatpay_serial(&url, &body).await?;
        Ok(())
    }
}

impl<T: WxPayService + ?Sized> PassengerTransportInvoiceService for T {}
