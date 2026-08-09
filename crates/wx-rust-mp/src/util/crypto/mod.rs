//! 公众号消息加解密。
//!
//! 对应 Java `me.chanjar.weixin.mp.util.crypto.WxMpCryptUtil`：包装
//! common `WxCryptUtil`，从 `WxMpConfigStorage` 取 token/aesKey/appid。

pub mod wx_mp_crypt_util;

pub use wx_mp_crypt_util::WxMpCryptUtil;
