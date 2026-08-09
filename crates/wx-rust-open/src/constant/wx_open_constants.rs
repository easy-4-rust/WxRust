//! 开放平台（第三方平台）常量。
//!
//! 对应 Java `me.chanjar.weixin.open` 包。
//!
//! 说明：Java `weixin-java-open` 模块**无独立 constant 包**（与 mp/ma 不同），
//! URL 常量直接定义在各 Service 接口中（`WxOpenComponentService` 等），
//! 已按任务要求拆分至 `crate::enums::url_core`；本文件仅收录引擎所需的
//! 非 URL 常量。

/// component_access_token 注入参数键（对应 Java
/// `WxOpenComponentServiceImpl.post(String, String)` 的默认
/// `accessTokenKey` 参数值）。
pub const ACCESS_TOKEN_KEY_COMPONENT: &str = "component_access_token";

/// 授权方 access_token 注入参数键（对应 Java
/// `WxOpenComponentServiceImpl.getAuthorizerAccessToken` 刷新链中
/// `api_authorizer_token` 接口的 token 键）。
pub const ACCESS_TOKEN_KEY_AUTHORIZER: &str = "authorizer_access_token";

/// 执行引擎校验的 token 参数前缀（对应 Java `executeInternal` 的
/// 「uri 参数中不允许有 access_token」校验，mp/ma 同一语义）。
pub const URI_TOKEN_PARAM_PREFIX: &str = "component_access_token=";
