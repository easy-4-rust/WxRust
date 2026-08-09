//! 企业微信 API 地址常量（门面/核心）。
//!
//! 对应 Java `WxCpApiPathConsts` 顶层常量（`DEFAULT_CP_BASE_URL` 及各
//! 门面执行引擎使用的路径）。完整地址 = `getApiUrl(path)`（baseUrl +
//! path，见 `WxCpConfigStorage::api_url`）。各业务子域常量按 Java 内部类
//! 拆分到 `url_message`/`url_agent`/`url_oa` 等子模块。

/// 企业微信 API 默认 baseUrl（对应 Java
/// `WxCpApiPathConsts.DEFAULT_CP_BASE_URL`）。
pub const DEFAULT_CP_BASE_URL: &str = "https://qyapi.weixin.qq.com";

/// 获取 jsapi_ticket（对应 Java `GET_JSAPI_TICKET`）。
pub const GET_JSAPI_TICKET: &str = "/cgi-bin/get_jsapi_ticket";
/// 获取应用的 jsapi_ticket（对应 Java `GET_AGENT_CONFIG_TICKET`）。
pub const GET_AGENT_CONFIG_TICKET: &str = "/cgi-bin/ticket/get?&type=agent_config";
/// 获取企业微信服务器 IP 段（对应 Java `GET_CALLBACK_IP`）。
pub const GET_CALLBACK_IP: &str = "/cgi-bin/getcallbackip";
/// 获取企业微信接口 IP 段（对应 Java `GET_API_DOMAIN_IP`）。
pub const GET_API_DOMAIN_IP: &str = "/cgi-bin/get_api_domain_ip";
/// 上传部门列表覆盖企业号上的部门信息（对应 Java `BATCH_REPLACE_PARTY`）。
pub const BATCH_REPLACE_PARTY: &str = "/cgi-bin/batch/replaceparty";
/// 上传用户列表，增量更新成员（对应 Java `BATCH_SYNC_USER`）。
pub const BATCH_SYNC_USER: &str = "/cgi-bin/batch/syncuser";
/// 上传用户列表覆盖企业号上的用户信息（对应 Java `BATCH_REPLACE_USER`）。
pub const BATCH_REPLACE_USER: &str = "/cgi-bin/batch/replaceuser";
/// 获取异步任务结果（对应 Java `BATCH_GET_RESULT`，`jobid` 拼在路径后）。
pub const BATCH_GET_RESULT: &str = "/cgi-bin/batch/getresult?jobid=";
/// 企业微信小程序登录凭证校验（对应 Java `JSCODE_TO_SESSION`）。
pub const JSCODE_TO_SESSION: &str = "/cgi-bin/miniprogram/jscode2session";
/// 获取 access_token（对应 Java `GET_TOKEN`，`%s` 依次为 corpid/corpsecret）。
pub const GET_TOKEN: &str = "/cgi-bin/gettoken?corpid=%s&corpsecret=%s";
/// 群机器人 webhook 发送（对应 Java `WEBHOOK_SEND`，`key` 拼在路径后）。
pub const WEBHOOK_SEND: &str = "/cgi-bin/webhook/send?key=";
