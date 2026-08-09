//! 微信支付签名工具。
//!
//! 对应 Java `com.github.binarywang.wxpay.util.SignUtils`（v2 报文签名
//! `createSign`）。v3 RSA 签名（`WxPayV3HttpClientBuilder` 的
//! WECHATPAY2-SHA256-RSA2048 Authorization 头）留待 Wave 3 证书加载后实现。

use std::collections::HashMap;

use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::SignUtils as CommonSignUtils;

use crate::constant::wx_pay_constants::sign_type as sign_type_const;

/// 签名的时候不携带的参数（对应 Java `SignUtils.NO_SIGN_PARAMS`）。
pub const NO_SIGN_PARAMS: [&str; 5] = ["sign", "key", "xmlString", "xmlDoc", "couponList"];

/// 签名工具（对应 Java `SignUtils`，全部方法为静态）。
pub struct SignUtils;

impl SignUtils {
    /// 微信支付 v2 签名算法（对应 Java
    /// `SignUtils.createSign(Map<String,String> params, String signType,
    /// String signKey, String[] ignoredParams)`）。
    ///
    /// 语义（与 Java 逐行对齐）：
    /// 1. 参数按 key 字典序排序（Java `TreeMap`）；
    /// 2. 跳过空值、`ignoredParams` 与 `NO_SIGN_PARAMS` 中的参数；
    /// 3. 拼接 `k=v&k=v&...&key=signKey`；
    /// 4. `HMAC-SHA256` 时对拼接串做 HmacSHA256（十六进制**大写**，
    ///    对应 Java common `SignUtils.createHmacSha256Sign` 的
    ///    `Hex.encodeHexString(bytes).toUpperCase()`）；否则 MD5 十六进制
    ///    **大写**（对应 Java `DigestUtils.md5Hex(...).toUpperCase()`）。
    /// 5. `signType` 为空时默认 MD5（Java 语义）。
    ///
    /// # 参数
    /// - `params`：参与签名的参数 map（不含 sign 本身）
    /// - `sign_type`：`SignType.HMAC_SHA256` 或 `SignType.MD5`（可空）
    /// - `sign_key`：签名 key（商户密钥 mchKey）
    /// - `ignored_params`：签名时需要忽略的特殊参数
    pub fn create_sign(
        params: &HashMap<String, String>,
        sign_type: Option<&str>,
        sign_key: &str,
        ignored_params: &[&str],
    ) -> Result<String, WxErrorException> {
        let mut to_sign = String::new();
        let mut keys: Vec<&String> = params.keys().collect();
        keys.sort();
        for key in keys {
            let value = params.get(key).map(String::as_str).unwrap_or_default();
            let should_sign = !value.is_empty()
                && !ignored_params.contains(&key.as_str())
                && !NO_SIGN_PARAMS.contains(&key.as_str());
            if should_sign {
                to_sign.push_str(key);
                to_sign.push('=');
                to_sign.push_str(value);
                to_sign.push('&');
            }
        }
        to_sign.push_str("key=");
        to_sign.push_str(sign_key);

        if sign_type == Some(sign_type_const::HMAC_SHA256) {
            // 对应 Java `me.chanjar.weixin.common.util.SignUtils.createHmacSha256Sign`
            Ok(CommonSignUtils::create_hmac_sha256_sign(&to_sign, sign_key))
        } else {
            // 对应 Java `DigestUtils.md5Hex(toSign.toString()).toUpperCase()`
            let md5 = crate::util::crypto::wx_pay_crypto_utils::md5_hex(&to_sign);
            Ok(md5.to_uppercase())
        }
    }
}
