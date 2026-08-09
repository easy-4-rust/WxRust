//! 资金服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxChannelFundServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_channel_fund_service::WxChannelFundService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::fund::bank::{
    BankCityResponse, BankInfoResponse, BankListResponse, BankProvinceResponse, BankSearchParam,
    BranchInfoResponse, BranchSearchParam,
};
use crate::bean::fund::qrcode::{QrCheckResponse, QrCodeResponse};
use crate::bean::fund::{
    AccountInfo, AccountInfoParam, AccountInfoResponse, BalanceInfoResponse, FlowListResponse,
    FundsFlowResponse, FundsListParam, WithdrawDetailResponse, WithdrawListParam,
    WithdrawListResponse, WithdrawSubmitParam, WithdrawSubmitResponse,
};
use crate::enums::url_funds::{
    CHECK_QRCODE_URL, GET_BALANCE_FLOW_DETAIL_URL, GET_BALANCE_FLOW_LIST_URL, GET_BALANCE_URL,
    GET_BANK_ACCOUNT_URL, GET_BANK_BY_NUM_URL, GET_BANK_LIST_URL, GET_CITY_URL, GET_PROVINCE_URL,
    GET_QRCODE_URL, GET_SUB_BANK_URL, GET_WITHDRAW_DETAIL_URL, GET_WITHDRAW_LIST_URL,
    SET_BANK_ACCOUNT_URL, WITHDRAW_URL,
};

/// 资金服务实现（对应 Java `WxChannelFundServiceImpl`）。
pub struct WxChannelFundServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxChannelFundServiceImpl {
    /// 构建资金服务（对应 Java `new WxChannelFundServiceImpl(shopService)`）。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }

    /// 发送 POST 请求并解析响应（对应 Java `shopService.post(url, json)` +
    /// `ResponseUtils.decode(resJson, clazz)`）。
    ///
    /// errcode 校验由执行引擎在响应处完成（Rust `handle_response` / Java
    /// `SimplePostRequestExecutor.handleResponse` 抛 `WxErrorException` 同语义），
    /// 此处仅反序列化。
    async fn post_as<T>(
        svc: &dyn WxChannelService,
        url: &str,
        post_data: &str,
    ) -> Result<T, WxErrorException>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = svc.post(url, post_data).await?;
        serde_json::from_str(&response).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxChannelFundService for WxChannelFundServiceImpl {
    /// 获取账户余额（对应 Java `getBalance`，POST 空对象 `{}`）。
    async fn get_balance(&self) -> Result<BalanceInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        Self::post_as(svc.as_ref(), GET_BALANCE_URL, "{}").await
    }

    /// 获取结算账户（对应 Java `getBankAccount`，POST 空对象 `{}`）。
    async fn get_bank_account(&self) -> Result<AccountInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        Self::post_as(svc.as_ref(), GET_BANK_ACCOUNT_URL, "{}").await
    }

    /// 获取资金流水详情（对应 Java `getFundsFlowDetail`，请求体
    /// `{"flow_id":"..."}` 逐字对齐）。
    async fn get_funds_flow_detail(
        &self,
        flow_id: String,
    ) -> Result<FundsFlowResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"flow_id\":\"{flow_id}\"}}");
        Self::post_as(svc.as_ref(), GET_BALANCE_FLOW_DETAIL_URL, &req_json).await
    }

    /// 获取资金流水列表（对应 Java `listFundsFlow(FundsListParam)`）。
    async fn list_funds_flow(
        &self,
        param: FundsListParam,
    ) -> Result<FlowListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_BALANCE_FLOW_LIST_URL, &req_json).await
    }

    /// 获取提现记录（对应 Java `getWithdrawDetail`，请求体 `{"withdraw_id":"..."}`）。
    async fn get_withdraw_detail(
        &self,
        withdraw_id: String,
    ) -> Result<WithdrawDetailResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"withdraw_id\":\"{withdraw_id}\"}}");
        Self::post_as(svc.as_ref(), GET_WITHDRAW_DETAIL_URL, &req_json).await
    }

    /// 获取提现记录列表（对应 Java `listWithdraw(Integer, Integer, Long, Long)`，
    /// 内部构造 `WithdrawListParam`；null 参数以默认值表达，见
    /// `WithdrawListParam` 生成结构）。
    async fn list_withdraw(
        &self,
        page_num: Option<i32>,
        page_size: Option<i32>,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<WithdrawListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = WithdrawListParam {
            page_num: page_num.unwrap_or(0),
            page_size: page_size.unwrap_or(0),
            start_time: start_time.unwrap_or(0),
            end_time: end_time.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_WITHDRAW_LIST_URL, &req_json).await
    }

    /// 修改结算账户（对应 Java `setBankAccount(AccountInfo)`，内部构造
    /// `AccountInfoParam`，请求体 `{"account_info":{...}}`）。
    async fn set_bank_account(
        &self,
        account_info: AccountInfo,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = AccountInfoParam { account_info };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), SET_BANK_ACCOUNT_URL, &req_json).await
    }

    /// 商户提现（对应 Java `submitWithdraw(Integer, String, String)`，内部构造
    /// `WithdrawSubmitParam`，请求体 `{"amount":..,"remark":"..","bank_memo":".."}`）。
    async fn submit_withdraw(
        &self,
        amount: Option<i32>,
        remark: String,
        bank_memo: String,
    ) -> Result<WithdrawSubmitResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = WithdrawSubmitParam {
            amount: amount.unwrap_or(0),
            remark,
            bank_memo,
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), WITHDRAW_URL, &req_json).await
    }

    /// 根据卡号查银行信息（对应 Java `getBankInfoByCardNo`，请求体
    /// `{"account_number":"..."}`）。
    async fn get_bank_info_by_card_no(
        &self,
        account_number: String,
    ) -> Result<BankInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"account_number\":\"{account_number}\"}}");
        Self::post_as(svc.as_ref(), GET_BANK_BY_NUM_URL, &req_json).await
    }

    /// 搜索银行列表（对应 Java `searchBankList(Integer, Integer, String, Integer)`，
    /// 内部构造 `BankSearchParam`）。
    async fn search_bank_list(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        keywords: String,
        bank_type: Option<i32>,
    ) -> Result<BankListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = BankSearchParam {
            offset: offset.unwrap_or(0),
            limit: limit.unwrap_or(0),
            key_words: keywords,
            bank_type: bank_type.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_BANK_LIST_URL, &req_json).await
    }

    /// 查询城市列表（对应 Java `searchCityList`，请求体 `{"province_code":"..."}`）。
    async fn search_city_list(
        &self,
        province_code: String,
    ) -> Result<BankCityResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"province_code\":\"{province_code}\"}}");
        Self::post_as(svc.as_ref(), GET_CITY_URL, &req_json).await
    }

    /// 查询大陆银行省份列表（对应 Java `getProvinceList`，POST 空对象 `{}`）。
    async fn get_province_list(&self) -> Result<BankProvinceResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        Self::post_as(svc.as_ref(), GET_PROVINCE_URL, "{}").await
    }

    /// 查询支行列表（对应 Java `searchBranchList(String, String, Integer, Integer)`，
    /// 内部构造 `BranchSearchParam`）。
    async fn search_branch_list(
        &self,
        bank_code: String,
        city_code: String,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<BranchInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = BranchSearchParam {
            bank_code,
            city_code,
            offset: offset.unwrap_or(0),
            limit: limit.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SUB_BANK_URL, &req_json).await
    }

    /// 获取二维码（对应 Java `getQrCode`，请求体 `{"qrcode_ticket":"..."}`）。
    async fn get_qr_code(&self, qrcode_ticket: String) -> Result<QrCodeResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"qrcode_ticket\":\"{qrcode_ticket}\"}}");
        Self::post_as(svc.as_ref(), GET_QRCODE_URL, &req_json).await
    }

    /// 查询扫码状态（对应 Java `checkQrStatus`，请求体 `{"qrcode_ticket":"..."}`）。
    async fn check_qr_status(
        &self,
        qrcode_ticket: String,
    ) -> Result<QrCheckResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"qrcode_ticket\":\"{qrcode_ticket}\"}}");
        Self::post_as(svc.as_ref(), CHECK_QRCODE_URL, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 获取账户余额：请求路径/请求体/响应解析（对应 Java
    /// `WxChannelFundServiceImplTest#testGetBalance`）。
    #[tokio::test]
    async fn test_get_balance() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","available_amount":100,"pending_amount":50,"sub_mchid":"1900000001"}"#,
        );
        let sub = WxChannelFundServiceImpl::new(weak);
        let resp = sub.get_balance().await.unwrap();
        assert_eq!(resp.available_amount, 100);
        assert_eq!(resp.pending_amount, 50);
        assert_eq!(resp.sub_mchid, "1900000001");
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_BALANCE_URL);
        assert_eq!(body, "{}");
    }

    /// 商户提现：请求体字段与响应解析（对应 Java `submitWithdraw`）。
    #[tokio::test]
    async fn test_submit_withdraw() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","qrcode_ticket":"ticket_abc"}"#,
        );
        let sub = WxChannelFundServiceImpl::new(weak);
        let resp = sub
            .submit_withdraw(Some(1000), "测试提现".to_string(), "附言".to_string())
            .await
            .unwrap();
        assert_eq!(resp.qrcode_ticket, "ticket_abc");
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, WITHDRAW_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["amount"], 1000);
        assert_eq!(json["remark"], "测试提现");
        assert_eq!(json["bank_memo"], "附言");
    }

    /// 获取资金流水详情：字面量请求体逐字对齐 Java `{"flow_id":"..."}`。
    #[tokio::test]
    async fn test_get_funds_flow_detail() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","funds_flow":{"flow_id":"flow_001","amount":888}}"#,
        );
        let sub = WxChannelFundServiceImpl::new(weak);
        let resp = sub
            .get_funds_flow_detail("flow_001".to_string())
            .await
            .unwrap();
        assert_eq!(resp.funds_flow.flow_id, "flow_001");
        assert_eq!(resp.funds_flow.amount, 888);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_BALANCE_FLOW_DETAIL_URL);
        assert_eq!(body, r#"{"flow_id":"flow_001"}"#);
    }

    /// 获取提现记录列表：`Option` 参数默认值构造 + 响应解析。
    #[tokio::test]
    async fn test_list_withdraw() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","withdraw_ids":["w1","w2"]}"#,
        );
        let sub = WxChannelFundServiceImpl::new(weak);
        let resp = sub
            .list_withdraw(Some(1), Some(10), Some(1700000000), Some(1700000100))
            .await
            .unwrap();
        assert_eq!(resp.withdraw_ids, vec!["w1".to_string(), "w2".to_string()]);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_WITHDRAW_LIST_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["page_num"], 1);
        assert_eq!(json["page_size"], 10);
        assert_eq!(json["start_time"], 1700000000);
    }

    /// 业务错误码响应透传为 Err（对应 Java 执行器对 errcode!=0 抛
    /// `WxErrorException` 的语义，测试经由 mock 执行器模拟）。
    #[tokio::test]
    async fn test_business_error_propagates() {
        // 注意：`svc` 须保持存活直至调用结束（弱引用升级语义）
        let (svc, weak) = test_support::build_service(r#"{"errcode":93001,"errmsg":"余额不足"}"#);
        let sub = WxChannelFundServiceImpl::new(weak);
        let err = sub.get_balance().await.unwrap_err();
        assert_eq!(err.error_code(), Some(93001));
        let (url, _) = test_support::last_request(&svc);
        assert_eq!(url, GET_BALANCE_URL);
    }
}
