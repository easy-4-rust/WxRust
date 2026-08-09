//! 对应 Java `com.github.binarywang.wxpay.service.MiPayService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// MiPayService（对应 Java `MiPayService`）。
#[async_trait]
pub trait MiPayService: Send + Sync {
    /// 医保相关接口 医保相关接口
    async fn med_ins_orders(
        &self,
        request: &MedInsOrdersRequest,
    ) -> Result<MedInsOrdersResult, WxErrorException>;

    /// 使用医保自费混合订单号查看下单结果 从业机构使用混合下单订单号，通过该接口主动查询订单状态，完成下一步的业务逻辑。 文档地址：使用医保自费混合订单号查看下单结果
    async fn get_med_ins_order_by_mix_trade_no(
        &self,
        mix_trade_no: &str,
        sub_mchid: &str,
    ) -> Result<MedInsOrdersResult, WxErrorException>;

    /// 使用从业机构订单号查看下单结果 从业机构使用从业机构订单号、医疗机构商户号，通过该接口主动查询订单状态，完成下一步的业务逻辑。 文档地址：使用从业机构订单号查看下单结果
    async fn get_med_ins_order_by_out_trade_no(
        &self,
        out_trade_no: &str,
        sub_mchid: &str,
    ) -> Result<MedInsOrdersResult, WxErrorException>;

    /// 解析医保混合收款成功通知 微信支付会通过POST请求向商户设置的回调URL推送医保混合收款成功通知，商户需要接收处理该消息，并返回应答。 文档地址：医保混合收款成功通知
    async fn parse_mi_pay_notify_v3_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<MiPayNotifyV3Result, WxErrorException>;

    /// 医保退款通知 从业机构调用该接口向微信医保后台通知医保订单的退款成功结果 文档地址：医保退款通知
    async fn med_ins_refund_notify(
        &self,
        request: &MedInsRefundNotifyRequest,
        mix_trade_no: &str,
    ) -> Result<(), WxErrorException>;
}
