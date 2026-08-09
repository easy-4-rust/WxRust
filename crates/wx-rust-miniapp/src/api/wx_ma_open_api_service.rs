//! openApi 管理服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaOpenApiService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::openapi::{WxMiniGetApiQuotaResult, WxMiniGetRidInfoResult};

/// openApi 管理服务。
///
/// 文档：
/// <https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/openApi-mgnt/clearQuota.html>
#[async_trait]
pub trait WxMaOpenApiService: Send + Sync {
    /// 清空小程序等接口的每日调用接口次数（对应 Java
    /// `WxMaOpenApiService.clearQuota()`）。
    ///
    /// POST `/cgi-bin/clear_quota`，请求体 `{"appid": ...}`；成功返回 true。
    async fn clear_quota(&self) -> Result<bool, WxErrorException>;

    /// 查询 API 调用额度（对应 Java
    /// `WxMaOpenApiService.getApiQuota(String)`）。
    ///
    /// POST `/cgi-bin/openapi/quota/get`，请求体 `{"cgi_path": ...}`。
    async fn get_api_quota(
        &self,
        cgi_path: &str,
    ) -> Result<WxMiniGetApiQuotaResult, WxErrorException>;

    /// 查询 rid 信息（对应 Java `WxMaOpenApiService.getRidInfo(String)`）。
    ///
    /// POST `/cgi-bin/openapi/rid/get`，请求体 `{"rid": ...}`；响应无
    /// `request` 字段时返回 `None`（Java 返回 null）。
    async fn get_rid_info(
        &self,
        rid: &str,
    ) -> Result<Option<WxMiniGetRidInfoResult>, WxErrorException>;

    /// 使用 AppSecret 重置 API 调用次数（对应 Java
    /// `WxMaOpenApiService.clearQuotaByAppSecret()`）。
    ///
    /// POST `/cgi-bin/clear_quota/v2?appid=&appsecret=`，请求体为空；成功返回
    /// true。
    async fn clear_quota_by_app_secret(&self) -> Result<bool, WxErrorException>;
}
