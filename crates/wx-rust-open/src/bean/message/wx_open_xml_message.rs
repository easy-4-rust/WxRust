//! 第三方平台推送消息（xml 格式）。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.message.WxOpenXmlMessage`。Java 用
//! XStream 反射映射（`@XStreamAlias` 元素名 + CDATA）；Rust 以 quick-xml
//! serde 派生（`quick_xml::de::from_str`）表达同一线格式：根元素 `xml`、
//! 元素名精确映射、缺失元素为 `None`/`Default`。
//!
//! 解析入口（对应 Java 同名静态方法）：
//! - [`WxOpenXmlMessage::from_xml`]：明文 XML 解析；
//! - [`WxOpenXmlMessage::from_encrypted_xml`]：加密回调解密（复用 Wave 0
//!   的 [`crate::util::crypto::WxOpenCryptUtils`]，SHA1 验签 + AES-256-CBC）
//!   后解析，并回填原始明文到 `context`；
//! - [`WxOpenXmlMessage::from_encrypted_mp_xml`]：解密公众号消息回调
//!   （ADAPTED，见方法文档）。
//!
//! 出站（被动回复）侧：第三方平台回复公众号消息与 mp 模块同线格式，
//! 加密打包入口为 [`wx_mp_out_xml_message_to_encrypted_xml`]
//! （对应 Java `wxMpOutXmlMessageToEncryptedXml`，ADAPTED 以明文 XML 入参）。

use crate::config::WxOpenConfigStorage;
use crate::util::crypto::WxOpenCryptUtils;

/// 第三方平台推送消息。
///
/// 元素名与 Java `@XStreamAlias` 一一对应；`context` 为原始明文
/// （`from_encrypted_xml` 回填，非 XML 元素）。
#[derive(Debug, Clone, Default, PartialEq, serde::Deserialize)]
#[serde(rename = "xml")]
pub struct WxOpenXmlMessage {
    /// 第三方平台的APPID。
    #[serde(rename = "AppId", default)]
    pub app_id: Option<String>,
    /// 消息创建时间。
    #[serde(rename = "CreateTime", default)]
    pub create_time: Option<i64>,
    /// 消息类型（verify_ticket / authorized / unauthorized / updateauthorized /
    /// notify_third_fasteregister / notify_3rd_wxa_auth 等）。
    #[serde(rename = "InfoType", default)]
    pub info_type: Option<String>,
    /// 第三方平台 component_verify_ticket（票据，5 分钟推送一次）。
    #[serde(rename = "ComponentVerifyTicket", default)]
    pub component_verify_ticket: Option<String>,
    /// 授权方 appid（authorized/updateauthorized/unauthorized 事件）。
    #[serde(rename = "AuthorizerAppid", default)]
    pub authorizer_appid: Option<String>,
    /// 授权码（authorized 事件，换取授权信息）。
    #[serde(rename = "AuthorizationCode", default)]
    pub authorization_code: Option<String>,
    /// 授权码过期时间（秒）。
    #[serde(rename = "AuthorizationCodeExpiredTime", default)]
    pub authorization_code_expired_time: Option<i64>,
    /// 预授权码。
    #[serde(rename = "PreAuthCode", default)]
    pub pre_auth_code: Option<String>,
    /// 子平台APPID（公众号/小程序的APPID）——快速创建小程序、小程序认证中。
    #[serde(rename = "appid", default)]
    pub sub_app_id: Option<String>,
    /// 快速创建小程序接口推送的状态（对应 Java `status`）。
    #[serde(rename = "status", default)]
    pub status: Option<i32>,
    /// 快速创建小程序的 auth_code。
    #[serde(rename = "auth_code", default)]
    pub auth_code: Option<String>,
    /// 快速创建小程序推送的消息。
    #[serde(rename = "msg", default)]
    pub msg: Option<String>,
    /// 快速创建小程序推送的信息。
    #[serde(rename = "info", default)]
    pub info: Info,
    /// 小程序认证（年审）任务ID。
    #[serde(rename = "taskid", default)]
    pub task_id: Option<String>,
    /// 认证任务状态：0初始 1超24小时 2用户拒绝 3用户同意 4发起人脸
    /// 5人脸失败 6人脸ok 7人脸认证后手机验证码 8手机验证失败 9手机验证成功
    /// 11创建审核单失败 12创建审核单成功 14验证失败 15等待支付。
    #[serde(rename = "task_status", default)]
    pub task_status: Option<i32>,
    /// 审核单状态，创建审核单成功后有效：0审核单不存在 1待支付 2审核中
    /// 3打回重填 4认证通过 5认证最终失败（不能再修改）。
    #[serde(rename = "apply_status", default)]
    pub apply_status: Option<i32>,
    /// 审核消息或失败原因。
    #[serde(rename = "message", default)]
    pub message: Option<String>,
    /// 审核提供商分配信息。
    #[serde(rename = "dispatch_info", default)]
    pub dispatch_info: Option<DispatchInfo>,
    /// 小程序认证（年审）即将到期通知的过期时间戳（秒数）。
    #[serde(rename = "expired", default)]
    pub expired: Option<i64>,
    /// 人脸核验任务id（infoType=notify_icpfiling_verify_result）。
    #[serde(rename = "task_id", default)]
    pub icp_verify_task_id: Option<String>,
    /// 小程序唯一id。
    #[serde(rename = "verify_appid", default)]
    pub verify_app_id: Option<String>,
    /// 人脸核验结果：2-核验失败；3-核验成功。
    #[serde(rename = "result", default)]
    pub result: Option<i32>,
    /// 发起时 along_with_auth 填 true 时有效：9. 认证短信核验通过。
    #[serde(rename = "along_with_auth_result", default)]
    pub along_with_auth_result: Option<i32>,
    /// 小程序唯一id（备案审核被驳回/通过事件 notify_apply_icpfiling_result）。
    #[serde(rename = "authorizer_appid", default)]
    pub beian_authorizer_app_id: Option<String>,
    /// 备案状态，参考“获取小程序备案状态及驳回原因”接口的备案状态枚举。
    #[serde(rename = "beian_status", default)]
    pub beian_status: Option<i32>,
    /// 小程序认证及备案任务流程id（notify_3rd_wxa_auth_and_icp）。
    #[serde(rename = "procedure_id", default)]
    pub procedure_id: Option<String>,
    /// 任务流程状态：9手机验证成功 15等待支付 16支付成功 17认证审核中
    /// 18认证审核驳回 19认证审核通过 20认证最终失败 21创建备案审核单失败
    /// 22备案平台审核中 23备案平台审核驳回 24备案管局审核中 25管局审核驳回
    /// 26认证及备案完成 27流程已过期 28流程已终止 29备案已撤回。
    #[serde(rename = "procedure_status", default)]
    pub procedure_status: Option<i32>,
    /// 原始通知内容（由 [`from_encrypted_xml`](Self::from_encrypted_xml) 回填，
    /// 非 XML 元素）。
    #[serde(default, skip)]
    pub context: Option<String>,
}

/// 快速创建小程序推送的信息（对应 Java 内嵌类
/// `WxOpenXmlMessage.Info`，`@XStreamAlias("info")`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Deserialize)]
pub struct Info {
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    #[serde(rename = "code", default)]
    pub code: Option<String>,
    #[serde(rename = "code_type", default)]
    pub code_type: Option<i32>,
    #[serde(rename = "legal_persona_wechat", default)]
    pub legal_persona_wechat: Option<String>,
    #[serde(rename = "legal_persona_name", default)]
    pub legal_persona_name: Option<String>,
    #[serde(rename = "component_phone", default)]
    pub component_phone: Option<String>,
    /// 创建个人小程序审核通知数据。
    #[serde(rename = "wxuser", default)]
    pub wxuser: Option<String>,
    #[serde(rename = "idname", default)]
    pub idname: Option<String>,
    /// 创建试用小程序成功/失败的通知数据。
    #[serde(rename = "unique_id", default)]
    pub unique_id: Option<String>,
}

/// 审核提供商分配信息（对应 Java 内嵌类 `WxOpenXmlMessage.DispatchInfo`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Deserialize)]
pub struct DispatchInfo {
    /// 提供商，如：上海倍通企业信用征信有限公司。
    #[serde(rename = "provider", default)]
    pub provider: Option<String>,
    /// 联系方式，如：咨询电话：0411-84947888。
    #[serde(rename = "contact", default)]
    pub contact: Option<String>,
    /// 派遣时间戳(秒)，如：1704952913。
    #[serde(rename = "dispatch_time", default)]
    pub dispatch_time: Option<i64>,
}

impl WxOpenXmlMessage {
    /// 从明文 xml 解析消息。
    ///
    /// 对应 Java `fromXml(String)`：先修正微信变态的消息内容格式
    /// （`</PicList><PicList>` 相邻闭合/开启，open 消息中为无操作），
    /// 再以 quick-xml serde 按 `@XStreamAlias` 元素名映射字段。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let xml = xml.replace("</PicList><PicList>", "");
        quick_xml::de::from_str(&xml).map_err(|e| format!("WxOpenXmlMessage 解析失败: {e}"))
    }

    /// 从加密字符串转换（第三方平台回调消息解密入口）。
    ///
    /// 对应 Java `fromEncryptedXml(String, WxOpenConfigStorage, String, String,
    /// String)`：`WxOpenCryptUtils` 验签并 AES 解密（Wave 0 已实现），
    /// 解密为空时返回错误，解析后回填原始明文到 `context`。
    pub fn from_encrypted_xml(
        encrypted_xml: &str,
        config: &dyn WxOpenConfigStorage,
        timestamp: &str,
        nonce: &str,
        msg_signature: &str,
    ) -> Result<Self, String> {
        let crypt_util = WxOpenCryptUtils::new(config)?;
        let plain_text = crypt_util.decrypt_xml(msg_signature, timestamp, nonce, encrypted_xml)?;
        if plain_text.trim().is_empty() {
            return Err("解密后的xml消息内容为空，请检查加密参数是否正确".to_string());
        }
        let mut message = Self::from_xml(&plain_text)?;
        message.context = Some(plain_text);
        Ok(message)
    }

    /// 解密第三方平台推送的公众号消息回调。
    ///
    /// 对应 Java `fromEncryptedMpXml(...)`（返回 `WxMpXmlMessage`）。
    ///
    /// ADAPTED：wx-rust-open 尚未依赖 wx-rust-mp，此处返回解密后的明文
    /// XML 字符串，由调用方交给 `wx_rust_mp::bean::message::WxMpXmlMessage::
    /// from_xml` 解析；Wave 2 引入跨 crate 依赖后换为返回 `WxMpXmlMessage`。
    pub fn from_encrypted_mp_xml(
        encrypted_xml: &str,
        config: &dyn WxOpenConfigStorage,
        timestamp: &str,
        nonce: &str,
        msg_signature: &str,
    ) -> Result<String, String> {
        let crypt_util = WxOpenCryptUtils::new(config)?;
        crypt_util.decrypt_xml(msg_signature, timestamp, nonce, encrypted_xml)
    }
}

/// 将待回复的公众号消息加密打包为回调 xml。
///
/// 对应 Java `wxMpOutXmlMessageToEncryptedXml(WxMpXmlOutMessage,
/// WxOpenConfigStorage)`：Java 先 `message.toXml()` 得明文 XML 再加密；
/// ADAPTED：Rust 侧以明文 XML 字符串入参（由调用方用
/// `wx_rust_mp` 的 `WxMpXmlOutMessage::to_xml` 生成），加密流程
/// （`WxOpenCryptUtils::encrypt`）与 Java 逐字一致。
pub fn wx_mp_out_xml_message_to_encrypted_xml(
    plain_xml: &str,
    config: &dyn WxOpenConfigStorage,
) -> Result<String, String> {
    let crypt_util = WxOpenCryptUtils::new(config)?;
    crypt_util.encrypt(plain_xml)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// verify_ticket 推送（Java 测试 golden 线格式）。
    #[test]
    fn parse_verify_ticket() {
        let xml = r#"<xml>
  <AppId><![CDATA[wxb1234567890abcdef]]></AppId>
  <CreateTime>1413192605</CreateTime>
  <InfoType><![CDATA[verify_ticket]]></InfoType>
  <ComponentVerifyTicket><![CDATA[ticket@@@6iJtQHC1]]></ComponentVerifyTicket>
</xml>"#;
        let m = WxOpenXmlMessage::from_xml(xml).unwrap();
        assert_eq!(m.app_id.as_deref(), Some("wxb1234567890abcdef"));
        assert_eq!(m.create_time, Some(1413192605));
        assert_eq!(m.info_type.as_deref(), Some("verify_ticket"));
        assert_eq!(
            m.component_verify_ticket.as_deref(),
            Some("ticket@@@6iJtQHC1")
        );
        assert_eq!(m.context, None);
    }

    /// 快速创建小程序（notify_third_fasteregister）推送：info 嵌套 + 小写键。
    #[test]
    fn parse_fasteregister() {
        let xml = r#"<xml>
<AppId><![CDATA[wx1234567890abcdef]]></AppId>
<CreateTime>1492747804</CreateTime>
<InfoType><![CDATA[notify_third_fasteregister]]></InfoType>
<appid><![CDATA[wx1234567890abcdef]]></appid>
<status>0</status>
<auth_code><![CDATA[queryauthcode@@@123]]></auth_code>
<msg><![CDATA[]]></msg>
<info><name><![CDATA[test]]></name><code><![CDATA[123456]]></code><code_type>1</code_type><legal_persona_wechat><![CDATA[wx123]]></legal_persona_wechat><legal_persona_name><![CDATA[张三]]></legal_persona_name><component_phone><![CDATA[13800000000]]></component_phone></info>
</xml>"#;
        let m = WxOpenXmlMessage::from_xml(xml).unwrap();
        assert_eq!(m.info_type.as_deref(), Some("notify_third_fasteregister"));
        assert_eq!(m.sub_app_id.as_deref(), Some("wx1234567890abcdef"));
        assert_eq!(m.status, Some(0));
        assert_eq!(m.auth_code.as_deref(), Some("queryauthcode@@@123"));
        assert_eq!(m.info.name.as_deref(), Some("test"));
        assert_eq!(m.info.code.as_deref(), Some("123456"));
        assert_eq!(m.info.code_type, Some(1));
        assert_eq!(m.info.legal_persona_name.as_deref(), Some("张三"));
    }
}
