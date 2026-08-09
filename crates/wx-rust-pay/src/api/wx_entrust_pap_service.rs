//! 对应 Java `com.github.binarywang.wxpay.service.WxEntrustPapService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// WxEntrustPapService（对应 Java `WxEntrustPapService`）。
#[async_trait]
pub trait WxEntrustPapService: Send + Sync {
    /// 微信签约代扣相关接口. https://pay.weixin.qq.com/wiki/doc/api/wxpay_v2/papay/chapter2_8.shtml created on 2021 -
    async fn mp_sign(
        &self,
        wx_mp_entrust_request: &WxMpEntrustRequest,
    ) -> Result<String, WxErrorException>;

    /// 获取小程序纯签约参数json 详见：https://pay.weixin.qq.com/wiki/doc/api/wxpay_v2/papay/chapter3_3.shtml 返回一个json 前端
    async fn ma_sign(
        &self,
        wx_ma_entrust_request: &WxMaEntrustRequest,
    ) -> Result<String, WxErrorException>;

    /// 获取h5纯签约支付跳转链接 详见：https://pay.weixin.qq.com/wiki/doc/api/wxpay_v2/papay/chapter3_4.shtml 返回一个签约链接 在浏览
    async fn h5_sign(
        &self,
        wx_h5_entrust_request: &WxH5EntrustRequest,
    ) -> Result<WxH5EntrustResult, WxErrorException>;

    /// 支付中签约 详见：https://pay.weixin.qq.com/wiki/doc/api/wxpay_v2/papay/chapter3_5.shtml 请求微信 若微信内请求 需要构造json
    async fn pay_sign(
        &self,
        wx_pay_entrust_request: &WxPayEntrustRequest,
    ) -> Result<WxPayEntrustResult, WxErrorException>;

    /// 申请扣款 申请扣款 详见：https://pay.weixin.qq.com/wiki/doc/api/wxpay_v2/papay/chapter3_8.shtml 请求微信发起委托扣款，扣款额度和
    async fn withhold(
        &self,
        wx_withhold_request: &WxWithholdRequest,
    ) -> Result<WxWithholdResult, WxErrorException>;

    /// 服务商模式的申请扣款 申请扣款 详见：https://pay.weixin.qq.com/wiki/doc/api/wxpay_v2/papay/chapter5_8.shtml 请求微信发起委托扣款
    async fn withhold_partner(
        &self,
        wx_withhold_request: &WxWithholdRequest,
    ) -> Result<WxPayCommonResult, WxErrorException>;

    /// 预扣费通知 预扣费接口 详见：https://pay.weixin.qq.com/wiki/doc/api/wxpay_v2/papay/chapter3_10.shtml 商户进行委托代扣扣费前需要
    async fn pre_withhold(
        &self,
        wx_pre_withhold_request: &WxPreWithholdRequest,
    ) -> Result<String, WxErrorException>;

    /// 签约状态查询 签约状态查询 详见：https://pay.weixin.qq.com/wiki/doc/api/wxpay_v2/papay/chapter3_7.shtml 查询签约关系接口提供单笔
    async fn query_sign(
        &self,
        wx_sign_query_request: &WxSignQueryRequest,
    ) -> Result<WxSignQueryResult, WxErrorException>;

    /// 申请解约 申请解约 详见：https://pay.weixin.qq.com/wiki/doc/api/wxpay_v2/papay/chapter3_9.shtml 商户与用户的签约关系有误或者商户
    async fn termination_contract(
        &self,
        wx_terminated_contract_request: &WxTerminatedContractRequest,
    ) -> Result<WxTerminationContractResult, WxErrorException>;

    /// 查询代扣订单 详见：https://pay.weixin.qq.com/wiki/doc/api/wxpay_v2/papay/chapter4_5.shtml 该接口仅提供微信扣款服务申请扣款接口创
    async fn pap_order_query(
        &self,
        wx_withhold_order_query_request: &WxWithholdOrderQueryRequest,
    ) -> Result<WxWithholdOrderQueryResult, WxErrorException>;

    /// 签约、解约结果通知解析 详见：签约、解约结果通知 注意： 1、同样的通知可能会多次发送给商户系统。商户系统必须能够正确处理重复的通知。 推荐的做法是：当商户系统收到通知进行处理时，先检查对应业务数据的
    async fn parse_sign_notify_result(
        &self,
        xml_data: &str,
    ) -> Result<WxSignQueryResult, WxErrorException>;
}
