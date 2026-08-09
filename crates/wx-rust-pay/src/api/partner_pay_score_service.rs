//! 对应 Java `com.github.binarywang.wxpay.service.PartnerPayScoreService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// PartnerPayScoreService（对应 Java `PartnerPayScoreService`）。
#[async_trait]
pub trait PartnerPayScoreService: Send + Sync {
    /// 服务商支付分相关服务类. 微信支付分是对个人的身份特质、支付行为、使用历史等情况的综合计算分值，旨在为用户提供更简单便捷的生活方式。 微信用户可以在具体应用场景中，开通微信支付分。开通后，用户可以在【
    async fn permissions(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 商户查询与用户授权记录 （authorization_code） 请求URL：https://api.mch.weixin.qq.com/v3/payscore/partner/permissions
    async fn permissions_query_by_authorization_code(
        &self,
        service_id: &str,
        sub_mchid: &str,
        authorization_code: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 商户解除用户授权关系（authorization_code） 请求URL：https://api.mch.weixin.qq.com/v3/payscore/partner/permissions/a
    async fn permissions_terminate_by_authorization_code(
        &self,
        service_id: &str,
        sub_mchid: &str,
        authorization_code: &str,
        reason: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 商户查询与用户授权记录（OpenID） 请求URL：https://api.mch.weixin.qq.com/v3/payscore/partner/permissions/search
    async fn permissions_query_by_open_id(
        &self,
        service_id: &str,
        app_id: &str,
        sub_mchid: &str,
        sub_appid: &str,
        open_id: &str,
        sub_openid: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 商户解除用户授权关系API（OpenID） 请求URL：https://api.mch.weixin.qq.com/v3/payscore/partner/permissions/terminate
    async fn permissions_terminate_by_open_id(
        &self,
        service_id: &str,
        app_id: &str,
        sub_mchid: &str,
        sub_appid: &str,
        open_id: &str,
        sub_openid: &str,
        reason: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 支付分创建订单API. 请求URL：https://api.mch.weixin.qq.com/v3/payscore/partner/serviceorder
    async fn create_service_order(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 支付分查询订单API. 请求URL：https://api.mch.weixin.qq.com/v3/payscore/partner/serviceorder
    async fn query_service_order(
        &self,
        service_id: &str,
        sub_mchid: &str,
        out_order_no: &str,
        query_id: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 支付分取消订单API. 请求URL：https://api.mch.weixin.qq.com/v3/payscore/partner/serviceorder/{out_order_no}/canc
    async fn cancel_service_order(
        &self,
        service_id: &str,
        app_id: &str,
        sub_mchid: &str,
        out_order_no: &str,
        reason: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 支付分修改订单金额API. 请求URL：https://api.mch.weixin.qq.com/v3/payscore/partner/serviceorder/{out_order_no}/mo
    async fn modify_service_order(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 支付分完结订单API. 请求URL：https://api.mch.weixin.qq.com/v3/payscore/partner/serviceorder/{out_order_no}/comp
    async fn complete_service_order(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<(), WxErrorException>;

    /// 订单收款 请求URL：https://api.mch.weixin.qq.com/v3/payscore/partner/serviceorder/{out_order_no}/pay
    async fn pay_service_order(
        &self,
        service_id: &str,
        app_id: &str,
        sub_mchid: &str,
        out_order_no: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 同步订单信息 请求URL： https://api.mch.weixin.qq.com/v3/payscore/partner/serviceorder/{out_order_no}/sync
    async fn sync_service_order(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 收付通子商户申请绑定支付分服务API. 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore_partner/chapter9_1
    async fn apply_service_account(
        &self,
        request: &WxPartnerPayScoreRequest,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 查询收付通子商户服务绑定结果API. 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/payscore_partner/chapter9_2.
    async fn query_service_account_state(
        &self,
        out_apply_no: &str,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;

    /// 授权/解除授权服务回调通知
    async fn parse_user_authorization_status_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<WxPartnerUserAuthorizationStatusNotifyResult, WxErrorException>;

    /// 支付分回调内容解析方法
    async fn parse_notify_data(
        &self,
        data: &str,
        header: &SignatureHeader,
    ) -> Result<PayScoreNotifyData, WxErrorException>;

    /// 支付分回调NotifyData解密resource
    async fn decrypt_notify_data_resource(
        &self,
        data: &PayScoreNotifyData,
    ) -> Result<WxPartnerPayScoreResult, WxErrorException>;
}
