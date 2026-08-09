//! v3 通知解析辅助（通知头解析、验签、resource 解密与反序列化入口）。
//!
//! 对应 Java（weixin-java-pay）：
//! - `service/impl/BaseWxPayServiceImpl#verifyNotifySign`（通知验签串构造与
//!   WECHATPAY/SIGNTEST/ 探测流量识别）；
//! - `service/impl/BaseWxPayServiceImpl#baseParseOrderNotifyV3Result`
//!   （验签 → `OriginNotifyResponse` 解析 → `AesUtils.decryptToString`
//!   解密 → JSON 反序列化为解密结果）；
//! - `bean/notify/OriginNotifyResponse`（通知原始报文结构，本文件内定义；
//!   Wave 1 bean 生成后如 bean 侧落地可迁移）。
//!
//! 注意：类型化通知结果 bean（`WxPayNotifyV3Result` 等）在 `bean/notify.rs`
//! （Wave 1 填充），本文件只提供无状态的纯函数入口。

use serde::{Deserialize, Serialize};
use serde_json::Value;

use wx_rust_common::error::{WxErrorException, WxRuntimeError};

use crate::bean::notify::SignatureHeader;
use crate::util::crypto::wx_pay_v3_crypto_utils::{aes_gcm_decrypt, verify_sha256_rsa};

/// 通知验签失败文案（对应 Java `baseParseOrderNotifyV3Result`：
/// `throw new WxPayException("非法请求，头部信息验证失败")`）。
const INVALID_HEADER_MSG: &str = "非法请求，头部信息验证失败";

/// 通知解析失败文案（对应 Java `baseParseOrderNotifyV3Result` 的 catch 分支：
/// `throw new WxPayException("解析报文异常！", e)`）。
const PARSE_FAILED_MSG: &str = "解析报文异常！";

/// 微信支付签名探测流量前缀（对应 Java `verifyNotifySign` 中
/// `wxPaySign.startsWith("WECHATPAY/SIGNTEST/")`）。
const SIGN_TEST_PREFIX: &str = "WECHATPAY/SIGNTEST/";

/// 通知原始报文（对应 Java `bean/notify/OriginNotifyResponse`）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginNotifyResponse {
    /// 通知 ID（对应 Java `id`）
    pub id: Option<String>,
    /// 通知创建时间（对应 Java `create_time`）
    pub create_time: Option<String>,
    /// 通知类型（对应 Java `event_type`，如 TRANSACTION.SUCCESS）
    pub event_type: Option<String>,
    /// 通知简要说明（对应 Java `summary`）
    pub summary: Option<String>,
    /// 通知数据类型（对应 Java `resource_type`，支付成功通知为
    /// encrypt-resource）
    pub resource_type: Option<String>,
    /// 通知资源数据（对应 Java `resource`）
    pub resource: Option<NotifyResource>,
}

/// 通知资源数据（对应 Java `OriginNotifyResponse.Resource`）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyResource {
    /// 加密算法（对应 Java `algorithm`，目前只支持 AEAD_AES_256_GCM）
    pub algorithm: Option<String>,
    /// 原始回调类型（对应 Java `original_type`，如 transaction）
    pub original_type: Option<String>,
    /// 数据密文（对应 Java `ciphertext`，Base64 编码）
    pub ciphertext: Option<String>,
    /// 附加数据（对应 Java `associated_data`，AAD）
    pub associated_data: Option<String>,
    /// 加密使用的随机串（对应 Java `nonce`）
    pub nonce: Option<String>,
}

/// 通知解析结果（对应 Java `WxPayBaseNotifyV3Result<E>`：`rawData` + `result`）。
#[derive(Debug, Clone)]
pub struct NotifyV3Result<T> {
    /// 源数据（对应 Java `rawData`）
    pub raw_data: OriginNotifyResponse,
    /// 解密后的数据（对应 Java `result`）
    pub result: T,
}

/// 从 HTTP 响应头解析通知签名头（对应 Java 侧从请求头构造
/// `SignatureHeader` 的用法；`Wechatpay-Timestamp`/`Wechatpay-Nonce`/
/// `Wechatpay-Signature`/`Wechatpay-Serial`，名称大小写不敏感）。
///
/// # 参数
/// - `headers`：HTTP 头 (名称, 值) 列表
pub fn parse_signature_header(headers: &[(&str, &str)]) -> SignatureHeader {
    let mut header = SignatureHeader::default();
    for (name, value) in headers {
        match name.to_ascii_lowercase().as_str() {
            "wechatpay-timestamp" => header.time_stamp = Some((*value).to_string()),
            "wechatpay-nonce" => header.nonce = Some((*value).to_string()),
            "wechatpay-signature" => header.signature = Some((*value).to_string()),
            "wechatpay-serial" => header.serial = Some((*value).to_string()),
            _ => {}
        }
    }
    header
}

/// 构造通知验签串（对应 Java `verifyNotifySign` 的
/// `String.format("%s\n%s\n%s\n", timeStamp, nonce, data)`）。
pub fn build_notify_sign_message(header: &SignatureHeader, data: &str) -> String {
    let timestamp = header.time_stamp.as_deref().unwrap_or_default();
    let nonce = header.nonce.as_deref().unwrap_or_default();
    format!("{timestamp}\n{nonce}\n{data}\n")
}

/// 通知验签（对应 Java `verifyNotifySign`：先识别签名探测流量，再以平台证书
/// 公钥做 SHA256withRSA 验签）。
///
/// 说明：Java 侧按 `header.getSerial()` 从平台证书存储（`AutoUpdateCertificatesVerifier`）
/// 取对应公钥验签；证书存储为 Wave 2 项，本函数以调用方传入的单把公钥验签，
/// 多证书（按序列号路由）场景由调用方在包装层完成。
///
/// # 参数
/// - `public_key`：平台证书公钥
/// - `header`：通知签名头（`SignatureHeader`）
/// - `data`：通知原文（未解密 JSON）
///
/// # 返回
/// `Ok(true)` 验签通过；`Ok(false)` 验签不通过；头字段缺失/签名以
/// `WECHATPAY/SIGNTEST/` 开头（探测流量）返回 `Err`
pub fn verify_notify_signature(
    public_key: &rsa::RsaPublicKey,
    header: &SignatureHeader,
    data: &str,
) -> Result<bool, WxErrorException> {
    if let Some(signature) = header.signature.as_deref() {
        if signature.starts_with(SIGN_TEST_PREFIX) {
            // 对应 Java `WxSignTestException("微信支付签名探测流量")`
            return Err(WxErrorException::Runtime(WxRuntimeError::new(
                "微信支付签名探测流量",
            )));
        }
    }
    // 单公钥验签不校验 serial（Java 侧按 serial 路由证书存储的职责在调用方，
    // 见函数文档）
    let signature = header.signature.as_deref().unwrap_or_default();
    let message = build_notify_sign_message(header, data);
    verify_sha256_rsa(public_key, message.as_bytes(), signature).map_err(WxErrorException::from)
}

/// 解析通知原始报文（对应 Java `baseParseOrderNotifyV3Result` 中的
/// `GSON.fromJson(notifyData, OriginNotifyResponse.class)`）。
pub fn parse_origin_notify(notify_data: &str) -> Result<OriginNotifyResponse, WxErrorException> {
    serde_json::from_str(notify_data).map_err(|e| {
        WxErrorException::Runtime(WxRuntimeError::new(format!("{PARSE_FAILED_MSG} {e}")))
    })
}

/// 解密通知 resource 得到明文 JSON（对应 Java
/// `baseParseOrderNotifyV3Result` 中的
/// `AesUtils.decryptToString(associatedData, nonce, cipherText, apiV3Key)`）。
pub fn decrypt_resource(
    api_v3_key: &str,
    resource: &NotifyResource,
) -> Result<String, WxErrorException> {
    let ciphertext = resource.ciphertext.as_deref().ok_or_else(|| {
        WxErrorException::Runtime(WxRuntimeError::new(format!(
            "{PARSE_FAILED_MSG} 缺少 resource.ciphertext"
        )))
    })?;
    let associated_data = resource.associated_data.as_deref().unwrap_or_default();
    let nonce = resource.nonce.as_deref().ok_or_else(|| {
        WxErrorException::Runtime(WxRuntimeError::new(format!(
            "{PARSE_FAILED_MSG} 缺少 resource.nonce"
        )))
    })?;
    aes_gcm_decrypt(api_v3_key, associated_data, nonce, ciphertext).map_err(WxErrorException::from)
}

/// 完整 v3 通知解析入口（对应 Java `baseParseOrderNotifyV3Result`）：
/// 1. `header` 非空时先验签（失败 → "非法请求，头部信息验证失败"）；
/// 2. 解析 `OriginNotifyResponse`；
/// 3. 用 apiV3Key AES-256-GCM 解密 `resource.ciphertext`；
/// 4. 解密结果反序列化为 `T`。
///
/// # 参数
/// - `notify_data`：通知原文 JSON
/// - `header`：通知签名头（`None` 时跳过验签，对应 Java
///   `Objects.nonNull(header)` 语义）
/// - `api_v3_key`：APIv3 密钥
/// - `verify`：验签函数 `(serial, message, signature) -> bool`，由调用方按
///   序列号选择平台证书公钥（对应 Java `getConfig().getVerifier().verify`；
///   Wave 2 实现证书自动下载存储后可替换为 `AutoUpdateCertificatesVerifier`
///   等价物）
/// - `T`：解密后 JSON 对应的目标类型（对应 Java `DecryptNotifyResult` 类）
pub fn parse_notify_v3_result<T: serde::de::DeserializeOwned>(
    notify_data: &str,
    header: Option<&SignatureHeader>,
    api_v3_key: &str,
    verify: impl Fn(&str, &[u8], &str) -> bool,
) -> Result<NotifyV3Result<T>, WxErrorException> {
    // 对应 Java：if (Objects.nonNull(header) && !this.verifyNotifySign(header, notifyData))
    if let Some(header) = header {
        let message = build_notify_sign_message(header, notify_data);
        let signature = header.signature.as_deref().unwrap_or_default();
        let serial = header.serial.as_deref().unwrap_or_default();
        if !verify(serial, message.as_bytes(), signature) {
            return Err(WxErrorException::Runtime(WxRuntimeError::new(
                INVALID_HEADER_MSG,
            )));
        }
    }

    let raw_data = parse_origin_notify(notify_data)?;
    let resource = raw_data.resource.as_ref().ok_or_else(|| {
        WxErrorException::Runtime(WxRuntimeError::new(format!(
            "{PARSE_FAILED_MSG} 缺少 resource"
        )))
    })?;
    let decrypted = decrypt_resource(api_v3_key, resource)?;
    let result: T = serde_json::from_str(&decrypted).map_err(|e| {
        WxErrorException::Runtime(WxRuntimeError::new(format!("{PARSE_FAILED_MSG} {e}")))
    })?;

    Ok(NotifyV3Result { raw_data, result })
}

/// 解密通知 resource 得到 JSON Value（不关心目标类型的场景，
/// 对应 Java `parseOrderNotifyV3Result` 解密后直接反序列化前的中间结果）。
pub fn decrypt_resource_to_json(
    api_v3_key: &str,
    resource: &NotifyResource,
) -> Result<Value, WxErrorException> {
    let decrypted = decrypt_resource(api_v3_key, resource)?;
    serde_json::from_str(&decrypted).map_err(|e| {
        WxErrorException::Runtime(WxRuntimeError::new(format!("{PARSE_FAILED_MSG} {e}")))
    })
}
