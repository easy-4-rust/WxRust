//! jsapi 相关服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaJsapiService`。

use async_trait::async_trait;
use wx_rust_common::bean::WxJsapiSignature;
use wx_rust_common::error::WxErrorException;

/// jsapi 相关服务。
#[async_trait]
pub trait WxMaJsapiService: Send + Sync {
    /// 获得卡券 api_ticket，不强制刷新（对应 Java
    /// `WxMaJsapiService.getCardApiTicket()`）。
    async fn get_card_api_ticket(&self) -> Result<String, WxErrorException>;

    /// 获得卡券 api_ticket（对应 Java
    /// `WxMaJsapiService.getCardApiTicket(boolean)`）。
    ///
    /// 获得时会检查 api_ticket 是否过期，过期则刷新；`force_refresh` 时强制
    /// 过期后刷新（对应 `?type=wx_card`）。
    async fn get_card_api_ticket_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException>;

    /// 获得 jsapi_ticket，不强制刷新（对应 Java
    /// `WxMaJsapiService.getJsapiTicket()`）。
    async fn get_jsapi_ticket(&self) -> Result<String, WxErrorException>;

    /// 获得 jsapi_ticket（对应 Java
    /// `WxMaJsapiService.getJsapiTicket(boolean)`）。
    ///
    /// 获得时会检查 jsapi_ticket 是否过期，过期则刷新；`force_refresh` 时强制
    /// 过期后刷新（对应 `?type=jsapi`）。
    async fn get_jsapi_ticket_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException>;

    /// 创建调用 jsapi 时所需要的签名（对应 Java
    /// `WxMaJsapiService.createJsapiSignature(String)`）。
    ///
    /// `jsapi_ticket=`/`noncestr=`/`timestamp=`/`url=` 排序拼接后 SHA1。
    async fn create_jsapi_signature(&self, url: &str)
    -> Result<WxJsapiSignature, WxErrorException>;
}
