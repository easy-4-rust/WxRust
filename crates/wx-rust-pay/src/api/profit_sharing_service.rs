//! 对应 Java `com.github.binarywang.wxpay.service.ProfitSharingService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;
// 扁平重导出的 ProfitSharingRequest/ProfitSharingResult 等为 ecommerce 同名类型，
// 此处以全限定路径显式导入 profitsharing 包版本（对应 Java `bean.profitsharing` 包）。
use crate::bean::profitsharing::request::profit_sharing_merchant_ratio_query_request::ProfitSharingMerchantRatioQueryRequest;
use crate::bean::profitsharing::request::profit_sharing_order_amount_query_request::ProfitSharingOrderAmountQueryRequest;
use crate::bean::profitsharing::request::profit_sharing_query_request::ProfitSharingQueryRequest;
use crate::bean::profitsharing::request::profit_sharing_receiver_request::ProfitSharingReceiverRequest;
use crate::bean::profitsharing::request::profit_sharing_request::ProfitSharingRequest;
use crate::bean::profitsharing::request::profit_sharing_return_query_request::ProfitSharingReturnQueryRequest;
use crate::bean::profitsharing::request::profit_sharing_return_request::ProfitSharingReturnRequest;
use crate::bean::profitsharing::result::profit_sharing_receiver_result::ProfitSharingReceiverResult;
use crate::bean::profitsharing::result::profit_sharing_result::ProfitSharingResult;

/// ProfitSharingService（对应 Java `ProfitSharingService`）。
#[async_trait]
pub trait ProfitSharingService: Send + Sync {
    /// 注意：微信最高分账比例为30% 可多次分账到同一个人，但是依然不能超过30%
    async fn profit_sharing(
        &self,
        request: &ProfitSharingRequest,
    ) -> Result<ProfitSharingResult, WxErrorException>;

    /// 微信订单支付成功后，服务商代子商户发起分账请求，将结算后的钱分到分账接收方。多次分账请求仅会按照传入的分账接收方进行分账，不会对剩余的金额进行任何操作。故操作成功后，在待分账金额不等于零时，订单依旧能
    async fn multi_profit_sharing(
        &self,
        request: &ProfitSharingRequest,
    ) -> Result<ProfitSharingResult, WxErrorException>;

    /// 请求分账API 微信订单支付成功后，商户发起分账请求，将结算后的资金分到分账接收方 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapte
    async fn profit_sharing_v3(
        &self,
        request: &ProfitSharingV3Request,
    ) -> Result<ProfitSharingV3Result, WxErrorException>;

    /// 1、不需要进行分账的订单，可直接调用本接口将订单的金额全部解冻给特约商户 2、调用多次分账接口后，需要解冻剩余资金时，调用本接口将剩余的分账金额全部解冻给特约商户 3、已调用请求单次分账后，剩余待分账
    async fn profit_sharing_finish(
        &self,
        request: &ProfitSharingUnfreezeRequest,
    ) -> Result<ProfitSharingResult, WxErrorException>;

    /// 服务商代子商户发起添加分账接收方请求，后续可通过发起分账请求将结算后的钱分到该分账接收方。 文档详见: https://pay.weixin.qq.com/wiki/doc/api/allocatio
    async fn add_receiver(
        &self,
        request: &ProfitSharingReceiverRequest,
    ) -> Result<ProfitSharingReceiverResult, WxErrorException>;

    /// 服务商代子商户发起删除分账接收方请求，删除后不支持将结算后的钱分到该分账接收方。 文档详见: https://pay.weixin.qq.com/wiki/doc/api/allocation_sl.
    async fn remove_receiver(
        &self,
        request: &ProfitSharingReceiverRequest,
    ) -> Result<ProfitSharingReceiverResult, WxErrorException>;

    /// 添加分账接收方API 商户发起添加分账接收方请求，建立分账接收方列表。后续可通过发起分账请求，将分账方商户结算后的资金，分到该分账接收方 文档详见: https://pay.weixin.qq.com
    async fn add_receiver_v3(
        &self,
        request: &ProfitSharingReceiverV3Request,
    ) -> Result<ProfitSharingReceiverV3Result, WxErrorException>;

    /// 删除分账接收方API 商户发起删除分账接收方请求。删除后，不支持将分账方商户结算后的资金，分到该分账接收方 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3
    async fn remove_receiver_v3(
        &self,
        request: &ProfitSharingReceiverV3Request,
    ) -> Result<ProfitSharingReceiverV3Result, WxErrorException>;

    /// TODO:微信返回签名失败 发起分账请求后，可调用此接口查询分账结果；发起分账完结请求后，可调用此接口查询分账完结的执行结果。 接口频率：80QPS
    async fn profit_sharing_query(
        &self,
        request: &ProfitSharingQueryRequest,
    ) -> Result<ProfitSharingQueryResult, WxErrorException>;

    /// 查询分账结果API（商户平台） 发起分账请求后，可调用此接口查询分账结果 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter8_1_
    async fn profit_sharing_query_v3(
        &self,
        out_order_no: &str,
        transaction_id: &str,
    ) -> Result<ProfitSharingV3Result, WxErrorException>;

    /// 查询分账结果API（商户平台） 发起分账请求后，可调用此接口查询分账结果 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter8_1_
    async fn profit_sharing_query_v3_with_account_type(
        &self,
        out_order_no: &str,
        transaction_id: &str,
        sub_mch_id: &str,
    ) -> Result<ProfitSharingV3Result, WxErrorException>;

    /// 查询分账结果API（商户平台） 发起分账请求后，可调用此接口查询分账结果 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter8_1_
    async fn profit_sharing_query_v3_with_request(
        &self,
        request: &ProfitSharingQueryV3Request,
    ) -> Result<ProfitSharingV3Result, WxErrorException>;

    /// 服务商可通过调用此接口查询订单剩余待分金额。 接口频率：30QPS 文档详见: https://pay.weixin.qq.com/wiki/doc/api/allocation_sl.php?cha
    async fn profit_sharing_order_amount_query(
        &self,
        request: &ProfitSharingOrderAmountQueryRequest,
    ) -> Result<ProfitSharingOrderAmountQueryResult, WxErrorException>;

    /// 查询剩余待分金额API 可调用此接口查询订单剩余待分金额 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter8_1_6.shtml
    async fn profit_sharing_unsplit_amount_query_v3(
        &self,
        transaction_id: &str,
    ) -> Result<ProfitSharingOrderAmountQueryV3Result, WxErrorException>;

    /// 服务商可以查询子商户设置的允许服务商分账的最大比例。 接口频率：30QPS 文档详见: https://pay.weixin.qq.com/wiki/doc/api/allocation_sl.php
    async fn profit_sharing_merchant_ratio_query(
        &self,
        request: &ProfitSharingMerchantRatioQueryRequest,
    ) -> Result<ProfitSharingMerchantRatioQueryResult, WxErrorException>;

    /// 查询最大分账比例 可调用此接口查询特约商户设置的允许服务商分账的最大比例 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3_partner/apis/cha
    async fn profit_sharing_merchant_ratio_query_v3(
        &self,
        sub_mch_id: &str,
    ) -> Result<ProfitSharingMerchantRatioQueryV3Result, WxErrorException>;

    /// TODO:这个接口用真实的数据返回【参数不正确】，我对比官方文档除了缺少sub_mch_id，和sub_appid之外其他相同，当我随便填了一个商户id的时候，提示【回退方没有开通分账回退功能】 仅对
    async fn profit_sharing_return(
        &self,
        return_request: &ProfitSharingReturnRequest,
    ) -> Result<ProfitSharingReturnResult, WxErrorException>;

    /// 请求分账回退API 如果订单已经分账，在退款时，可以先调此接口，将已分账的资金从分账接收方的账户回退给分账方，再发起退款 文档详见: https://pay.weixin.qq.com/wiki/do
    async fn profit_sharing_return_v3(
        &self,
        request: &ProfitSharingReturnV3Request,
    ) -> Result<ProfitSharingReturnV3Result, WxErrorException>;

    /// TODO:因profitsharingReturn接口无法使用，没有办法对这里进行真实的测试，模拟数据这里返回【记录不存在】 商户需要核实回退结果，可调用此接口查询回退结果。 如果分账回退接口返回状态
    async fn profit_sharing_return_query(
        &self,
        query_request: &ProfitSharingReturnQueryRequest,
    ) -> Result<ProfitSharingReturnResult, WxErrorException>;

    /// 查询分账回退结果API（商户平台） 商户需要核实回退结果，可调用此接口查询回退结果 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapte
    async fn profit_sharing_return_query_v3(
        &self,
        out_order_no: &str,
        out_return_no: &str,
    ) -> Result<ProfitSharingReturnV3Result, WxErrorException>;

    /// 查询分账回退结果API（商户平台） 商户需要核实回退结果，可调用此接口查询回退结果 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapte
    async fn profit_sharing_return_query_v3_with_account_type(
        &self,
        out_order_no: &str,
        out_return_no: &str,
        sub_mch_id: &str,
    ) -> Result<ProfitSharingReturnV3Result, WxErrorException>;

    /// 解冻剩余资金API 不需要进行分账的订单，可直接调用本接口将订单的金额全部解冻给特约商户 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/apis/cha
    async fn profit_sharing_unfreeze(
        &self,
        request: &ProfitSharingUnfreezeV3Request,
    ) -> Result<ProfitSharingUnfreezeV3Result, WxErrorException>;

    /// 分账动账通知 分账或分账回退成功后，微信会把相关变动结果发送给分账接收方（只支持商户）。 对后台通知交互时，如果微信收到应答不是成功或超时，微信认为通知失败，微信会通过一定的策略定期重新发起通知，尽可
    async fn parse_profit_sharing_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<ProfitSharingNotifyV3Result, WxErrorException>;

    /// 申请分账账单 微信支付按天提供分账账单文件，商户可以通过该接口获取账单文件的下载地址 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3_partner/ap
    async fn profit_sharing_bill(
        &self,
        request: &ProfitSharingBillV3Request,
    ) -> Result<ProfitSharingBillV3Result, WxErrorException>;
}
