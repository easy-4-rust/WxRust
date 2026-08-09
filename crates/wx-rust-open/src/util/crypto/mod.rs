//! 开放平台（第三方平台）回调消息加解密。
//!
//! 对应 Java `me.chanjar.weixin.open.util.WxOpenCryptUtil`：继承
//! `me.chanjar.weixin.common.util.crypto.WxCryptUtil`，构造时以第三方平台
//! 的 componentToken / componentAesKey / componentAppId 初始化
//! （Java：`Base64.getDecoder().decode(StringUtils.remove(encodingAesKey, " "))`）。
//!
//! 算法与公众号/企业微信一致（AES-CBC + PKCS7 + Base64 + SHA1 签名，
//! 签名用排序后 `&` 连接即 `Sha1::digest_with_amp`），直接包装 common
//! `WxCryptUtil`，无独立密码学原语。

pub mod wx_open_crypt_utils;

pub use wx_open_crypt_utils::WxOpenCryptUtils;
