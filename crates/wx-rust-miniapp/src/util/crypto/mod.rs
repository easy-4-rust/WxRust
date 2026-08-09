//! 小程序消息加解密。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.util.crypt.WxMaCryptUtils`：包装
//! common `WxCryptUtil`，从 `WxMaConfig` 取 token/aesKey/appid。

pub mod wx_ma_crypt_utils;

pub use wx_ma_crypt_utils::WxMaCryptUtils;
