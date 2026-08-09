//! 对应 Java `com.github.binarywang.wxpay.service.EntPayService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// EntPayService（对应 Java `EntPayService`）。
#[async_trait]
pub trait EntPayService: Send + Sync {
    /// 企业付款相关服务类. Created by BinaryWang on 2017/12/19.
    async fn ent_pay(&self, request: &EntPayRequest) -> Result<EntPayResult, WxErrorException>;

    /// 查询企业付款API. 用于商户的企业付款操作进行结果查询，返回付款操作详细结果。 文档详见:https://pay.weixin.qq.com/wiki/doc/api/tools/mch_pay.p
    async fn query_ent_pay(
        &self,
        partner_trade_no: &str,
    ) -> Result<EntPayQueryResult, WxErrorException>;

    /// 查询企业付款API. 用于商户的企业付款操作进行结果查询，返回付款操作详细结果。 文档详见:https://pay.weixin.qq.com/wiki/doc/api/tools/mch_pay.p
    async fn query_ent_pay_with_request(
        &self,
        request: &EntPayQueryRequest,
    ) -> Result<EntPayQueryResult, WxErrorException>;

    /// 获取RSA加密公钥API. RSA算法使用说明（非对称加密算法，算法采用RSA/ECB/OAEPPadding模式） 1、 调用获取RSA公钥API获取RSA公钥，落地成本地文件，假设为public.
    async fn get_public_key(&self) -> Result<String, WxErrorException>;

    /// 企业付款到银行卡. 用于企业向微信用户银行卡付款 目前支持接口API的方式向指定微信用户的银行卡付款。 文档详见：https://pay.weixin.qq.com/wiki/doc/api/tool
    async fn pay_bank(
        &self,
        request: &EntPayBankRequest,
    ) -> Result<EntPayBankResult, WxErrorException>;

    /// 企业付款到银行卡查询. 用于对商户企业付款到银行卡操作进行结果查询，返回付款操作详细结果。 文档详见：https://pay.weixin.qq.com/wiki/doc/api/tools/mch_
    async fn query_pay_bank(
        &self,
        partner_trade_no: &str,
    ) -> Result<EntPayBankQueryResult, WxErrorException>;

    /// 企业付款到银行卡查询. 用于对商户企业付款到银行卡操作进行结果查询，返回付款操作详细结果。 文档详见：https://pay.weixin.qq.com/wiki/doc/api/tools/mch_
    async fn query_pay_bank_with_request(
        &self,
        request: &EntPayBankQueryRequest,
    ) -> Result<EntPayBankQueryResult, WxErrorException>;

    /// 企业发送微信红包给个人用户 文档地址：https://work.weixin.qq.com/api/doc 接口地址： https://api.mch.weixin.qq.com/mmpaymkttr
    async fn send_enterprise_redpack(
        &self,
        request: &EntPayRedpackRequest,
    ) -> Result<EntPayRedpackResult, WxErrorException>;

    /// 企业发送微信红包查询 文档地址：https://work.weixin.qq.com/api/doc 接口地址： https://api.mch.weixin.qq.com/mmpaymkttrans
    async fn query_enterprise_redpack(
        &self,
        request: &EntPayRedpackQueryRequest,
    ) -> Result<EntPayRedpackQueryResult, WxErrorException>;

    /// 向员工付款 文档详见 https://work.weixin.qq.com/api/doc/90000/90135/90278 接口链接 https://api.mch.weixin.qq.com/m
    async fn to_emp_pay(
        &self,
        request: &EntWxEmpPayRequest,
    ) -> Result<EntPayResult, WxErrorException>;
}
