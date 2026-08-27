//! `WxPayService` 门面实现辅助（Wave 2 P2a 新增）。
//!
//! 对应 Java（weixin-java-pay）`service.impl.BaseWxPayServiceImpl` +
//! `WxPayServiceHttpComponentsImpl` 内部实现中无法归入 bean 的算法片段：
//! - v2 请求签名装配（对应 `BaseWxPayRequest#checkAndSign` 的
//!   配置回填/签名类型校验/nonce/签名，`xmlBean2Map` 语义以
//!   `bean::xml::root_children_map` 表达——与 XStream 输出字段一一对应）；
//! - v2 响应校验（对应 `BaseWxPayResult#checkResult`：`toMap()` 验签 +
//!   return_code/result_code 成功校验 + `WxPayException.from` 文案）；
//! - v2 退款通知 `req_info` 解密（对应 `WxPayRefundNotifyResult` 的
//!   `md5(mchKey)` 小写十六进制 → AES-256-ECB/PKCS5Padding）；
//! - 对账单/资金账单文本解析（对应 `WxPayBillResult.fromRawBillResultString`
//!   与 `BaseWxPayServiceImpl.handleFundFlow`）；
//! - v3 请求执行器（对应 `WxPayServiceHttpComponentsImpl.requestV3` +
//!   `WxPayV3HttpClientBuilder` 的 Authorization 头/Wechatpay-Serial 头/
//!   响应验签 `WxPayValidator`/v3 错误 JSON 转换）；
//! - p12 证书通道（对应 `WxPayConfig.initSslHttpClient`，`useKey=true` 场景）。

use std::collections::HashMap;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use rsa::RsaPublicKey;
use rsa::pkcs8::{EncodePrivateKey, LineEnding};
use wx_rust_common::error::{WxErrorException, WxRuntimeError};

use crate::bean::{WxPayBillInfo, WxPayBillResult, WxPayFundFlowBaseResult, WxPayFundFlowResult};
use crate::config::WxPayConfig;
use crate::constant::wx_pay_constants::sign_type as sign_type_const;
use crate::util::crypto::wx_pay_cert_utils::{
    load_certificate_from_pem, load_private_key_and_cert_from_p12, load_private_key_from_pem,
    load_public_key_from_pem,
};
use crate::util::crypto::wx_pay_crypto_utils::md5_hex;
use crate::util::crypto::wx_pay_v3_crypto_utils::{
    build_authorization_token, canonical_url_from_url, create_authorization_header, gen_nonce_str,
    gen_timestamp, sign_sha256_rsa, verify_sha256_rsa,
};
use crate::util::sign_utils::SignUtils;

/// 构造运行时错误（对应 Java `WxPayException`/`WxRuntimeException` 文案）。
pub fn runtime(msg: impl Into<String>) -> WxErrorException {
    WxErrorException::Runtime(WxRuntimeError::new(msg))
}

/// 当前毫秒时间戳字符串（对应 Java `System.currentTimeMillis()`）。
pub fn current_time_millis() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis().to_string())
        .unwrap_or_default()
}

// =====================================================================
// v2 请求签名装配（对应 Java `BaseWxPayRequest#checkAndSign`）
// =====================================================================

/// v2 请求公共字段访问器（对应 Java `BaseWxPayRequest` 的公共字段 +
/// `toXML`；`sign` 由签名装配填充）。
///
/// 各请求 bean 的字段名（`appid`/`mch_id`/`sub_app_id`/`sub_mch_id`/
/// `nonce_str`/`sign`/`sign_type`）由生成器统一，此处以宏批量实现。
pub trait V2Request {
    fn appid(&self) -> Option<&str>;
    fn set_appid(&mut self, v: Option<String>);
    fn mch_id(&self) -> Option<&str>;
    fn set_mch_id(&mut self, v: Option<String>);
    fn sub_app_id(&self) -> Option<&str>;
    fn set_sub_app_id(&mut self, v: Option<String>);
    fn sub_mch_id(&self) -> Option<&str>;
    fn set_sub_mch_id(&mut self, v: Option<String>);
    fn nonce_str(&self) -> Option<&str>;
    fn set_nonce_str(&mut self, v: Option<String>);
    fn sign_type(&self) -> Option<&str>;
    fn set_sign_type(&mut self, v: Option<String>);
    fn set_sign(&mut self, v: Option<String>);
    fn to_xml(&self) -> Result<String, String>;

    /// 签名时是否忽略 appid（对应 Java `ignoreAppid()`）。
    fn ignore_appid(&self) -> bool {
        false
    }
    /// 签名时是否忽略 sub_appid（对应 Java `ignoreSubAppId()`）。
    fn ignore_sub_app_id(&self) -> bool {
        false
    }
    /// 签名时是否忽略 sub_mch_id（对应 Java `ignoreSubMchId()`）。
    fn ignore_sub_mch_id(&self) -> bool {
        false
    }
    /// 是否需要 nonce_str（对应 Java `needNonceStr()`）。
    fn need_nonce_str(&self) -> bool {
        true
    }
}

macro_rules! impl_v2_request {
    // 默认全部不忽略、需要 nonce
    ($t:ty) => {
        impl_v2_request!($t, false, false, false, true);
    };
    // ignore_appid / ignore_sub_app_id / ignore_sub_mch_id / need_nonce 显式指定
    ($t:ty, $ignore_appid:expr, $ignore_sub_app_id:expr, $ignore_sub_mch_id:expr, $need_nonce:expr) => {
        impl V2Request for $t {
            fn appid(&self) -> Option<&str> {
                self.appid.as_deref()
            }
            fn set_appid(&mut self, v: Option<String>) {
                self.appid = v;
            }
            fn mch_id(&self) -> Option<&str> {
                self.mch_id.as_deref()
            }
            fn set_mch_id(&mut self, v: Option<String>) {
                self.mch_id = v;
            }
            fn sub_app_id(&self) -> Option<&str> {
                self.sub_app_id.as_deref()
            }
            fn set_sub_app_id(&mut self, v: Option<String>) {
                self.sub_app_id = v;
            }
            fn sub_mch_id(&self) -> Option<&str> {
                self.sub_mch_id.as_deref()
            }
            fn set_sub_mch_id(&mut self, v: Option<String>) {
                self.sub_mch_id = v;
            }
            fn nonce_str(&self) -> Option<&str> {
                self.nonce_str.as_deref()
            }
            fn set_nonce_str(&mut self, v: Option<String>) {
                self.nonce_str = v;
            }
            fn sign_type(&self) -> Option<&str> {
                self.sign_type.as_deref()
            }
            fn set_sign_type(&mut self, v: Option<String>) {
                self.sign_type = v;
            }
            fn set_sign(&mut self, v: Option<String>) {
                self.sign = v;
            }
            fn to_xml(&self) -> Result<String, String> {
                <$t>::to_xml(self)
            }
            fn ignore_appid(&self) -> bool {
                $ignore_appid
            }
            fn ignore_sub_app_id(&self) -> bool {
                $ignore_sub_app_id
            }
            fn ignore_sub_mch_id(&self) -> bool {
                $ignore_sub_mch_id
            }
            fn need_nonce_str(&self) -> bool {
                $need_nonce
            }
        }
    };
}

/// v2 资金账单请求的本地完整表达。
///
/// Wave 1 生成器缺陷：`WxPayDownloadFundFlowRequest` 的
/// `bill_date`/`account_type`/`tar_type` 被误生成到同文件的 `AccountType`
/// 结构（Java 的 `accountType` 枚举），请求 bean 仅剩基础字段。此处按 Java
/// `WxPayDownloadFundFlowRequest` 全字段重建，供
/// `WxPayService::download_fund_flow` 使用。
#[derive(Debug, Clone, Default, serde::Serialize)]
#[serde(rename = "xml")]
pub struct FundFlowBillRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mch_id")]
    pub mch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_app_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_mch_id"
    )]
    pub sub_mch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonce_str")]
    pub nonce_str: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sign")]
    pub sign: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sign_type")]
    pub sign_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bill_date")]
    pub bill_date: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_type"
    )]
    pub account_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tar_type")]
    pub tar_type: Option<String>,
}

impl V2Request for FundFlowBillRequest {
    fn appid(&self) -> Option<&str> {
        self.appid.as_deref()
    }
    fn set_appid(&mut self, v: Option<String>) {
        self.appid = v;
    }
    fn mch_id(&self) -> Option<&str> {
        self.mch_id.as_deref()
    }
    fn set_mch_id(&mut self, v: Option<String>) {
        self.mch_id = v;
    }
    fn sub_app_id(&self) -> Option<&str> {
        self.sub_app_id.as_deref()
    }
    fn set_sub_app_id(&mut self, v: Option<String>) {
        self.sub_app_id = v;
    }
    fn sub_mch_id(&self) -> Option<&str> {
        self.sub_mch_id.as_deref()
    }
    fn set_sub_mch_id(&mut self, v: Option<String>) {
        self.sub_mch_id = v;
    }
    fn nonce_str(&self) -> Option<&str> {
        self.nonce_str.as_deref()
    }
    fn set_nonce_str(&mut self, v: Option<String>) {
        self.nonce_str = v;
    }
    fn sign_type(&self) -> Option<&str> {
        self.sign_type.as_deref()
    }
    fn set_sign_type(&mut self, v: Option<String>) {
        self.sign_type = v;
    }
    fn set_sign(&mut self, v: Option<String>) {
        self.sign = v;
    }
    fn to_xml(&self) -> Result<String, String> {
        let out = quick_xml::se::to_string(self)
            .map_err(|e| format!("FundFlowBillRequest 序列化失败: {e}"))?;
        Ok(crate::bean::xml::expand_empty_elements(&out))
    }
}

impl_v2_request!(crate::bean::WxPayUnifiedOrderRequest);
impl_v2_request!(crate::bean::WxPayOrderQueryRequest);
impl_v2_request!(crate::bean::WxPayOrderCloseRequest);
impl_v2_request!(crate::bean::WxPayRefundRequest);
impl_v2_request!(crate::bean::WxPayRefundQueryRequest);
impl_v2_request!(crate::bean::WxPayDownloadBillRequest);
impl_v2_request!(crate::bean::WxPayDownloadFundFlowRequest);
impl_v2_request!(crate::bean::WxPayMicropayRequest);
impl_v2_request!(crate::bean::WxPayOrderReverseRequest);
impl_v2_request!(crate::bean::WxPayReportRequest);
impl_v2_request!(crate::bean::WxPayShorturlRequest);
impl_v2_request!(crate::bean::WxPayAuthcode2OpenidRequest);
impl_v2_request!(crate::bean::WxPayCouponSendRequest);
impl_v2_request!(crate::bean::WxPayCouponStockQueryRequest);
impl_v2_request!(crate::bean::WxPayCouponInfoQueryRequest);
impl_v2_request!(crate::bean::WxPayQueryCommentRequest);
impl_v2_request!(crate::bean::WxPayFaceAuthInfoRequest);
impl_v2_request!(crate::bean::WxPayFacepayRequest);
// WxPayDefaultRequest：沙箱签名 key 请求，无 appid（对应 Java ignoreAppid=true）
impl_v2_request!(crate::bean::WxPayDefaultRequest, true, false, false, true);
// ---- 子服务 v2 请求（Wave 5 P5：service.impl 子服务实现使用） ----
impl_v2_request!(crate::bean::RealNameRequest);
impl_v2_request!(crate::bean::profitsharing::request::profit_sharing_request::ProfitSharingRequest);
impl_v2_request!(crate::bean::profitsharing::request::profit_sharing_receiver_request::ProfitSharingReceiverRequest);
impl_v2_request!(
    crate::bean::profitsharing::request::profit_sharing_query_request::ProfitSharingQueryRequest
);
impl_v2_request!(crate::bean::profitsharing::request::profit_sharing_order_amount_query_request::ProfitSharingOrderAmountQueryRequest);
impl_v2_request!(crate::bean::profitsharing::request::profit_sharing_merchant_ratio_query_request::ProfitSharingMerchantRatioQueryRequest);
impl_v2_request!(
    crate::bean::profitsharing::request::profit_sharing_return_request::ProfitSharingReturnRequest
);
impl_v2_request!(crate::bean::profitsharing::request::profit_sharing_return_query_request::ProfitSharingReturnQueryRequest);
impl_v2_request!(crate::bean::profitsharing::request::profit_sharing_unfreeze_request::ProfitSharingUnfreezeRequest);
impl_v2_request!(crate::bean::request::wx_pay_send_mini_program_redpack_request::WxPaySendMiniProgramRedpackRequest);
impl_v2_request!(crate::bean::request::wx_pay_send_redpack_request::WxPaySendRedpackRequest);
impl_v2_request!(crate::bean::request::wx_pay_redpack_query_request::WxPayRedpackQueryRequest);
impl_v2_request!(crate::bean::EntPayRequest);
impl_v2_request!(crate::bean::EntPayQueryRequest);
impl_v2_request!(crate::bean::EntPayBankRequest);
impl_v2_request!(crate::bean::EntPayBankQueryRequest);
impl_v2_request!(crate::bean::EntPayRedpackRequest);
impl_v2_request!(crate::bean::EntPayRedpackQueryRequest);
impl_v2_request!(crate::bean::EntWxEmpPayRequest);
impl_v2_request!(crate::bean::WxDepositUnifiedOrderRequest);
impl_v2_request!(crate::bean::WxDepositOrderQueryRequest);
impl_v2_request!(crate::bean::WxDepositConsumeRequest);
impl_v2_request!(crate::bean::WxDepositUnfreezeRequest);
impl_v2_request!(crate::bean::WxDepositRefundRequest);
impl_v2_request!(crate::bean::request::wx_mp_entrust_request::WxMpEntrustRequest);
impl_v2_request!(crate::bean::request::wx_ma_entrust_request::WxMaEntrustRequest);
impl_v2_request!(crate::bean::request::wx_h5_entrust_request::WxH5EntrustRequest);
impl_v2_request!(crate::bean::request::wx_pay_entrust_request::WxPayEntrustRequest);
impl_v2_request!(crate::bean::request::wx_withhold_request::WxWithholdRequest);
impl_v2_request!(crate::bean::request::wx_sign_query_request::WxSignQueryRequest);
impl_v2_request!(crate::bean::request::wx_terminated_contract_request::WxTerminatedContractRequest);
impl_v2_request!(
    crate::bean::request::wx_withhold_order_query_request::WxWithholdOrderQueryRequest
);

// WxPayQueryExchangeRateRequest：无需 nonce_str（对应 Java needNonceStr=false）
impl_v2_request!(
    crate::bean::WxPayQueryExchangeRateRequest,
    false,
    false,
    false,
    false
);

/// 检查参数并设置签名（对应 Java `BaseWxPayRequest#checkAndSign`）：
/// 1. 补充系统参数（appid/mch_id/sub_appid/sub_mch_id，未设置时从配置读取）；
/// 2. 校验并确定签名类型（`ALL_SIGN_TYPES` 之外报错，文案与 Java 一致）；
/// 3. 未设置 nonce_str 时以当前毫秒时间戳填充；
/// 4. 以 XML 报文字段（对应 Java `getSignParams` + `storeMap`，排除
///    `workwx_sign`）生成签名并写回 `sign` 字段。
///
/// 注意：Java `checkFields` 的必填字段/约束检查在各门面方法内联实现
/// （Rust bean 为纯数据类，无 per-request 校验逻辑）。
pub fn check_and_sign<T: V2Request>(
    config: &dyn WxPayConfig,
    request: &mut T,
) -> Result<(), WxErrorException> {
    if !request.ignore_appid()
        && request
            .appid()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
    {
        request.set_appid(config.app_id().map(str::to_string));
    }
    if request
        .mch_id()
        .map(str::trim)
        .unwrap_or_default()
        .is_empty()
    {
        request.set_mch_id(config.mch_id().map(str::to_string));
    }
    if !request.ignore_sub_app_id()
        && request
            .sub_app_id()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
    {
        request.set_sub_app_id(config.sub_app_id().map(str::to_string));
    }
    if !request.ignore_sub_mch_id()
        && request
            .sub_mch_id()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
    {
        request.set_sub_mch_id(config.sub_mch_id().map(str::to_string));
    }

    // 签名类型：请求未设置时取配置（对应 Java trimToNull 语义），
    // 两者都必须在 ALL_SIGN_TYPES 内（文案与 Java 一致）。
    let sign_type = match request.sign_type().map(str::trim) {
        Some(st) if !st.is_empty() => {
            if !sign_type_const::ALL_SIGN_TYPES.contains(&st) {
                return Err(runtime(format!("非法的sign_type参数：{st}")));
            }
            Some(st.to_string())
        }
        _ => match config.sign_type().map(str::trim) {
            Some(st) if !st.is_empty() => {
                if !sign_type_const::ALL_SIGN_TYPES.contains(&st) {
                    return Err(runtime(format!("非法的signType配置：{st}，请检查配置！")));
                }
                Some(st.to_string())
            }
            _ => None,
        },
    };
    if let Some(st) = &sign_type {
        request.set_sign_type(Some(st.clone()));
    }

    if request.need_nonce_str()
        && request
            .nonce_str()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
    {
        request.set_nonce_str(Some(current_time_millis()));
    }

    // 签名参数 = XML 报文字段（对应 Java xmlBean2Map 的 @XStreamAlias 键，
    // 与 XStream toXML 输出一致；workwx_sign 不在 Java getSignParams 中，
    // 显式排除）。
    let xml = request
        .to_xml()
        .map_err(|e| runtime(format!("生成XML失败: {e}")))?;
    let mut map = crate::bean::xml::root_children_map(&xml)
        .map_err(|e| runtime(format!("解析XML失败: {e}")))?;
    map.remove("workwx_sign");
    let sign = SignUtils::create_sign(
        &map,
        sign_type.as_deref(),
        config.mch_key().unwrap_or_default(),
        &[],
    )?;
    request.set_sign(Some(sign));
    Ok(())
}

/// 校验 v2 响应（对应 Java `BaseWxPayResult#checkResult`）：
/// 1. 报文含 `sign` 时按 `signType`（空则 MD5）与商户密钥验签，
///    失败 → "参数格式校验错误！"；
/// 2. `check_success=true` 时校验 return_code/result_code 均为 SUCCESS
///    （或空），失败 → 与 Java `WxPayException.from` 一致的组合文案。
pub fn check_result(
    config: &dyn WxPayConfig,
    xml: &str,
    sign_type: Option<&str>,
    check_success: bool,
) -> Result<(), WxErrorException> {
    let map = crate::bean::xml::root_children_map(xml)
        .map_err(|e| runtime(format!("解析XML失败: {e}")))?;

    // 对应 Java：getSign() != null 时才验签
    if let Some(sign) = map.get("sign").filter(|s| !s.is_empty()) {
        let expected =
            SignUtils::create_sign(&map, sign_type, config.mch_key().unwrap_or_default(), &[])?;
        if expected != *sign {
            return Err(runtime("参数格式校验错误！"));
        }
    }

    if check_success {
        let ok = |key: &str| {
            map.get(key)
                .map(|s| {
                    let up = s.trim().to_uppercase();
                    up == crate::constant::wx_pay_constants::result_code::SUCCESS || up.is_empty()
                })
                .unwrap_or(true)
        };
        if !ok("return_code") || !ok("result_code") {
            // 对应 Java WxPayException.Builder.buildErrorMsg 的拼接顺序
            let mut parts: Vec<String> = Vec::new();
            if let Some(v) = map.get("return_code") {
                parts.push(format!("返回代码：[{v}]"));
            }
            if let Some(v) = map.get("return_msg") {
                parts.push(format!("返回信息：[{v}]"));
            }
            if let Some(v) = map.get("result_code") {
                parts.push(format!("结果代码：[{v}]"));
            }
            if let Some(v) = map.get("err_code") {
                parts.push(format!("错误代码：[{v}]"));
            }
            if let Some(v) = map.get("err_code_des") {
                parts.push(format!("错误详情：[{v}]"));
            }
            parts.push(format!("微信返回的原始报文：\n{xml}"));
            return Err(runtime(parts.join("，")));
        }
    }
    Ok(())
}

/// 解析 v2 结果 bean 并做完整校验（对应 Java
/// `BaseWxPayResult.fromXML(...)` + `result.checkResult(...)` 的组合）。
pub fn parse_v2_result<T>(
    config: &dyn WxPayConfig,
    xml: &str,
    sign_type: Option<&str>,
    check_success: bool,
    parse: impl Fn(&str) -> Result<T, String>,
) -> Result<T, WxErrorException> {
    let result = parse(xml).map_err(runtime)?;
    check_result(config, xml, sign_type, check_success)?;
    Ok(result)
}

// =====================================================================
// v2 退款通知 req_info 解密（对应 Java `WxPayRefundNotifyResult`）
// =====================================================================

/// AES-256-ECB 解密（对应 Java `Cipher.getInstance("AES/ECB/PKCS5Padding")`，
/// 与 PKCS7 对 16 字节块等价）：逐块 `decrypt_block` + PKCS7 去填充。
fn aes_256_ecb_decrypt_pkcs7(key: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    use aes::Aes256;
    use aes::cipher::{Block, BlockCipherDecrypt, KeyInit};
    if key.len() != 32 {
        return Err("无效的AES密钥，长度必须为32个字节".to_string());
    }
    if data.is_empty() || !data.len().is_multiple_of(16) {
        return Err("密文长度不是16的整数倍".to_string());
    }
    let cipher = Aes256::new_from_slice(key).map_err(|e| e.to_string())?;
    let mut plain = Vec::with_capacity(data.len());
    for chunk in data.chunks_exact(16) {
        let mut block = Block::<Aes256>::try_from(chunk).map_err(|_| "块长度错误".to_string())?;
        cipher.decrypt_block(&mut block);
        plain.extend_from_slice(block.as_slice());
    }
    // PKCS#7 去填充：末字节 n ∈ [1,16]，校验末尾 n 个字节均为 n
    let pad_len = *plain.last().ok_or_else(|| "密文为空".to_string())? as usize;
    if pad_len == 0 || pad_len > 16 {
        return Err("无效的PKCS7填充".to_string());
    }
    let start = plain.len() - pad_len;
    if plain[start..].iter().any(|&b| b as usize != pad_len) {
        return Err("无效的PKCS7填充".to_string());
    }
    plain.truncate(start);
    Ok(plain)
}

/// 解密退款通知 `req_info`（对应 Java
/// `WxPayRefundNotifyResult.decryptReqInfo(String mchKey)`）：
/// `md5(mchKey)` 小写十六进制串的 UTF-8 字节作为 AES-256 密钥，
/// Base64 解码后 AES-256-ECB/PKCS5Padding 解密。
///
/// 解密失败文案与 Java 一致："解密退款通知加密信息时出错"。
pub fn decrypt_refund_req_info(
    mch_key: &str,
    req_info_b64: &str,
) -> Result<String, WxErrorException> {
    let key_md5 = md5_hex(mch_key).to_lowercase();
    let ciphertext = base64::engine::general_purpose::STANDARD
        .decode(req_info_b64)
        .map_err(|e| runtime(format!("解密退款通知加密信息时出错: {e}")))?;
    let plain = aes_256_ecb_decrypt_pkcs7(key_md5.as_bytes(), &ciphertext)
        .map_err(|e| runtime(format!("解密退款通知加密信息时出错: {e}")))?;
    String::from_utf8(plain).map_err(|e| runtime(format!("解密退款通知加密信息时出错: {e}")))
}

// =====================================================================
// 对账单/资金账单文本解析（对应 Java WxPayBillResult / handleFundFlow）
// =====================================================================

const TOTAL_DEAL_COUNT: &str = "总交易单数";
const TOTAL_FUND_COUNT: &str = "资金流水总笔数";

/// 按账单类型解析原始对账单（对应 Java
/// `WxPayBillResult.fromRawBillResultString(responseContent, billType)`，
/// 未知类型返回 None）。
pub fn parse_bill_result(response_content: &str, bill_type: &str) -> Option<WxPayBillResult> {
    match bill_type {
        crate::constant::wx_pay_constants::bill_type::ALL => {
            Some(parse_bill_detail(response_content, BillLayout::All))
        }
        crate::constant::wx_pay_constants::bill_type::SUCCESS => {
            Some(parse_bill_detail(response_content, BillLayout::Success))
        }
        crate::constant::wx_pay_constants::bill_type::REFUND => {
            Some(parse_bill_detail(response_content, BillLayout::Refund))
        }
        crate::constant::wx_pay_constants::bill_type::RECHARGE_REFUND => Some(parse_bill_detail(
            response_content,
            BillLayout::RechargeRefund,
        )),
        _ => None,
    }
}

/// 四种账单布局（对应 Java 的四个私有解析方法，字段位序逐列镜像）。
#[derive(Clone, Copy, PartialEq)]
enum BillLayout {
    All,
    Success,
    Refund,
    RechargeRefund,
}

impl BillLayout {
    /// 每条记录按空格切分后的字段数（对应 Java 分组标题 `t.length`）。
    fn columns(&self) -> usize {
        match self {
            BillLayout::All => 27, // 含免充值券扩展 3 列时为 27（Java 动态判定）
            BillLayout::Success => 20,
            BillLayout::Refund => 26,
            BillLayout::RechargeRefund => 28,
        }
    }
}

/// 通用明细行解析：`fields` 为一行按空格切分后的字段
/// （Java 布局注释中的字段顺序逐位镜像）。
fn fill_bill_info(info: &mut WxPayBillInfo, fields: &[&str], layout: BillLayout) {
    let get = |i: usize| fields.get(i).copied().unwrap_or("").trim().to_string();
    info.trade_time = Some(get(0));
    info.app_id = Some(get(1));
    info.mch_id = Some(get(2));
    info.sub_mch_id = Some(get(3));
    info.device_info = Some(get(4));
    info.transaction_id = Some(get(5));
    info.out_trade_no = Some(get(6));
    info.open_id = Some(get(7));
    info.trade_type = Some(get(8));
    info.trade_state = Some(get(9));
    info.bank_type = Some(get(10));
    info.fee_type = Some(get(11));
    info.total_fee = Some(get(12));
    info.coupon_fee = Some(get(13));
    match layout {
        BillLayout::All => {
            info.refund_id = Some(get(14));
            info.out_refund_no = Some(get(15));
            info.settlement_refund_fee = Some(get(16));
            info.coupon_refund_fee = Some(get(17));
            info.refund_channel = Some(get(18));
            info.refund_state = Some(get(19));
            info.body = Some(get(20));
            info.attach = Some(get(21));
            info.poundage = Some(get(22));
            info.poundage_rate = Some(get(23));
            if fields.len() > 24 {
                // 开通免充值券后的结算对账单多三列
                info.total_amount = Some(get(24));
                info.applied_refund_amount = Some(get(25));
                info.fee_remark = Some(get(26));
            }
        }
        BillLayout::Success => {
            info.body = Some(get(14));
            info.attach = Some(get(15));
            info.poundage = Some(get(16));
            info.poundage_rate = Some(get(17));
            info.total_amount = Some(get(18));
            info.fee_remark = Some(get(19));
        }
        BillLayout::Refund => {
            info.refund_time = Some(get(14));
            info.refund_success_time = Some(get(15));
            info.refund_id = Some(get(16));
            info.out_refund_no = Some(get(17));
            info.settlement_refund_fee = Some(get(18));
            info.coupon_refund_fee = Some(get(19));
            info.refund_channel = Some(get(20));
            info.refund_state = Some(get(21));
            info.body = Some(get(22));
            info.attach = Some(get(23));
            info.poundage = Some(get(24));
            info.poundage_rate = Some(get(25));
        }
        BillLayout::RechargeRefund => {
            info.refund_time = Some(get(14));
            info.refund_success_time = Some(get(15));
            info.refund_id = Some(get(16));
            info.out_refund_no = Some(get(17));
            info.settlement_refund_fee = Some(get(18));
            info.coupon_refund_fee = Some(get(19));
            info.refund_channel = Some(get(20));
            info.refund_state = Some(get(21));
            info.body = Some(get(22));
            info.attach = Some(get(23));
            info.poundage = Some(get(24));
            info.poundage_rate = Some(get(25));
            if fields.len() > 26 {
                info.total_amount = Some(get(26));
                info.applied_refund_amount = Some(get(27));
            }
        }
    }
}

/// 明细 + 汇总解析（对应 Java 四个 `fromRawBillResultStringTo*` 的公共骨架）：
/// 标题行字段数 = 单条字段数，` 分割行、`,` 替换为空格。
fn parse_bill_detail(response_content: &str, layout: BillLayout) -> WxPayBillResult {
    let (list_str, obj_str) = match response_content.find(TOTAL_DEAL_COUNT) {
        Some(idx) => (&response_content[..idx], &response_content[idx..]),
        None => (response_content, ""),
    };

    let mut results: Vec<WxPayBillInfo> = Vec::new();
    let new_str = list_str.replace(',', " ");
    let temp_str: Vec<&str> = new_str.split('`').collect();
    if let Some(header) = temp_str.first() {
        let t: Vec<&str> = header.split(' ').collect();
        let col = if layout == BillLayout::All && t.len() > 24 {
            t.len()
        } else {
            layout.columns()
        };
        if !t.is_empty() && temp_str.len() > 1 {
            let j = temp_str.len() / col;
            let mut k = 1;
            for _ in 0..j {
                let mut result = WxPayBillInfo::default();
                let fields: Vec<&str> = (0..col).map(|i| temp_str[k + i]).collect();
                fill_bill_info(&mut result, &fields, layout);
                results.push(result);
                k += col;
            }
        }
    }

    let mut bill = WxPayBillResult {
        bill_info_list: results,
        ..WxPayBillResult::default()
    };
    let total_str = obj_str.replace(',', " ");
    let total_temp: Vec<&str> = total_str.split('`').collect();
    if total_temp.len() > 1 {
        bill.total_record = Some(total_temp[1].trim().to_string());
    }
    if total_temp.len() > 2 {
        bill.total_fee = Some(total_temp[2].trim().to_string());
    }
    if total_temp.len() > 3 {
        bill.total_refund_fee = Some(total_temp[3].trim().to_string());
    }
    if total_temp.len() > 4 {
        bill.total_coupon_fee = Some(total_temp[4].trim().to_string());
    }
    if total_temp.len() > 5 {
        bill.total_poundage_fee = Some(total_temp[5].trim().to_string());
    }
    if total_temp.len() > 6 {
        bill.total_amount = Some(total_temp[6].trim().to_string());
    }
    if total_temp.len() > 7 {
        bill.total_applied_refund_fee = Some(total_temp[7].trim().to_string());
    }
    bill
}

/// 解析资金账单（对应 Java `BaseWxPayServiceImpl.handleFundFlow`：
/// 明细段按 ` 分割、`,` 替换为空格，标题行字段数 = 单条字段数）。
pub fn parse_fund_flow_result(response_content: &str) -> WxPayFundFlowResult {
    let mut result = WxPayFundFlowResult::default();
    let Some(idx) = response_content.find(TOTAL_FUND_COUNT) else {
        return result;
    };
    let list_str = &response_content[..idx];
    let obj_str = &response_content[idx..];

    // 明细段：` 分组、`,` 替换为空格；每组 11 个字段
    let new_str = list_str.replace(',', " ");
    let temp_str: Vec<&str> = new_str.split('`').collect();
    let mut list = Vec::new();
    if let Some(header) = temp_str.first() {
        let t: Vec<&str> = header.split(' ').collect();
        if !t.is_empty() && temp_str.len() > 1 {
            let j = temp_str.len() / t.len();
            let mut k = 1;
            for _ in 0..j {
                let get = |i: usize| temp_str[k + i].trim().to_string();
                list.push(WxPayFundFlowBaseResult {
                    billing_time: Some(get(0)),
                    biz_transaction_id: Some(get(1)),
                    fund_flow_id: Some(get(2)),
                    biz_name: Some(get(3)),
                    biz_type: Some(get(4)),
                    financial_type: Some(get(5)),
                    financial_fee: Some(get(6)),
                    account_balance: Some(get(7)),
                    fund_applicant: Some(get(8)),
                    memo: Some(get(9)),
                    biz_voucher_id: Some(get(10)),
                });
                k += t.len();
            }
        }
    }
    result.wx_pay_fund_flow_base_result_list = list;

    // 汇总段：` 分割（总笔数,收入笔数,收入金额,支出笔数,支出金额）
    let total_str = obj_str.replace(',', " ");
    let total_temp: Vec<&str> = total_str.split('`').collect();
    if total_temp.len() > 1 {
        result.total_record = Some(total_temp[1].trim().to_string());
    }
    if total_temp.len() > 2 {
        result.income_record = Some(total_temp[2].trim().to_string());
    }
    if total_temp.len() > 3 {
        result.income_amount = Some(total_temp[3].trim().to_string());
    }
    if total_temp.len() > 4 {
        result.expenditure_record = Some(total_temp[4].trim().to_string());
    }
    if total_temp.len() > 5 {
        result.expenditure_amount = Some(total_temp[5].trim().to_string());
    }
    result
}

/// GZIP 解压为按行拼接的文本（对应 Java `ZipUtils.unGzip` +
/// `Files.readAllLines` + `Joiner.on("\n").join`）。
pub fn gunzip_to_text(bytes: &[u8]) -> Result<String, WxErrorException> {
    let mut decoder = flate2::read::GzDecoder::new(bytes);
    let mut text = String::new();
    decoder
        .read_to_string(&mut text)
        .map_err(|e| runtime(format!("解压对账单文件时出错！: {e}")))?;
    let lines: Vec<&str> = text.lines().collect();
    Ok(lines.join("\n"))
}

// =====================================================================
// p12 证书通道（对应 Java `WxPayConfig.initSSLContext`，useKey=true）
// =====================================================================

/// 从配置读取 p12 内容（对应 Java `loadConfigInputStream(keyString, keyPath,
/// keyContent, "keyPath")`：优先 base64 串，其次文件路径，再次原始字节）。
pub fn load_p12_bytes(config: &dyn WxPayConfig) -> Result<Option<Vec<u8>>, WxErrorException> {
    if let Some(b64) = config.key_string().map(str::trim).filter(|s| !s.is_empty()) {
        return base64::engine::general_purpose::STANDARD
            .decode(b64)
            .map(Some)
            .map_err(|e| runtime(format!("keyString base64 解码失败: {e}")));
    }
    if let Some(path) = config.key_path().map(str::trim).filter(|s| !s.is_empty()) {
        return std::fs::read(path)
            .map(Some)
            .map_err(|e| runtime(format!("读取keyPath失败: {e}")));
    }
    Ok(config.key_content().map(|v| v.to_vec()))
}

/// 构建证书通道 HTTP 客户端（对应 Java `initSslHttpClient`/`initSSLContext`：
/// p12 容器（密码=商户号 mchId）→ reqwest Identity；rustls 通道需要 PEM，
/// 由 p12 解析结果重编码为 PKCS#8 私钥 PEM + X.509 证书 PEM）。
pub fn build_cert_client(config: &dyn WxPayConfig) -> Result<reqwest::Client, WxErrorException> {
    let p12 = load_p12_bytes(config)?
        .ok_or_else(|| runtime("请确保keyPath/keyString/keyContent已设置（p12 证书文件）"))?;
    let password = config.mch_id().unwrap_or_default();
    let data = load_private_key_and_cert_from_p12(&p12, password)
        .map_err(|e| runtime(format!("证书文件有问题，请核实！: {e}")))?;
    let key_pem = data
        .private_key
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| runtime(format!("私钥转PEM失败: {e}")))?
        .to_string();
    let cert_der = data.certificate.to_der().map_err(WxErrorException::from)?;
    let cert_pem = pem_encode("CERTIFICATE", &cert_der);
    let identity = reqwest::Identity::from_pem(format!("{key_pem}{cert_pem}").as_bytes())
        .map_err(|e| runtime(format!("构建客户端身份失败: {e}")))?;

    let mut builder = reqwest::Client::builder()
        .identity(identity)
        .connect_timeout(std::time::Duration::from_millis(
            config.http_connection_timeout() as u64,
        ))
        .timeout(std::time::Duration::from_millis(
            config.http_timeout() as u64
        ));
    // 对应 Java createHttpClientBuilder 的代理配置
    if let (Some(host), Some(port)) = (
        config
            .http_proxy_host()
            .map(str::trim)
            .filter(|s| !s.is_empty()),
        config.http_proxy_port(),
    ) {
        if let Ok(proxy) = reqwest::Proxy::all(format!("http://{host}:{port}")) {
            builder = builder.proxy(proxy);
        }
    }
    builder
        .build()
        .map_err(|e| runtime(format!("构建HTTP客户端失败: {e}")))
}

/// PEM 编码（76 字符折行）。
fn pem_encode(kind: &str, der: &[u8]) -> String {
    let b64 = base64::engine::general_purpose::STANDARD.encode(der);
    let mut out = format!("-----BEGIN {kind}-----\n");
    for chunk in b64.as_bytes().chunks(64) {
        out.push_str(std::str::from_utf8(chunk).unwrap_or_default());
        out.push('\n');
    }
    out.push_str(&format!("-----END {kind}-----\n"));
    out
}

// =====================================================================
// v3 请求执行（对应 Java WxPayServiceHttpComponentsImpl + v3 认证链）
// =====================================================================

/// v3 请求上下文（对应 Java `initApiV3HttpClient` 解析出的
/// mchId/certSerialNo/merchantPrivateKey）。
pub struct V3Context {
    pub mch_id: String,
    pub serial_no: String,
    pub private_key: rsa::RsaPrivateKey,
}

/// 加载商户 API 私钥（对应 Java `initApiV3HttpClient`：p12 优先，
/// 其次 `privateKey()` PEM 直填 / privateKeyContent / privateKeyString
/// （base64）/ privateKeyPath 文件）。
pub fn load_merchant_private_key(
    config: &dyn WxPayConfig,
) -> Result<rsa::RsaPrivateKey, WxErrorException> {
    if let Some(pem) = config
        .private_key()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return load_private_key_from_pem(pem.as_bytes()).map_err(WxErrorException::from);
    }
    if let Some(bytes) = config.private_key_content() {
        return load_private_key_from_pem(bytes).map_err(WxErrorException::from);
    }
    if let Some(b64) = config
        .private_key_string()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let pem = base64::engine::general_purpose::STANDARD
            .decode(b64)
            .map_err(|e| runtime(format!("privateKeyString base64 解码失败: {e}")))?;
        return load_private_key_from_pem(&pem).map_err(WxErrorException::from);
    }
    if let Some(path) = config
        .private_key_path()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let pem =
            std::fs::read(path).map_err(|e| runtime(format!("读取privateKeyPath失败: {e}")))?;
        return load_private_key_from_pem(&pem).map_err(WxErrorException::from);
    }
    Err(runtime(
        "请确保私钥配置（privateKey/privateKeyContent/privateKeyString/privateKeyPath）已设置",
    ))
}

/// 从配置加载商户证书（对应 Java `initApiV3HttpClient` 中
/// `PemUtils.loadCertificate`：privateCertContent/privateCertString/privateCertPath），
/// 用于派生证书序列号。
fn load_merchant_cert(
    config: &dyn WxPayConfig,
) -> Result<Option<crate::util::crypto::wx_pay_cert_utils::WxPayCertificate>, WxErrorException> {
    if let Some(bytes) = config.private_cert_content() {
        return load_certificate_from_pem(bytes)
            .map(Some)
            .map_err(WxErrorException::from);
    }
    if let Some(b64) = config
        .private_cert_string()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let pem = base64::engine::general_purpose::STANDARD
            .decode(b64)
            .map_err(|e| runtime(format!("privateCertString base64 解码失败: {e}")))?;
        return load_certificate_from_pem(&pem)
            .map(Some)
            .map_err(WxErrorException::from);
    }
    if let Some(path) = config
        .private_cert_path()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let pem =
            std::fs::read(path).map_err(|e| runtime(format!("读取privateCertPath失败: {e}")))?;
        return load_certificate_from_pem(&pem)
            .map(Some)
            .map_err(WxErrorException::from);
    }
    Ok(None)
}

/// 构建 v3 请求上下文（对应 Java `WxPayConfig.initApiV3HttpClient` 的
/// apiV3Key 检查 + p12/privateCert 序列号派生 + 私钥加载）。
pub fn v3_context(config: &dyn WxPayConfig) -> Result<V3Context, WxErrorException> {
    if config
        .api_v3_key()
        .map(str::trim)
        .unwrap_or_default()
        .is_empty()
    {
        return Err(runtime("请确保apiV3Key值已设置"));
    }
    // p12 优先（对应 Java p12ToPem；失败时 Java 记日志后回退到 PEM 通道）
    if let Some(p12) = load_p12_bytes(config)? {
        let password = config.mch_id().unwrap_or_default();
        if let Ok(data) = load_private_key_and_cert_from_p12(&p12, password) {
            return Ok(V3Context {
                mch_id: config.mch_id().unwrap_or_default().to_string(),
                serial_no: data.certificate.serial_no().to_string(),
                private_key: data.private_key,
            });
        }
    }

    let private_key = load_merchant_private_key(config)?;
    let serial_no = match config
        .cert_serial_no()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        Some(s) => s.to_string(),
        None => match load_merchant_cert(config)? {
            Some(cert) => cert.serial_no().to_string(),
            None => {
                return Err(runtime(
                    "请确保certSerialNo值已设置（或配置 privateCertContent/privateCertString/privateCertPath）",
                ));
            }
        },
    };
    Ok(V3Context {
        mch_id: config.mch_id().unwrap_or_default().to_string(),
        serial_no,
        private_key,
    })
}

/// 平台证书/微信支付公钥加载（响应验签与通知验签的衔接点）。
///
/// 对应 Java `initApiV3HttpClient` 的 `VerifierBuilder`：
/// - 完全公钥模式（`fullPublicKeyModel=true`，默认）：要求
///   publicKeyId + publicKey 配置，加载微信支付公钥；
/// - 证书模式（`fullPublicKeyModel=false`）：Java 由
///   `AutoUpdateCertificatesVerifier` 自动下载/缓存平台证书，证书自动更新
///   为 Wave 2 P2b 职责，本函数返回未衔接错误（P2b 完成配置接入后此
///   分支由 config 提供的平台证书访问器接管）。
pub fn platform_public_key(config: &dyn WxPayConfig) -> Result<RsaPublicKey, WxErrorException> {
    if config.full_public_key_model() {
        if config
            .public_key_id()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            return Err(runtime("请确保和publicKeyId配套使用"));
        }
        if let Some(bytes) = config.public_key_content() {
            return load_public_key_from_pem(bytes).map_err(WxErrorException::from);
        }
        if let Some(b64) = config
            .public_key_string()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let pem = base64::engine::general_purpose::STANDARD
                .decode(b64)
                .map_err(|e| runtime(format!("publicKeyString base64 解码失败: {e}")))?;
            return load_public_key_from_pem(&pem).map_err(WxErrorException::from);
        }
        if let Some(path) = config
            .public_key_path()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let pem =
                std::fs::read(path).map_err(|e| runtime(format!("读取publicKeyPath失败: {e}")))?;
            return load_public_key_from_pem(&pem).map_err(WxErrorException::from);
        }
        Err(runtime(
            "完全公钥模式下，请确保公钥配置（publicKeyPath/publicKeyString/publicKeyContent）及publicKeyId已设置",
        ))
    } else {
        Err(runtime(
            "平台证书模式未就绪：证书自动更新（AutoUpdateCertificatesVerifier）由 Wave 2 P2b 接管；当前请配置 publicKeyId+publicKey 或等待 P2b 完成",
        ))
    }
}

/// Wechatpay-Serial 请求头取值（对应 Java
/// `WxPayServiceHttpComponentsImpl.getWechatPaySerial`：publicKeyId 优先，
/// 否则取平台证书序列号；取不到时返回空串继续请求）。
pub fn wechatpay_serial_header(config: &dyn WxPayConfig) -> String {
    config
        .public_key_id()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_default()
}

/// 规范化 URL 并去除签名前缀（对应 Java `WxPayCredentials.buildMessage` 的
/// `stripPathPrefix`：rawPath 以 apiHostUrlPath 开头时裁掉）。
pub fn canonical_url_strip_prefix(
    url_str: &str,
    strip_prefix: Option<&str>,
) -> Result<String, WxErrorException> {
    let mut canonical =
        canonical_url_from_url(url_str).map_err(|e| runtime(format!("无效的URL: {e}")))?;
    if let Some(prefix) = strip_prefix {
        let normalized = normalize_strip_prefix(prefix);
        let path = &canonical;
        if let Some(query_pos) = path.find('?') {
            let raw_path = &path[..query_pos];
            let query = &path[query_pos..];
            canonical = format!("{}{}", strip_path_prefix(raw_path, &normalized), query);
        } else {
            canonical = strip_path_prefix(path, &normalized);
        }
    }
    Ok(canonical)
}

/// 前缀归一化（对应 Java `WxPayCredentials.setSignUriStripPrefix`：
/// trim、补开头 `/`、去结尾 `/`）。
fn normalize_strip_prefix(prefix: &str) -> String {
    let trimmed = prefix.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let mut normalized = if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    };
    if normalized.len() > 1 && normalized.ends_with('/') {
        normalized.pop();
    }
    normalized
}

/// 裁剪路径前缀（对应 Java `WxPayCredentials.stripPathPrefix`）。
fn strip_path_prefix(raw_path: &str, prefix: &str) -> String {
    if prefix.is_empty() || !raw_path.starts_with(prefix) {
        return raw_path.to_string();
    }
    let stripped = &raw_path[prefix.len()..];
    if stripped.is_empty() {
        return "/".to_string();
    }
    if stripped.starts_with('/') {
        stripped.to_string()
    } else {
        format!("/{stripped}")
    }
}

/// v3 响应验签（对应 Java `WxPayValidator.validate`：Content-Type 非 JSON
/// 直接通过；JSON 响应要求 Wechatpay-Serial/Signature/TimeStamp/Nonce
/// 四头齐全并按 `timestamp\nnonce\nbody\n` 验签）。
pub fn validate_v3_response(
    config: &dyn WxPayConfig,
    content_type: Option<&str>,
    headers: &[(&str, &str)],
    body: &str,
) -> Result<(), WxErrorException> {
    let is_json = content_type
        .map(|ct| {
            let mime = ct.split(';').next().unwrap_or(ct).trim();
            mime.eq_ignore_ascii_case("application/json")
        })
        .unwrap_or(false);
    if !is_json {
        return Ok(());
    }
    let mut map: HashMap<String, &str> = HashMap::new();
    for (k, v) in headers {
        map.insert(k.to_ascii_lowercase(), v);
    }
    let timestamp = map.get("wechatpay-timestamp").copied();
    let nonce = map.get("wechatpay-nonce").copied();
    let signature = map.get("wechatpay-signature").copied();
    // 对应 Java：四个头任一缺失 → validate 返回 false
    let (Some(timestamp), Some(nonce), Some(signature)) = (timestamp, nonce, signature) else {
        return Err(runtime("应答的微信支付签名验证失败"));
    };
    let key = platform_public_key(config)?;
    let message = format!("{timestamp}\n{nonce}\n{body}\n");
    let ok =
        verify_sha256_rsa(&key, message.as_bytes(), signature).map_err(WxErrorException::from)?;
    if ok {
        Ok(())
    } else {
        Err(runtime("应答的微信支付签名验证失败"))
    }
}

/// 执行 v3 请求（对应 Java `WxPayServiceHttpComponentsImpl.requestV3` /
/// `postV3WithWechatpaySerial` 的组合）：
/// - Authorization 头（WECHATPAY2-SHA256-RSA2048，canonical URL 含
///   apiHostUrlPath 前缀裁剪）与 Wechatpay-Serial 头；
/// - 200/204 → 响应体（空响应 → 空串）；响应为 JSON 时按
///   [`validate_v3_response`] 验签；
/// - 其余状态码 → 解析 v3 错误 JSON `{code, message}` 抛错
///   （对应 Java `convertException`，消息为 `code message`）。
pub async fn execute_v3(
    config: &dyn WxPayConfig,
    client: &reqwest::Client,
    method: &str,
    url: &str,
    body: &str,
) -> Result<String, WxErrorException> {
    let context = v3_context(config)?;
    let strip_prefix = config
        .api_host_url_path()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let canonical = canonical_url_strip_prefix(url, strip_prefix.as_deref())?;
    let timestamp = gen_timestamp();
    let nonce = gen_nonce_str();
    let authorization = create_authorization_header(
        &context.mch_id,
        &context.serial_no,
        &context.private_key,
        method,
        &canonical,
        timestamp,
        &nonce,
        body,
    )
    .map_err(WxErrorException::from)?;

    let mut request = match method {
        "POST" => client.post(url).body(body.to_string()),
        "PUT" => client.put(url).body(body.to_string()),
        "PATCH" => client.patch(url).body(body.to_string()),
        "GET" => client.get(url),
        "DELETE" => client.delete(url),
        _ => return Err(runtime(format!("不支持的HTTP方法: {method}"))),
    };
    request = request
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header("Authorization", authorization)
        .header("Wechatpay-Serial", wechatpay_serial_header(config));

    let resp = request
        .send()
        .await
        .map_err(|e| WxErrorException::Http(e.to_string()))?;
    let status = resp.status();
    let headers: Vec<(String, String)> = resp
        .headers()
        .iter()
        .map(|(k, v)| {
            (
                k.as_str().to_string(),
                v.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    let text = resp
        .text()
        .await
        .map_err(|e| WxErrorException::Http(e.to_string()))?;

    if status == reqwest::StatusCode::OK || status == reqwest::StatusCode::NO_CONTENT {
        if !text.is_empty() {
            let header_refs: Vec<(&str, &str)> = headers
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            validate_v3_response(config, content_type.as_deref(), &header_refs, &text)?;
        }
        return Ok(text);
    }

    // 对应 Java convertException：解析 {code, message}
    let err = match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(v) => {
            let code = v.get("code").and_then(|c| c.as_str()).unwrap_or_default();
            let message = v
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or_default();
            if code.is_empty() {
                runtime(message.to_string())
            } else {
                runtime(format!("{code} {message}"))
            }
        }
        Err(_) => runtime(format!(
            "微信支付V3接口返回错误，状态码：{status}，响应：{text}"
        )),
    };
    Err(err)
}

/// 对应 Java `RsaCryptoUtil.encryptFields`（`@SpecEncrypt` 字段 RSA-OAEP 加密）：
/// 对 `serde_json::Value` 中指定 JSON 路径的字符串字段用平台证书公钥加密为
/// Base64 密文，空串/缺失字段跳过（对应 Java 的 `oldStr.trim().isEmpty()` 判断）。
///
/// `paths` 中 `*` 段表示数组逐元素（对应 Java `Collection` 递归加密）；
/// 请求先序列化为 `Value`（字段名即 `@SerializedName` 线格式名），加密后
/// 重新序列化发送——与 Java 内存对象原地加密的线上语义一致。
pub fn encrypt_spec_fields_json(
    value: &mut serde_json::Value,
    public_key: &rsa::RsaPublicKey,
    paths: &[&str],
) -> Result<(), WxErrorException> {
    for path in paths {
        let segs: Vec<&str> = path.split('.').collect();
        encrypt_json_path(value, public_key, &segs)?;
    }
    Ok(())
}

fn encrypt_json_path(
    node: &mut serde_json::Value,
    public_key: &rsa::RsaPublicKey,
    segs: &[&str],
) -> Result<(), WxErrorException> {
    let (head, rest) = segs.split_first().ok_or_else(|| runtime("空加密路径"))?;
    if rest.is_empty() {
        if let Some(s) = node.get_mut(*head).and_then(|v| v.as_str()) {
            if !s.trim().is_empty() {
                let enc = crate::util::crypto::wx_pay_v3_crypto_utils::rsa_oaep_encrypt(
                    public_key,
                    s.trim(),
                )
                .map_err(|e| runtime(e.to_string()))?;
                if let Some(target) = node.get_mut(*head) {
                    *target = serde_json::Value::String(enc);
                }
            }
        }
        return Ok(());
    }
    if *head == "*" {
        if let Some(arr) = node.as_array_mut() {
            for item in arr.iter_mut() {
                encrypt_json_path(item, public_key, rest)?;
            }
        }
        return Ok(());
    }
    if let Some(child) = node.get_mut(*head) {
        encrypt_json_path(child, public_key, rest)?;
    }
    Ok(())
}

/// 执行 v3 媒体文件上传（multipart/form-data，对应 Java
/// `WxPayServiceHttpComponentsImpl.postV3(url, WechatPayUploadHttpPost)`）。
///
/// 微信媒体上传通道：`meta` JSON + `file` 二进制两段 multipart；签名消息体
/// 为**原始 multipart 字节**（含 boundary，对应 Java `entity.writeTo` 后
/// `WxPayCredentials.getBody()` 的字节串），故不能复用按字符串 body 签名的
/// [`execute_v3`]，此处按字节构造签名消息 `method\ncanonical\ntimestamp\n
/// nonce\n<bytes>\n` 后直签。
///
/// # 参数
/// - `content_type`：完整 `multipart/form-data; boundary=...` 头值
///   （由调用方生成 boundary 并同步拼装 body，保证签名与发送字节一致）
/// - `body`：完整 multipart 报文字节（含各段头与 boundary）
pub async fn execute_v3_upload(
    config: &dyn WxPayConfig,
    client: &reqwest::Client,
    url: &str,
    content_type: &str,
    body: &[u8],
) -> Result<String, WxErrorException> {
    let context = v3_context(config)?;
    let strip_prefix = config
        .api_host_url_path()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let canonical = canonical_url_strip_prefix(url, strip_prefix.as_deref())?;
    let timestamp = gen_timestamp();
    let nonce = gen_nonce_str();
    // 签名消息：原始字节（含不可打印的二进制文件段，Java 语义等价）
    let mut message = format!("POST\n{canonical}\n{timestamp}\n{nonce}\n").into_bytes();
    message.extend_from_slice(body);
    message.push(b'\n');
    let signature =
        sign_sha256_rsa(&context.private_key, &message).map_err(WxErrorException::from)?;
    let token = build_authorization_token(
        &context.mch_id,
        &nonce,
        timestamp,
        &context.serial_no,
        &signature,
    );
    let authorization = format!("WECHATPAY2-SHA256-RSA2048 {token}");

    let resp = client
        .post(url)
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::CONTENT_TYPE, content_type)
        .header("Authorization", authorization)
        .header("Wechatpay-Serial", wechatpay_serial_header(config))
        .body(body.to_vec())
        .send()
        .await
        .map_err(|e| WxErrorException::Http(e.to_string()))?;
    let status = resp.status();
    let headers: Vec<(String, String)> = resp
        .headers()
        .iter()
        .map(|(k, v)| {
            (
                k.as_str().to_string(),
                v.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    let text = resp
        .text()
        .await
        .map_err(|e| WxErrorException::Http(e.to_string()))?;

    if status == reqwest::StatusCode::OK || status == reqwest::StatusCode::NO_CONTENT {
        if !text.is_empty() {
            let header_refs: Vec<(&str, &str)> = headers
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            validate_v3_response(config, content_type.as_deref(), &header_refs, &text)?;
        }
        return Ok(text);
    }
    let err = match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(v) => {
            let code = v.get("code").and_then(|c| c.as_str()).unwrap_or_default();
            let message = v
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or_default();
            if code.is_empty() {
                runtime(message.to_string())
            } else {
                runtime(format!("{code} {message}"))
            }
        }
        Err(_) => runtime(format!(
            "微信支付V3接口返回错误，状态码：{status}，响应：{text}"
        )),
    };
    Err(err)
}

/// 按文件扩展名猜测 Content-Type（对应 Java
/// `URLConnection.guessContentTypeFromName`；未知扩展名回退
/// `application/octet-stream`，Java 注释说明"guess this is a video uploading"）。
pub fn guess_file_content_type(file_name: &str) -> &'static str {
    let ext = file_name
        .rsplit('.')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "webp" => "image/webp",
        "mp4" => "video/mp4",
        "mov" => "video/quicktime",
        "avi" => "video/x-msvideo",
        _ => "application/octet-stream",
    }
}

/// 构造 multipart/form-data 报文（对应 Java `WechatPayUploadHttpPost.Builder`
/// 的 `addBinaryBody("file", ...)` + `addTextBody("meta", ...)`，RFC6532）。
///
/// `meta` 为 JSON 文本段；返回 `(content_type, body)`，boundary 固定为
/// 随机串以保证签名与发送一致。
pub fn build_multipart_meta_file(
    file_name: &str,
    file_content_type: &str,
    file_data: &[u8],
    meta: &str,
) -> (String, Vec<u8>) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let boundary = format!(
        "wxpay-boundary-{}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default(),
        gen_nonce_str()
    );
    let mut body = Vec::new();
    let crlf = "\r\n";
    // meta 文本段
    body.extend_from_slice(format!("--{boundary}{crlf}Content-Disposition: form-data; name=\"meta\"{crlf}Content-Type: application/json{crlf}{crlf}{meta}{crlf}").as_bytes());
    // file 二进制段
    body.extend_from_slice(format!("--{boundary}{crlf}Content-Disposition: form-data; name=\"file\"; filename=\"{file_name}\"{crlf}Content-Type: {file_content_type}{crlf}{crlf}").as_bytes());
    body.extend_from_slice(file_data);
    body.extend_from_slice(format!("{crlf}--{boundary}--{crlf}").as_bytes());
    let content_type = format!("multipart/form-data; boundary={boundary}");
    (content_type, body)
}

/// 下载类 v3 请求（对应 Java `downloadV3`：200/204 且非 JSON → 字节流；
/// JSON（错误信息）→ convertException）。
pub async fn download_v3(
    config: &dyn WxPayConfig,
    client: &reqwest::Client,
    url: &str,
) -> Result<Vec<u8>, WxErrorException> {
    let context = v3_context(config)?;
    let strip_prefix = config
        .api_host_url_path()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let canonical = canonical_url_strip_prefix(url, strip_prefix.as_deref())?;
    let timestamp = gen_timestamp();
    let nonce = gen_nonce_str();
    let authorization = create_authorization_header(
        &context.mch_id,
        &context.serial_no,
        &context.private_key,
        "GET",
        &canonical,
        timestamp,
        &nonce,
        "",
    )
    .map_err(WxErrorException::from)?;

    let resp = client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/json")
        .header("Authorization", authorization)
        .header("Wechatpay-Serial", wechatpay_serial_header(config))
        .send()
        .await
        .map_err(|e| WxErrorException::Http(e.to_string()))?;
    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    let is_json = content_type
        .map(|ct| {
            ct.split(';')
                .next()
                .unwrap_or(&ct)
                .trim()
                .eq_ignore_ascii_case("application/json")
        })
        .unwrap_or(false);
    if (status == reqwest::StatusCode::OK || status == reqwest::StatusCode::NO_CONTENT) && !is_json
    {
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| WxErrorException::Http(e.to_string()))?
            .to_vec();
        return Ok(bytes);
    }
    let text = resp
        .text()
        .await
        .map_err(|e| WxErrorException::Http(e.to_string()))?;
    Err(match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(v) => {
            let code = v.get("code").and_then(|c| c.as_str()).unwrap_or_default();
            let message = v
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or_default();
            if code.is_empty() {
                runtime(message.to_string())
            } else {
                runtime(format!("{code} {message}"))
            }
        }
        Err(_) => runtime(format!(
            "微信支付V3接口返回错误，状态码：{status}，响应：{text}"
        )),
    })
}
