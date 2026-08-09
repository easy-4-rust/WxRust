//! 微信支付默认内存配置存储。
//!
//! 对应 Java `com.github.binarywang.wxpay.config.WxPayConfig` 的字段全集
//! （Java 侧为可变 bean，无 token/锁语义；Rust 侧为值类型 + builder 风格
//! setter，构建完成后以 `Arc<dyn WxPayConfig>` 交给服务使用）。

use crate::config::{DEFAULT_PAY_BASE_URL, WxPayConfig};

/// 微信支付默认配置存储（值类型，字段与 Java `WxPayConfig` 全量镜像）。
#[derive(Debug, Clone, Default)]
pub struct WxPayDefaultConfig {
    /// 微信支付接口请求地址域名部分（对应 Java `apiHostUrl`）
    api_host_url: String,
    /// 微信支付接口请求地址路径前缀（对应 Java `apiHostUrlPath`）
    api_host_url_path: Option<String>,
    /// http 请求连接超时时间（对应 Java `httpConnectionTimeout`）
    http_connection_timeout: i32,
    /// http 请求数据读取等待时间（对应 Java `httpTimeout`）
    http_timeout: i32,
    /// 公众号 appid（对应 Java `appId`）
    app_id: Option<String>,
    /// 服务商模式下的子商户公众账号 ID（对应 Java `subAppId`）
    sub_app_id: Option<String>,
    /// 商户号（对应 Java `mchId`）
    mch_id: Option<String>,
    /// 商户密钥（对应 Java `mchKey`）
    mch_key: Option<String>,
    /// 企业支付密钥（对应 Java `entPayKey`）
    ent_pay_key: Option<String>,
    /// 服务商模式下的子商户号（对应 Java `subMchId`）
    sub_mch_id: Option<String>,
    /// 微信支付异步回调地址（对应 Java `notifyUrl`）
    notify_url: Option<String>,
    /// 退款结果异步回调地址（对应 Java `refundNotifyUrl`）
    refund_notify_url: Option<String>,
    /// 交易类型（对应 Java `tradeType`）
    trade_type: Option<String>,
    /// 签名方式（对应 Java `signType`）
    sign_type: Option<String>,
    /// p12 证书 base64 编码（对应 Java `keyString`）
    key_string: Option<String>,
    /// p12 证书文件的绝对路径或 `classpath:` 开头类路径（对应 Java `keyPath`）
    key_path: Option<String>,
    /// p12 证书文件内容的字节数组（对应 Java `keyContent`）
    key_content: Option<Vec<u8>>,
    /// apiclient_key.pem 证书 base64 编码（对应 Java `privateKeyString`）
    private_key_string: Option<String>,
    /// apiclient_key.pem 证书文件路径（对应 Java `privateKeyPath`）
    private_key_path: Option<String>,
    /// apiclient_key.pem 证书文件内容的字节数组（对应 Java `privateKeyContent`）
    private_key_content: Option<Vec<u8>>,
    /// apiclient_cert.pem 证书 base64 编码（对应 Java `privateCertString`）
    private_cert_string: Option<String>,
    /// apiclient_cert.pem 证书文件路径（对应 Java `privateCertPath`）
    private_cert_path: Option<String>,
    /// apiclient_cert.pem 证书文件内容的字节数组（对应 Java `privateCertContent`）
    private_cert_content: Option<Vec<u8>>,
    /// 公钥 ID（对应 Java `publicKeyId`）
    public_key_id: Option<String>,
    /// pub_key.pem 证书 base64 编码（对应 Java `publicKeyString`）
    public_key_string: Option<String>,
    /// pub_key.pem 证书文件路径（对应 Java `publicKeyPath`）
    public_key_path: Option<String>,
    /// pub_key.pem 证书文件内容的字节数组（对应 Java `publicKeyContent`）
    public_key_content: Option<Vec<u8>>,
    /// apiV3 秘钥值（对应 Java `apiV3Key`）
    api_v3_key: Option<String>,
    /// apiV3 证书序列号值（对应 Java `certSerialNo`）
    cert_serial_no: Option<String>,
    /// 私钥 PEM 字符串（`ADAPTED`：Java `privateKey` 为
    /// `java.security.PrivateKey` 对象）
    private_key: Option<String>,
    /// 微信支付分 serviceId（对应 Java `serviceId`）
    service_id: Option<String>,
    /// 微信支付分回调地址（对应 Java `payScoreNotifyUrl`）
    pay_score_notify_url: Option<String>,
    /// 微信支付分授权回调地址（对应 Java `payScorePermissionNotifyUrl`）
    pay_score_permission_notify_url: Option<String>,
    /// HTTP 连接池最大连接数（对应 Java `maxConnTotal`）
    max_conn_total: i32,
    /// HTTP 连接池每个路由的最大连接数（对应 Java `maxConnPerRoute`）
    max_conn_per_route: i32,
    /// 证书自动更新时间差（分钟，对应 Java `certAutoUpdateTime`）
    cert_auto_update_time: i32,
    /// 微信支付是否使用仿真测试环境（对应 Java `useSandboxEnv`）
    use_sandbox_env: bool,
    /// 是否将接口请求日志信息保存（对应 Java `ifSaveApiData`）
    if_save_api_data: bool,
    /// HTTP 代理主机（对应 Java `httpProxyHost`）
    http_proxy_host: Option<String>,
    /// HTTP 代理端口（对应 Java `httpProxyPort`）
    http_proxy_port: Option<i32>,
    /// HTTP 代理用户名（对应 Java `httpProxyUsername`）
    http_proxy_username: Option<String>,
    /// HTTP 代理密码（对应 Java `httpProxyPassword`）
    http_proxy_password: Option<String>,
    /// 是否将全部 v3 接口的请求都添加 `Wechatpay-Serial` 请求头
    /// （对应 Java `strictlyNeedWechatPaySerial`）
    strictly_need_wechat_pay_serial: bool,
    /// 是否完全使用公钥模式（对应 Java `fullPublicKeyModel`）
    full_public_key_model: bool,
}

impl WxPayDefaultConfig {
    /// 构建默认配置（域名默认 `https://api.mch.weixin.qq.com`，
    /// 超时 5000/10000ms，连接池 20/10，公钥模式默认开启）。
    pub fn new() -> Self {
        Self {
            api_host_url: DEFAULT_PAY_BASE_URL.to_string(),
            http_connection_timeout: 5000,
            http_timeout: 10000,
            max_conn_total: 20,
            max_conn_per_route: 10,
            cert_auto_update_time: 60,
            strictly_need_wechat_pay_serial: true,
            full_public_key_model: true,
            ..Self::default()
        }
    }

    // ---- builder 风格 setter（对应 Java Lombok 生成的全量 setter） ----

    /// 设置微信支付接口请求地址域名。
    pub fn set_api_host_url(&mut self, api_host_url: impl Into<String>) -> &mut Self {
        self.api_host_url = api_host_url.into();
        self
    }

    /// 设置微信支付接口请求地址路径前缀。
    pub fn set_api_host_url_path(&mut self, api_host_url_path: impl Into<String>) -> &mut Self {
        self.api_host_url_path = Some(api_host_url_path.into());
        self
    }

    /// 设置 http 请求连接超时时间（毫秒）。
    pub fn set_http_connection_timeout(&mut self, millis: i32) -> &mut Self {
        self.http_connection_timeout = millis;
        self
    }

    /// 设置 http 请求数据读取等待时间（毫秒）。
    pub fn set_http_timeout(&mut self, millis: i32) -> &mut Self {
        self.http_timeout = millis;
        self
    }

    /// 设置公众号 appid。
    pub fn set_app_id(&mut self, app_id: impl Into<String>) -> &mut Self {
        self.app_id = Some(app_id.into());
        self
    }

    /// 设置服务商模式下的子商户公众账号 ID。
    pub fn set_sub_app_id(&mut self, sub_app_id: impl Into<String>) -> &mut Self {
        self.sub_app_id = Some(sub_app_id.into());
        self
    }

    /// 设置商户号。
    pub fn set_mch_id(&mut self, mch_id: impl Into<String>) -> &mut Self {
        self.mch_id = Some(mch_id.into());
        self
    }

    /// 设置商户密钥。
    pub fn set_mch_key(&mut self, mch_key: impl Into<String>) -> &mut Self {
        self.mch_key = Some(mch_key.into());
        self
    }

    /// 设置企业支付密钥。
    pub fn set_ent_pay_key(&mut self, ent_pay_key: impl Into<String>) -> &mut Self {
        self.ent_pay_key = Some(ent_pay_key.into());
        self
    }

    /// 设置服务商模式下的子商户号。
    pub fn set_sub_mch_id(&mut self, sub_mch_id: impl Into<String>) -> &mut Self {
        self.sub_mch_id = Some(sub_mch_id.into());
        self
    }

    /// 设置微信支付异步回调地址。
    pub fn set_notify_url(&mut self, notify_url: impl Into<String>) -> &mut Self {
        self.notify_url = Some(notify_url.into());
        self
    }

    /// 设置退款结果异步回调地址。
    pub fn set_refund_notify_url(&mut self, refund_notify_url: impl Into<String>) -> &mut Self {
        self.refund_notify_url = Some(refund_notify_url.into());
        self
    }

    /// 设置交易类型。
    pub fn set_trade_type(&mut self, trade_type: impl Into<String>) -> &mut Self {
        self.trade_type = Some(trade_type.into());
        self
    }

    /// 设置签名方式（`HMAC-SHA256` 或 `MD5`）。
    pub fn set_sign_type(&mut self, sign_type: impl Into<String>) -> &mut Self {
        self.sign_type = Some(sign_type.into());
        self
    }

    /// 设置 p12 证书 base64 编码。
    pub fn set_key_string(&mut self, key_string: impl Into<String>) -> &mut Self {
        self.key_string = Some(key_string.into());
        self
    }

    /// 设置 p12 证书文件的绝对路径或 `classpath:` 开头类路径。
    pub fn set_key_path(&mut self, key_path: impl Into<String>) -> &mut Self {
        self.key_path = Some(key_path.into());
        self
    }

    /// 设置 p12 证书文件内容的字节数组。
    pub fn set_key_content(&mut self, key_content: Vec<u8>) -> &mut Self {
        self.key_content = Some(key_content);
        self
    }

    /// 设置 apiclient_key.pem 证书 base64 编码。
    pub fn set_private_key_string(&mut self, private_key_string: impl Into<String>) -> &mut Self {
        self.private_key_string = Some(private_key_string.into());
        self
    }

    /// 设置 apiclient_key.pem 证书文件路径。
    pub fn set_private_key_path(&mut self, private_key_path: impl Into<String>) -> &mut Self {
        self.private_key_path = Some(private_key_path.into());
        self
    }

    /// 设置 apiclient_key.pem 证书文件内容的字节数组。
    pub fn set_private_key_content(&mut self, private_key_content: Vec<u8>) -> &mut Self {
        self.private_key_content = Some(private_key_content);
        self
    }

    /// 设置 apiclient_cert.pem 证书 base64 编码。
    pub fn set_private_cert_string(&mut self, private_cert_string: impl Into<String>) -> &mut Self {
        self.private_cert_string = Some(private_cert_string.into());
        self
    }

    /// 设置 apiclient_cert.pem 证书文件路径。
    pub fn set_private_cert_path(&mut self, private_cert_path: impl Into<String>) -> &mut Self {
        self.private_cert_path = Some(private_cert_path.into());
        self
    }

    /// 设置 apiclient_cert.pem 证书文件内容的字节数组。
    pub fn set_private_cert_content(&mut self, private_cert_content: Vec<u8>) -> &mut Self {
        self.private_cert_content = Some(private_cert_content);
        self
    }

    /// 设置公钥 ID。
    pub fn set_public_key_id(&mut self, public_key_id: impl Into<String>) -> &mut Self {
        self.public_key_id = Some(public_key_id.into());
        self
    }

    /// 设置 pub_key.pem 证书 base64 编码。
    pub fn set_public_key_string(&mut self, public_key_string: impl Into<String>) -> &mut Self {
        self.public_key_string = Some(public_key_string.into());
        self
    }

    /// 设置 pub_key.pem 证书文件路径。
    pub fn set_public_key_path(&mut self, public_key_path: impl Into<String>) -> &mut Self {
        self.public_key_path = Some(public_key_path.into());
        self
    }

    /// 设置 pub_key.pem 证书文件内容的字节数组。
    pub fn set_public_key_content(&mut self, public_key_content: Vec<u8>) -> &mut Self {
        self.public_key_content = Some(public_key_content);
        self
    }

    /// 设置 apiV3 秘钥值。
    pub fn set_api_v3_key(&mut self, api_v3_key: impl Into<String>) -> &mut Self {
        self.api_v3_key = Some(api_v3_key.into());
        self
    }

    /// 设置 apiV3 证书序列号值。
    pub fn set_cert_serial_no(&mut self, cert_serial_no: impl Into<String>) -> &mut Self {
        self.cert_serial_no = Some(cert_serial_no.into());
        self
    }

    /// 设置私钥 PEM 字符串（`ADAPTED`：Java 为 `PrivateKey` 对象）。
    pub fn set_private_key(&mut self, private_key: impl Into<String>) -> &mut Self {
        self.private_key = Some(private_key.into());
        self
    }

    /// 设置微信支付分 serviceId。
    pub fn set_service_id(&mut self, service_id: impl Into<String>) -> &mut Self {
        self.service_id = Some(service_id.into());
        self
    }

    /// 设置微信支付分回调地址。
    pub fn set_pay_score_notify_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.pay_score_notify_url = Some(url.into());
        self
    }

    /// 设置微信支付分授权回调地址。
    pub fn set_pay_score_permission_notify_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.pay_score_permission_notify_url = Some(url.into());
        self
    }

    /// 设置 HTTP 连接池最大连接数。
    pub fn set_max_conn_total(&mut self, max_conn_total: i32) -> &mut Self {
        self.max_conn_total = max_conn_total;
        self
    }

    /// 设置 HTTP 连接池每个路由的最大连接数。
    pub fn set_max_conn_per_route(&mut self, max_conn_per_route: i32) -> &mut Self {
        self.max_conn_per_route = max_conn_per_route;
        self
    }

    /// 设置证书自动更新时间差（分钟）。
    pub fn set_cert_auto_update_time(&mut self, cert_auto_update_time: i32) -> &mut Self {
        self.cert_auto_update_time = cert_auto_update_time;
        self
    }

    /// 设置是否使用仿真测试环境。
    pub fn set_use_sandbox_env(&mut self, use_sandbox_env: bool) -> &mut Self {
        self.use_sandbox_env = use_sandbox_env;
        self
    }

    /// 设置是否保存接口请求日志信息。
    pub fn set_if_save_api_data(&mut self, if_save_api_data: bool) -> &mut Self {
        self.if_save_api_data = if_save_api_data;
        self
    }

    /// 设置 HTTP 代理主机。
    pub fn set_http_proxy_host(&mut self, host: impl Into<String>) -> &mut Self {
        self.http_proxy_host = Some(host.into());
        self
    }

    /// 设置 HTTP 代理端口。
    pub fn set_http_proxy_port(&mut self, port: i32) -> &mut Self {
        self.http_proxy_port = Some(port);
        self
    }

    /// 设置 HTTP 代理用户名。
    pub fn set_http_proxy_username(&mut self, username: impl Into<String>) -> &mut Self {
        self.http_proxy_username = Some(username.into());
        self
    }

    /// 设置 HTTP 代理密码。
    pub fn set_http_proxy_password(&mut self, password: impl Into<String>) -> &mut Self {
        self.http_proxy_password = Some(password.into());
        self
    }

    /// 设置是否将全部 v3 接口的请求都添加 `Wechatpay-Serial` 请求头。
    pub fn set_strictly_need_wechat_pay_serial(&mut self, v: bool) -> &mut Self {
        self.strictly_need_wechat_pay_serial = v;
        self
    }

    /// 设置是否完全使用公钥模式。
    pub fn set_full_public_key_model(&mut self, v: bool) -> &mut Self {
        self.full_public_key_model = v;
        self
    }
}

impl WxPayConfig for WxPayDefaultConfig {
    fn app_id(&self) -> Option<&str> {
        self.app_id.as_deref()
    }

    fn sub_app_id(&self) -> Option<&str> {
        self.sub_app_id.as_deref()
    }

    fn mch_id(&self) -> Option<&str> {
        self.mch_id.as_deref()
    }

    fn mch_key(&self) -> Option<&str> {
        self.mch_key.as_deref()
    }

    fn ent_pay_key(&self) -> Option<&str> {
        self.ent_pay_key.as_deref()
    }

    fn sub_mch_id(&self) -> Option<&str> {
        self.sub_mch_id.as_deref()
    }

    fn api_host_url(&self) -> Option<&str> {
        Some(&self.api_host_url)
    }

    fn api_host_url_path(&self) -> Option<&str> {
        self.api_host_url_path.as_deref()
    }

    fn notify_url(&self) -> Option<&str> {
        self.notify_url.as_deref()
    }

    fn refund_notify_url(&self) -> Option<&str> {
        self.refund_notify_url.as_deref()
    }

    fn trade_type(&self) -> Option<&str> {
        self.trade_type.as_deref()
    }

    fn sign_type(&self) -> Option<&str> {
        self.sign_type.as_deref()
    }

    fn key_string(&self) -> Option<&str> {
        self.key_string.as_deref()
    }

    fn key_path(&self) -> Option<&str> {
        self.key_path.as_deref()
    }

    fn key_content(&self) -> Option<&[u8]> {
        self.key_content.as_deref()
    }

    fn private_key_string(&self) -> Option<&str> {
        self.private_key_string.as_deref()
    }

    fn private_key_path(&self) -> Option<&str> {
        self.private_key_path.as_deref()
    }

    fn private_key_content(&self) -> Option<&[u8]> {
        self.private_key_content.as_deref()
    }

    fn private_cert_string(&self) -> Option<&str> {
        self.private_cert_string.as_deref()
    }

    fn private_cert_path(&self) -> Option<&str> {
        self.private_cert_path.as_deref()
    }

    fn private_cert_content(&self) -> Option<&[u8]> {
        self.private_cert_content.as_deref()
    }

    fn public_key_id(&self) -> Option<&str> {
        self.public_key_id.as_deref()
    }

    fn public_key_string(&self) -> Option<&str> {
        self.public_key_string.as_deref()
    }

    fn public_key_path(&self) -> Option<&str> {
        self.public_key_path.as_deref()
    }

    fn public_key_content(&self) -> Option<&[u8]> {
        self.public_key_content.as_deref()
    }

    fn api_v3_key(&self) -> Option<&str> {
        self.api_v3_key.as_deref()
    }

    fn cert_serial_no(&self) -> Option<&str> {
        self.cert_serial_no.as_deref()
    }

    fn private_key(&self) -> Option<&str> {
        self.private_key.as_deref()
    }

    fn service_id(&self) -> Option<&str> {
        self.service_id.as_deref()
    }

    fn pay_score_notify_url(&self) -> Option<&str> {
        self.pay_score_notify_url.as_deref()
    }

    fn pay_score_permission_notify_url(&self) -> Option<&str> {
        self.pay_score_permission_notify_url.as_deref()
    }

    fn http_connection_timeout(&self) -> i32 {
        self.http_connection_timeout
    }

    fn http_timeout(&self) -> i32 {
        self.http_timeout
    }

    fn max_conn_total(&self) -> i32 {
        self.max_conn_total
    }

    fn max_conn_per_route(&self) -> i32 {
        self.max_conn_per_route
    }

    fn cert_auto_update_time(&self) -> i32 {
        self.cert_auto_update_time
    }

    fn use_sandbox_env(&self) -> bool {
        self.use_sandbox_env
    }

    fn if_save_api_data(&self) -> bool {
        self.if_save_api_data
    }

    fn strictly_need_wechat_pay_serial(&self) -> bool {
        self.strictly_need_wechat_pay_serial
    }

    fn full_public_key_model(&self) -> bool {
        self.full_public_key_model
    }

    fn http_proxy_host(&self) -> Option<&str> {
        self.http_proxy_host.as_deref()
    }

    fn http_proxy_port(&self) -> Option<i32> {
        self.http_proxy_port
    }

    fn http_proxy_username(&self) -> Option<&str> {
        self.http_proxy_username.as_deref()
    }

    fn http_proxy_password(&self) -> Option<&str> {
        self.http_proxy_password.as_deref()
    }
}
