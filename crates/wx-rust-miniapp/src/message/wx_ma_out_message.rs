//! 输出消息通用接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.message.WxMaOutMessage`：
//! 支持 XML 与 JSON 两种格式输出，及加密打包（`toEncryptedXml`/
//! `toEncryptedJson`）。

use crate::config::WxMaConfig;

/// 微信小程序输出给微信服务器的消息的通用接口。
pub trait WxMaOutMessage {
    /// 转换成 XML 格式。
    fn to_xml(&self) -> String;

    /// 转换成 JSON 格式。
    fn to_json(&self) -> String;

    /// 转换成加密的 XML 格式（对应 Java `toEncryptedXml(WxMaConfig)`）。
    fn to_encrypted_xml(&self, config: &dyn WxMaConfig) -> Result<String, String>;

    /// 转换成加密的 JSON 格式（对应 Java `toEncryptedJson(WxMaConfig)`）。
    fn to_encrypted_json(&self, config: &dyn WxMaConfig) -> Result<String, String>;
}
