use wx_rust_common::error::WxErrorException;
use wx_rust_pay::api::{PassengerTransportInvoiceService, WxPayService};
use wx_rust_pay::bean::invoice::PassengerTransportInvoiceRequest;

async fn submit(
    service: &dyn WxPayService,
    encrypted_request: &PassengerTransportInvoiceRequest,
) -> Result<(), WxErrorException> {
    // 手机号、邮箱、证件号由调用方使用与 Wechatpay-Serial 对应的平台公钥预加密。
    // 必须持久化 fapiao_apply_id/fapiao_id，受理后通过原发票查询或通知确认最终结果。
    service
        .issue_passenger_transport_invoice(encrypted_request)
        .await
}

fn main() {
    // 编译示例，不创建真实发票；由业务系统传入完整、预加密的请求。
    let _ = submit;
}
