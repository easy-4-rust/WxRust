//! 小程序 - 微信客服相关接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaCustomserviceWorkService`。
//! 负责处理 https://api.weixin.qq.com/customservice/work/**。
//! 绑定的企业 ID 需和小程序主体一致，目前仅支持绑定非个人小程序。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::customservice::WxMaCustomserviceResult;

/// 小程序微信客服服务。
#[async_trait]
pub trait WxMaCustomserviceWorkService: Send + Sync {
    /// 查询小程序的微信客服绑定情况（对应 Java `getCustomservice()`）。
    async fn get_customservice(&self) -> Result<WxMaCustomserviceResult, WxErrorException>;

    /// 为小程序绑定微信客服（对应 Java `bindCustomservice(String corpid)`，
    /// 绑定的企业 ID 需完成企业认证）。
    async fn bind_customservice(
        &self,
        corpid: &str,
    ) -> Result<WxMaCustomserviceResult, WxErrorException>;

    /// 为小程序解除绑定微信客服（对应 Java `unbindCustomservice(String corpid)`）。
    async fn unbind_customservice(
        &self,
        corpid: &str,
    ) -> Result<WxMaCustomserviceResult, WxErrorException>;
}
