//! 企业微信消息加解密。
//!
//! 对应 Java `me.chanjar.weixin.cp.util.crypto` 包（`WxCpCryptUtil` +
//! `WxCpTpCryptUtil`）。`WxCpCryptUtils`（包装 common `WxCryptUtil`，
//! AES-CBC + PKCS7 + SHA1 签名）与会话存档 RSA 解密
//! （`decryptPriKey`/`decryptChatData` 系列）已实现；`WxCpTpCryptUtil`
//! （第三方代开发，包装 common `WxCryptUtil`，从 `WxCpTpConfigStorage`
//! 取 token/encodingAESKey/corpId）Wave 5 C5 补齐。

pub mod wx_cp_crypt_utils;
pub mod wx_cp_intelligent_robot_crypt_util;
pub mod wx_cp_tp_crypt_util;

pub use wx_cp_crypt_utils::WxCpCryptUtils;
pub use wx_cp_crypt_utils::decrypt_chat_data;
pub use wx_cp_crypt_utils::decrypt_encrypt_chat_msg;
pub use wx_cp_crypt_utils::decrypt_pri_key;
pub use wx_cp_crypt_utils::decrypt_pri_key_by_pkcs1;
pub use wx_cp_crypt_utils::decrypt_pri_key_by_pkcs8;
pub use wx_cp_intelligent_robot_crypt_util::WxCpIntelligentRobotCryptUtil;
pub use wx_cp_tp_crypt_util::WxCpTpCryptUtil;
