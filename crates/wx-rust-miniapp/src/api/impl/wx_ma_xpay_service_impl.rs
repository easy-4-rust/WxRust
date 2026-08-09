//! 小程序虚拟支付服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaXPayServiceImpl`。
//! 签名逻辑照搬 Java `WxMaXPaySigParams`：`pay_sig` 为
//! HMAC-SHA256(uri 无 query 部分 + "&" + 请求体, appKey) 十六进制小写，
//! `signature` 为 HMAC-SHA256(请求体, sessionKey) 十六进制小写；URL 中的
//! `%s` 占位符按序替换（Java `String.format`）。请求体/响应解析逐方法对齐。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::SignUtils;

use crate::api::WxMaService;
use crate::api::g4_services::WxMaXPayService;
use crate::bean::WxMaBaseResponse;
use crate::bean::xpay::{
    WxMaXPayBindTransferAccountRequest, WxMaXPayCancelCurrencyPayRequest,
    WxMaXPayCancelCurrencyPayResponse, WxMaXPayCompleteComplaintRequest,
    WxMaXPayCreateFundsBillRequest, WxMaXPayCreateFundsBillResponse,
    WxMaXPayCreateWithdrawOrderRequest, WxMaXPayCreateWithdrawOrderResponse,
    WxMaXPayCurrencyPayRequest, WxMaXPayCurrencyPayResponse,
    WxMaXPayDownloadAdverfundsOrderRequest, WxMaXPayDownloadAdverfundsOrderResponse,
    WxMaXPayDownloadBillRequest, WxMaXPayDownloadBillResponse, WxMaXPayGetComplaintDetailRequest,
    WxMaXPayGetComplaintDetailResponse, WxMaXPayGetComplaintListRequest,
    WxMaXPayGetComplaintListResponse, WxMaXPayGetNegotiationHistoryRequest,
    WxMaXPayGetNegotiationHistoryResponse, WxMaXPayGetUploadFileSignRequest,
    WxMaXPayGetUploadFileSignResponse, WxMaXPayNotifyProvideGoodsRequest,
    WxMaXPayPresentCurrencyRequest, WxMaXPayPresentCurrencyResponse, WxMaXPayPresentGoodsRequest,
    WxMaXPayPresentGoodsResponse, WxMaXPayQueryAdverFundsRequest, WxMaXPayQueryAdverFundsResponse,
    WxMaXPayQueryBizBalanceRequest, WxMaXPayQueryBizBalanceResponse, WxMaXPayQueryFundsBillRequest,
    WxMaXPayQueryFundsBillResponse, WxMaXPayQueryOrderRequest, WxMaXPayQueryOrderResponse,
    WxMaXPayQueryPublishGoodsRequest, WxMaXPayQueryPublishGoodsResponse,
    WxMaXPayQueryRecoverBillRequest, WxMaXPayQueryRecoverBillResponse,
    WxMaXPayQueryTransferAccountRequest, WxMaXPayQueryTransferAccountResponse,
    WxMaXPayQueryUploadGoodsRequest, WxMaXPayQueryUploadGoodsResponse,
    WxMaXPayQueryUserBalanceRequest, WxMaXPayQueryUserBalanceResponse,
    WxMaXPayQueryWithdrawOrderRequest, WxMaXPayQueryWithdrawOrderResponse,
    WxMaXPayRefundOrderRequest, WxMaXPayRefundOrderResponse, WxMaXPayResponseComplaintRequest,
    WxMaXPaySigParams, WxMaXPayStartPublishGoodsRequest, WxMaXPayStartUploadGoodsRequest,
    WxMaXPayUploadVpFileRequest, WxMaXPayUploadVpFileResponse,
};
use crate::enums::g4_urls::url_g4_ability::xpay as xpay_url;

/// 小程序虚拟支付服务实现。
pub struct WxMaXPayServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaXPayServiceImpl {
    /// 构建虚拟支付服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 转换 URL 为签名用 uri（对应 Java `WxMaXPaySigParams.convUrlToSigUri`：
    /// 去掉 `https://api.weixin.qq.com` 前缀并截掉 query 部分）。
    fn conv_url_to_sig_uri(url: &str) -> String {
        let t = url.replace("https://api.weixin.qq.com", "");
        match t.find('?') {
            Some(idx) => t[..idx].to_string(),
            None => t,
        }
    }

    /// 计算支付签名 pay_sig（对应 Java `WxMaXPaySigParams.calcPaySignature`：
    /// HMAC-SHA256(uri + '&' + postBody, appKey)，十六进制**小写**）。
    fn calc_pay_sig(url: &str, post_body: &str, app_key: &str) -> String {
        let app_key = app_key.trim();
        let sig_uri = Self::conv_url_to_sig_uri(url);
        let need_sign_data = format!("{sig_uri}&{post_body}");
        SignUtils::create_hmac_sha256_sign(&need_sign_data, app_key).to_lowercase()
    }

    /// 计算用户登录态签名 signature（对应 Java `WxMaXPaySigParams.calcSignature`：
    /// HMAC-SHA256(postBody, sessionKey)，十六进制**小写**）。
    fn calc_sig(post_body: &str, session_key: &str) -> String {
        let session_key = session_key.trim();
        SignUtils::create_hmac_sha256_sign(post_body, session_key).to_lowercase()
    }

    /// 双签名：URL 中两个 `%s` 按序替换为 pay_sig/signature
    /// （对应 Java `WxMaXPaySigParams.signUriWithBoth`）。
    fn sign_uri_with_both(url: &str, post_body: &str, sig_params: &WxMaXPaySigParams) -> String {
        let pay_sig = Self::calc_pay_sig(url, post_body, &sig_params.app_key);
        let sig = Self::calc_sig(post_body, &sig_params.session_key);
        url.replacen("%s", &pay_sig, 1).replacen("%s", &sig, 1)
    }

    /// 单签名：URL 中一个 `%s` 替换为 pay_sig
    /// （对应 Java `WxMaXPaySigParams.signUriWithPay`）。
    fn sign_uri_with_pay(url: &str, post_body: &str, sig_params: &WxMaXPaySigParams) -> String {
        let pay_sig = Self::calc_pay_sig(url, post_body, &sig_params.app_key);
        url.replacen("%s", &pay_sig, 1)
    }

    /// 序列化请求对象为 JSON（对应 Java `request.toJson()`）。
    fn to_json<T: serde::Serialize>(request: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(request).map_err(WxErrorException::from)
    }

    /// POST 已签名 URI 并解析响应（对应 Java `post` + gson `fromJson`；
    /// errcode!=0 由执行引擎抛错，Java 在此再校验一次，语义一致）。
    async fn post_signed<T>(
        svc: &dyn WxMaService,
        uri: &str,
        post_body: &str,
    ) -> Result<T, WxErrorException>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = svc.post(uri, post_body).await?;
        serde_json::from_str(&response).map_err(WxErrorException::from)
    }

    /// POST 已签名 URI 并解析 `WxMaBaseResponse`，成功返回 true
    /// （对应 Java 返回 boolean 的方法：errcode!=0 抛错，否则 true）。
    async fn post_signed_bool(
        svc: &dyn WxMaService,
        uri: &str,
        post_body: &str,
    ) -> Result<bool, WxErrorException> {
        let detail: WxMaBaseResponse = Self::post_signed(svc, uri, post_body).await?;
        if detail.errcode != 0 {
            return Err(WxErrorException::from_code(detail.errcode, detail.errmsg));
        }
        Ok(true)
    }
}

#[async_trait]
impl WxMaXPayService for WxMaXPayServiceImpl {
    /// 查询用户虚拟币余额（对应 Java `WxMaXPayServiceImpl.queryUserBalance`，双签名）。
    async fn query_user_balance(
        &self,
        request: &WxMaXPayQueryUserBalanceRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryUserBalanceResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::query_user_balance_url(config.as_ref());
        let uri = Self::sign_uri_with_both(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 虚拟币充值下单（对应 Java `WxMaXPayServiceImpl.currencyPay`，双签名）。
    async fn currency_pay(
        &self,
        request: &WxMaXPayCurrencyPayRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayCurrencyPayResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::currency_pay_url(config.as_ref());
        let uri = Self::sign_uri_with_both(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 查询订单信息（对应 Java `WxMaXPayServiceImpl.queryOrder`）。
    async fn query_order(
        &self,
        request: &WxMaXPayQueryOrderRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::query_order_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 取消虚拟币充值订单（对应 Java `WxMaXPayServiceImpl.cancelCurrencyPay`，双签名）。
    async fn cancel_currency_pay(
        &self,
        request: &WxMaXPayCancelCurrencyPayRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayCancelCurrencyPayResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::cancel_currency_pay_url(config.as_ref());
        let uri = Self::sign_uri_with_both(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 通知发货（对应 Java `WxMaXPayServiceImpl.notifyProvideGoods`）。
    async fn notify_provide_goods(
        &self,
        request: &WxMaXPayNotifyProvideGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::notify_provide_goods_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed_bool(svc.as_ref(), &uri, &post_body).await
    }

    /// 赠送虚拟币（对应 Java `WxMaXPayServiceImpl.presentCurrency`）。
    async fn present_currency(
        &self,
        request: &WxMaXPayPresentCurrencyRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayPresentCurrencyResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::present_currency_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 道具直购（对应 Java `WxMaXPayServiceImpl.presentGoods`）。
    async fn present_goods(
        &self,
        request: &WxMaXPayPresentGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayPresentGoodsResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::present_goods_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 下载对账单（对应 Java `WxMaXPayServiceImpl.downloadBill`）。
    async fn download_bill(
        &self,
        request: &WxMaXPayDownloadBillRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayDownloadBillResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::download_bill_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 退款申请（对应 Java `WxMaXPayServiceImpl.refundOrder`）。
    async fn refund_order(
        &self,
        request: &WxMaXPayRefundOrderRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayRefundOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::refund_order_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 创建提现订单（对应 Java `WxMaXPayServiceImpl.createWithdrawOrder`）。
    async fn create_withdraw_order(
        &self,
        request: &WxMaXPayCreateWithdrawOrderRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayCreateWithdrawOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::create_withdraw_order_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 查询提现订单（对应 Java `WxMaXPayServiceImpl.queryWithdrawOrder`）。
    async fn query_withdraw_order(
        &self,
        request: &WxMaXPayQueryWithdrawOrderRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryWithdrawOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::query_withdraw_order_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 启动道具上传（对应 Java `WxMaXPayServiceImpl.startUploadGoods`）。
    async fn start_upload_goods(
        &self,
        request: &WxMaXPayStartUploadGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::start_upload_goods_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed_bool(svc.as_ref(), &uri, &post_body).await
    }

    /// 查询道具上传状态（对应 Java `WxMaXPayServiceImpl.queryUploadGoods`）。
    async fn query_upload_goods(
        &self,
        request: &WxMaXPayQueryUploadGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryUploadGoodsResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::query_upload_goods_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 启动道具发布（对应 Java `WxMaXPayServiceImpl.startPublishGoods`）。
    async fn start_publish_goods(
        &self,
        request: &WxMaXPayStartPublishGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::start_publish_goods_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed_bool(svc.as_ref(), &uri, &post_body).await
    }

    /// 查询道具发布状态（对应 Java `WxMaXPayServiceImpl.queryPublishGoods`）。
    async fn query_publish_goods(
        &self,
        request: &WxMaXPayQueryPublishGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryPublishGoodsResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::query_publish_goods_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 查询商家账户可提现余额（对应 Java `WxMaXPayServiceImpl.queryBizBalance`）。
    async fn query_biz_balance(
        &self,
        request: &WxMaXPayQueryBizBalanceRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryBizBalanceResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::query_biz_balance_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 查询广告金充值账户（对应 Java `WxMaXPayServiceImpl.queryTransferAccount`）。
    async fn query_transfer_account(
        &self,
        request: &WxMaXPayQueryTransferAccountRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryTransferAccountResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::query_transfer_account_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 查询广告金发放记录（对应 Java `WxMaXPayServiceImpl.queryAdverFunds`）。
    async fn query_adver_funds(
        &self,
        request: &WxMaXPayQueryAdverFundsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryAdverFundsResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::query_adver_funds_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 充值广告金（对应 Java `WxMaXPayServiceImpl.createFundsBill`）。
    async fn create_funds_bill(
        &self,
        request: &WxMaXPayCreateFundsBillRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayCreateFundsBillResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::create_funds_bill_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 绑定广告金充值账户（对应 Java `WxMaXPayServiceImpl.bindTransferAccount`）。
    async fn bind_transfer_account(
        &self,
        request: &WxMaXPayBindTransferAccountRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::bind_transfer_account_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 查询广告金充值记录（对应 Java `WxMaXPayServiceImpl.queryFundsBill`）。
    async fn query_funds_bill(
        &self,
        request: &WxMaXPayQueryFundsBillRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryFundsBillResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::query_funds_bill_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 查询广告金回收记录（对应 Java `WxMaXPayServiceImpl.queryRecoverBill`）。
    async fn query_recover_bill(
        &self,
        request: &WxMaXPayQueryRecoverBillRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryRecoverBillResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::query_recover_bill_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 获取投诉列表（对应 Java `WxMaXPayServiceImpl.getComplaintList`）。
    async fn get_complaint_list(
        &self,
        request: &WxMaXPayGetComplaintListRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayGetComplaintListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::get_complaint_list_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 获取投诉详情（对应 Java `WxMaXPayServiceImpl.getComplaintDetail`）。
    async fn get_complaint_detail(
        &self,
        request: &WxMaXPayGetComplaintDetailRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayGetComplaintDetailResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::get_complaint_detail_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 获取协商历史（对应 Java `WxMaXPayServiceImpl.getNegotiationHistory`）。
    async fn get_negotiation_history(
        &self,
        request: &WxMaXPayGetNegotiationHistoryRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayGetNegotiationHistoryResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::get_negotiation_history_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 回复用户（对应 Java `WxMaXPayServiceImpl.responseComplaint`）。
    async fn response_complaint(
        &self,
        request: &WxMaXPayResponseComplaintRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::response_complaint_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 完成投诉处理（对应 Java `WxMaXPayServiceImpl.completeComplaint`）。
    async fn complete_complaint(
        &self,
        request: &WxMaXPayCompleteComplaintRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::complete_complaint_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 上传媒体文件（对应 Java `WxMaXPayServiceImpl.uploadVpFile`）。
    async fn upload_vp_file(
        &self,
        request: &WxMaXPayUploadVpFileRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayUploadVpFileResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::upload_vp_file_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 获取微信支付反馈投诉图片的签名头部（对应 Java
    /// `WxMaXPayServiceImpl.getUploadFileSign`）。
    async fn get_upload_file_sign(
        &self,
        request: &WxMaXPayGetUploadFileSignRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayGetUploadFileSignResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::get_upload_file_sign_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }

    /// 下载广告金对应的商户订单信息（对应 Java
    /// `WxMaXPayServiceImpl.downloadAdverfundsOrder`）。
    async fn download_adverfunds_order(
        &self,
        request: &WxMaXPayDownloadAdverfundsOrderRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayDownloadAdverfundsOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let post_body = Self::to_json(request)?;
        let config = svc.wx_ma_config();
        let url = xpay_url::download_adverfunds_order_url(config.as_ref());
        let uri = Self::sign_uri_with_pay(&url, &post_body, sig_params);
        Self::post_signed(svc.as_ref(), &uri, &post_body).await
    }
}
