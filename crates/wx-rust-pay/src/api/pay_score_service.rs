//! 对应 Java `com.github.binarywang.wxpay.service.PayScoreService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// PayScoreService（对应 Java `PayScoreService`）。
#[async_trait]
pub trait PayScoreService: Send + Sync {
    /// 支付分相关服务类. 微信支付分是对个人的身份特质、支付行为、使用历史等情况的综合计算分值，旨在为用户提供更简单便捷的生活方式。 微信用户可以在具体应用场景中，开通微信支付分。开通后，用户可以在【微信—
    async fn permissions(
        &self,
        request: &WxPayScoreRequest,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 支付分查询与用户授权记录（授权协议号）API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter5_2.shtm
    async fn permissions_query_by_authorization_code(
        &self,
        authorization_code: &str,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 解除用户授权关系（授权协议号）API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter5_3.shtml 接口
    async fn permissions_terminate_by_authorization_code(
        &self,
        authorization_code: &str,
        reason: &str,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 支付分查询与用户授权记录（openid）API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter5_4shtm
    async fn permissions_query_by_open_id(
        &self,
        open_id: &str,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 解除用户授权关系（openid）API 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter5_5.shtml 接
    async fn permissions_terminate_by_open_id(
        &self,
        open_id: &str,
        reason: &str,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 支付分创建订单API. 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter3_1.shtml 接口链接：http
    async fn create_service_order(
        &self,
        request: &WxPayScoreRequest,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 支付分查询订单API. 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter3_2.shtml 接口链接：http
    async fn query_service_order(
        &self,
        out_order_no: &str,
        query_id: &str,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 支付分取消订单API. 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter3_3.shtml 接口链接：http
    async fn cancel_service_order(
        &self,
        out_order_no: &str,
        reason: &str,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 支付分修改订单金额API. 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter3_4.shtml 接口链接：ht
    async fn modify_service_order(
        &self,
        request: &WxPayScoreRequest,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 支付分完结订单API. 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter3_5.shtml 请求URL：htt
    async fn complete_service_order(
        &self,
        request: &WxPayScoreRequest,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 支付分订单收款API. 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter3_6.shtml 请求URL：htt
    async fn pay_service_order(
        &self,
        out_order_no: &str,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 支付分订单收款API. 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter3_7.shtml 请求URL： ht
    async fn sync_service_order(
        &self,
        request: &WxPayScoreRequest,
    ) -> Result<WxPayScoreResult, WxErrorException>;

    /// 授权/解除授权服务回调数据处理 文档地址: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter4_4.shtml
    async fn parse_user_authorization_status_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<UserAuthorizationStatusNotifyResult, WxErrorException>;

    /// 支付分回调内容解析方法 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter5_2.shtml
    async fn parse_notify_data(
        &self,
        data: &str,
        header: &SignatureHeader,
    ) -> Result<PayScoreNotifyData, WxErrorException>;

    /// 支付分回调NotifyData解密resource 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore/chapter5_2.s
    async fn decrypt_notify_data_resource(
        &self,
        data: &PayScoreNotifyData,
    ) -> Result<WxPayScoreResult, WxErrorException>;
}
