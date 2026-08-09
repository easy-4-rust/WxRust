//! 微信支付配置存储接口。
//!
//! 对应 Java `com.github.binarywang.wxpay.config.WxPayConfig`。Java 侧为
//! 一个可变的 Lombok `@Data` 类；Rust 侧以只读 trait（值语义 + 默认方法）
//! 表达同一契约：服务侧只读使用，配置构建阶段通过 `WxPayDefaultConfig`
//! 的 builder 风格 setter 完成（见 `impl/wx_pay_default_config_impl.rs`）。
//!
//! 与 mp/miniapp 的 `WxConfigStorage`（access_token 缓存）不同：微信支付
//! **没有 access_token**，v2 靠 MD5/HMAC-SHA256 报文签名，v3 靠 RSA 私钥
//! 签名 + 平台证书验签，故本 trait 不继承 token 存储。
//!
//! 说明：
//! - `apiV3HttpClient`/`httpClient`/`sslHttpClient`/`verifier`/两个
//!   `HttpClientBuilderCustomizer`/`sslContext` 为 Java 侧运行时对象，
//!   Rust 以 reqwest 统一 HTTP（`PLATFORM_NA`，见迁移路线图），不设字段。
//! - `privateKey`（Java `java.security.PrivateKey`）在 Rust 中以 PEM 字符串
//!   表达（`ADAPTED`），解析为 `rsa::RsaPrivateKey` 留待 Wave 3。
//! - Java 的动态 setter 在 trait 中暂不提供（Wave 1 多商户切换需要时按
//!   需扩展），构建期 setter 见 `WxPayDefaultConfig`。

/// 微信支付接口请求地址默认域名（对应 Java `WxPayConfig.DEFAULT_PAY_BASE_URL`）。
pub const DEFAULT_PAY_BASE_URL: &str = "https://api.mch.weixin.qq.com";

/// 微信支付配置存储。
pub trait WxPayConfig: Send + Sync {
    // ---- 基础标识（对应 Java getAppId/getSubAppId/getMchId/getMchKey/...） ----

    /// 公众号 appid（对应 Java `getAppId()`）。
    fn app_id(&self) -> Option<&str>;

    /// 服务商模式下的子商户公众账号 ID（对应 Java `getSubAppId()`）。
    fn sub_app_id(&self) -> Option<&str>;

    /// 商户号（对应 Java `getMchId()`）。
    fn mch_id(&self) -> Option<&str>;

    /// 商户密钥（对应 Java `getMchKey()`，v2 签名 key）。
    fn mch_key(&self) -> Option<&str>;

    /// 企业支付密钥（对应 Java `getEntPayKey()`）。
    fn ent_pay_key(&self) -> Option<&str>;

    /// 服务商模式下的子商户号（对应 Java `getSubMchId()`）。
    fn sub_mch_id(&self) -> Option<&str>;

    // ---- 请求地址（对应 Java getApiHostUrl/getApiHostUrlPath） ----

    /// 微信支付接口请求地址域名原始值（对应 Java 字段 `apiHostUrl`，
    /// 默认 `DEFAULT_PAY_BASE_URL`）。
    fn api_host_url(&self) -> Option<&str>;

    /// 微信支付接口请求地址路径前缀原始值（用于网关代理前缀，对应 Java
    /// 字段 `apiHostUrlPath`，如 `/api-weixin`）。
    fn api_host_url_path(&self) -> Option<&str>;

    /// 返回所设置的微信支付接口请求地址域名（对应 Java `getApiHostUrl()`）。
    ///
    /// 语义：trim 后为空回退默认地址；去掉结尾 `/`。
    fn effective_api_host_url(&self) -> String {
        let host = self.api_host_url().map(str::trim).filter(|s| !s.is_empty());
        let mut host_url = host.unwrap_or(DEFAULT_PAY_BASE_URL).to_string();
        if host_url.ends_with('/') {
            host_url.pop();
        }
        host_url
    }

    /// 返回所设置的微信支付接口路径前缀（对应 Java `getApiHostUrlPath()`）。
    ///
    /// 语义：空或 `/` 时为空字符串；补齐开头 `/`；去掉结尾 `/`。
    fn effective_api_host_url_path(&self) -> String {
        let path = self
            .api_host_url_path()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let mut path_prefix = path.unwrap_or_default().to_string();
        if path_prefix == "/" {
            return String::new();
        }
        if !path_prefix.starts_with('/') {
            path_prefix.insert(0, '/');
        }
        if path_prefix.ends_with('/') {
            path_prefix.pop();
        }
        path_prefix
    }

    /// 返回用于请求层拼接的基础地址：host + pathPrefix
    /// （对应 Java `getApiHostWithPathPrefix()`）。
    fn api_host_with_path_prefix(&self) -> String {
        format!(
            "{}{}",
            self.effective_api_host_url(),
            self.effective_api_host_url_path()
        )
    }

    // ---- 回调地址（对应 Java getNotifyUrl/getRefundNotifyUrl） ----

    /// 微信支付异步回调地址（对应 Java `getNotifyUrl()`）。
    fn notify_url(&self) -> Option<&str>;

    /// 退款结果异步回调地址（对应 Java `getRefundNotifyUrl()`）。
    fn refund_notify_url(&self) -> Option<&str>;

    // ---- 交易/签名（对应 Java getTradeType/getSignType） ----

    /// 交易类型（对应 Java `getTradeType()`，如 JSAPI/NATIVE/APP）。
    fn trade_type(&self) -> Option<&str>;

    /// 签名方式（对应 Java `getSignType()`，`HMAC_SHA256` 或 `MD5`，
    /// 见 [`crate::constant::wx_pay_constants::SignType`]）。
    fn sign_type(&self) -> Option<&str>;

    // ---- p12 证书（对应 Java getKeyString/getKeyPath/getKeyContent） ----

    /// p12 证书 base64 编码（对应 Java `getKeyString()`）。
    fn key_string(&self) -> Option<&str>;

    /// p12 证书文件的绝对路径或 `classpath:` 开头类路径（对应 Java `getKeyPath()`）。
    fn key_path(&self) -> Option<&str>;

    /// p12 证书文件内容的字节数组（对应 Java `getKeyContent()`）。
    fn key_content(&self) -> Option<&[u8]>;

    // ---- apiclient_key.pem（对应 Java getPrivateKeyString/getPrivateKeyPath/...） ----

    /// apiclient_key.pem 证书 base64 编码（对应 Java `getPrivateKeyString()`）。
    fn private_key_string(&self) -> Option<&str>;

    /// apiclient_key.pem 证书文件的绝对路径或 `classpath:` 开头类路径
    /// （对应 Java `getPrivateKeyPath()`）。
    fn private_key_path(&self) -> Option<&str>;

    /// apiclient_key.pem 证书文件内容的字节数组（对应 Java `getPrivateKeyContent()`）。
    fn private_key_content(&self) -> Option<&[u8]>;

    // ---- apiclient_cert.pem（对应 Java getPrivateCertString/getPrivateCertPath/...） ----

    /// apiclient_cert.pem 证书 base64 编码（对应 Java `getPrivateCertString()`）。
    fn private_cert_string(&self) -> Option<&str>;

    /// apiclient_cert.pem 证书文件的绝对路径或 `classpath:` 开头类路径
    /// （对应 Java `getPrivateCertPath()`）。
    fn private_cert_path(&self) -> Option<&str>;

    /// apiclient_cert.pem 证书文件内容的字节数组（对应 Java `getPrivateCertContent()`）。
    fn private_cert_content(&self) -> Option<&[u8]>;

    // ---- 公钥（微信公钥模式，对应 Java getPublicKeyId/getPublicKeyString/...） ----

    /// 公钥 ID（对应 Java `getPublicKeyId()`）。
    fn public_key_id(&self) -> Option<&str>;

    /// pub_key.pem 证书 base64 编码（对应 Java `getPublicKeyString()`）。
    fn public_key_string(&self) -> Option<&str>;

    /// pub_key.pem 证书文件的绝对路径或 `classpath:` 开头类路径
    /// （对应 Java `getPublicKeyPath()`）。
    fn public_key_path(&self) -> Option<&str>;

    /// pub_key.pem 证书文件内容的字节数组（对应 Java `getPublicKeyContent()`）。
    fn public_key_content(&self) -> Option<&[u8]>;

    // ---- v3 密钥（对应 Java getApiV3Key/getCertSerialNo/getPrivateKey） ----

    /// apiV3 秘钥值（对应 Java `getApiV3Key()`）。
    fn api_v3_key(&self) -> Option<&str>;

    /// apiV3 证书序列号值（对应 Java `getCertSerialNo()`）。
    fn cert_serial_no(&self) -> Option<&str>;

    /// 私钥 PEM 字符串（`ADAPTED`：Java `getPrivateKey()` 返回
    /// `java.security.PrivateKey` 对象，Rust 以 PEM 文本表达，Wave 3 解析）。
    fn private_key(&self) -> Option<&str>;

    // ---- 微信支付分（对应 Java getServiceId/getPayScoreNotifyUrl/...） ----

    /// 微信支付分 serviceId（对应 Java `getServiceId()`）。
    fn service_id(&self) -> Option<&str>;

    /// 微信支付分回调地址（对应 Java `getPayScoreNotifyUrl()`）。
    fn pay_score_notify_url(&self) -> Option<&str>;

    /// 微信支付分授权回调地址（对应 Java `getPayScorePermissionNotifyUrl()`）。
    fn pay_score_permission_notify_url(&self) -> Option<&str>;

    // ---- HTTP（对应 Java getHttpConnectionTimeout/getHttpTimeout/...） ----

    /// http 请求连接超时时间（毫秒，对应 Java `getHttpConnectionTimeout()`）。
    fn http_connection_timeout(&self) -> i32 {
        5000
    }

    /// http 请求数据读取等待时间（毫秒，对应 Java `getHttpTimeout()`）。
    fn http_timeout(&self) -> i32 {
        10000
    }

    /// HTTP 连接池最大连接数（对应 Java `getMaxConnTotal()`，默认 20）。
    fn max_conn_total(&self) -> i32 {
        20
    }

    /// HTTP 连接池每个路由的最大连接数（对应 Java `getMaxConnPerRoute()`，默认 10）。
    fn max_conn_per_route(&self) -> i32 {
        10
    }

    /// 证书自动更新时间差（分钟，对应 Java `getCertAutoUpdateTime()`，默认 60）。
    fn cert_auto_update_time(&self) -> i32 {
        60
    }

    // ---- 开关（对应 Java isUseSandboxEnv/isIfSaveApiData/...） ----

    /// 微信支付是否使用仿真测试环境（对应 Java `isUseSandboxEnv()`，默认 false）。
    fn use_sandbox_env(&self) -> bool {
        false
    }

    /// 是否将接口请求日志信息保存到 threadLocal 中（对应 Java
    /// `isIfSaveApiData()`，默认 false；Rust 以 impl 内 RwLock 保存）。
    fn if_save_api_data(&self) -> bool {
        false
    }

    /// 是否将全部 v3 接口的请求都添加 `Wechatpay-Serial` 请求头
    /// （对应 Java `isStrictlyNeedWechatPaySerial()`，默认 true）。
    fn strictly_need_wechat_pay_serial(&self) -> bool {
        true
    }

    /// 是否完全使用公钥模式（微信从平台证书到公钥的灰度切换，
    /// 对应 Java `isFullPublicKeyModel()`，默认 true）。
    fn full_public_key_model(&self) -> bool {
        true
    }

    // ---- 代理（对应 Java getHttpProxyHost/getHttpProxyPort/...） ----

    /// HTTP 代理主机（对应 Java `getHttpProxyHost()`）。
    fn http_proxy_host(&self) -> Option<&str>;

    /// HTTP 代理端口（对应 Java `getHttpProxyPort()`）。
    fn http_proxy_port(&self) -> Option<i32>;

    /// HTTP 代理用户名（对应 Java `getHttpProxyUsername()`）。
    fn http_proxy_username(&self) -> Option<&str>;

    /// HTTP 代理密码（对应 Java `getHttpProxyPassword()`）。
    fn http_proxy_password(&self) -> Option<&str>;
}
