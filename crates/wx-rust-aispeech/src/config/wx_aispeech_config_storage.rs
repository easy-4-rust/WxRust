//! 智能对话配置存储接口。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.config.WxAispeechConfigStorage`。
//! 与公众号/小程序不同，aispeech 不使用 access_token，而是基于
//! appid/token/aesKey/secretKey 的请求头签名体系（见 `crate::util`），
//! 故该 trait 不继承 common `WxConfigStorage`。

/// 对话机器人 API 默认主机地址（对应 Java `WxAispeechDefaultConfigImpl`
/// 字段初始值 `dialogApiBaseUrl = "https://openaiapi.weixin.qq.com"`）。
pub const DEFAULT_DIALOG_API_BASE_URL: &str = "https://openaiapi.weixin.qq.com";
/// 知识库助理 API 默认主机地址（对应 Java 字段初始值
/// `knowledgeApiBaseUrl = "https://weknora.weixin.qq.com"`）。
pub const DEFAULT_KNOWLEDGE_API_BASE_URL: &str = "https://weknora.weixin.qq.com";

/// 智能对话配置存储。
///
/// Java 接口以 getter/setter 形式暴露；Rust trait 以只读方法表达同一契约。
/// `open_ai_token` 是唯一可写项（对应 Java `setOpenAiToken`，由
/// `getAccessToken` 成功后写入）。
pub trait WxAispeechConfigStorage: Send + Sync {
    /// appid（对应 Java `getAppid()`）。
    fn appid(&self) -> Option<&str>;

    /// 消息校验 token（对应 Java `getToken()`，dialog 请求签名用）。
    fn token(&self) -> Option<&str>;

    /// 对话查询 AES 密钥（对应 Java `getAesKey()`，base64 编码）。
    fn aes_key(&self) -> Option<&str>;

    /// OpenAI token（对应 Java `getOpenAiToken()`，`X-OPENAI-TOKEN` 头）。
    ///
    /// 返回 owned 值（Rust 适配：实现侧以锁保护该字段，无法返回借用）。
    fn open_ai_token(&self) -> Option<String>;

    /// 设置 OpenAI token（对应 Java `setOpenAiToken(String)`）。
    fn set_open_ai_token(&self, open_ai_token: &str);

    /// 知识库签名密钥（对应 Java `getSecretKey()`，HmacSHA256 签名用）。
    fn secret_key(&self) -> Option<&str>;

    /// 对话机器人 API 基地址（对应 Java `getDialogApiBaseUrl()`）。
    fn dialog_api_base_url(&self) -> String {
        DEFAULT_DIALOG_API_BASE_URL.to_string()
    }

    /// 知识库助理 API 基地址（对应 Java `getKnowledgeApiBaseUrl()`）。
    fn knowledge_api_base_url(&self) -> String {
        DEFAULT_KNOWLEDGE_API_BASE_URL.to_string()
    }

    /// HTTP 代理主机（对应 Java `getHttpProxyHost()`）。
    fn http_proxy_host(&self) -> Option<&str>;

    /// HTTP 代理端口（对应 Java `getHttpProxyPort()`）。
    fn http_proxy_port(&self) -> i32;

    /// HTTP 代理用户名（对应 Java `getHttpProxyUsername()`）。
    fn http_proxy_username(&self) -> Option<&str>;

    /// HTTP 代理密码（对应 Java `getHttpProxyPassword()`）。
    fn http_proxy_password(&self) -> Option<&str>;
}
