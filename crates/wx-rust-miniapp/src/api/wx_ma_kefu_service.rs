//! 小程序客服管理接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaKefuService`。
//! 不同于 WxMaCustomserviceWorkService（企业微信客服绑定）与
//! WxMaMsgService.sendKefuMsg（发送客服消息），此接口专门处理小程序客服
//! 账号管理、会话管理等功能。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::kefu::request::WxMaKfAccountRequest;
use crate::bean::kefu::{WxMaKfList, WxMaKfSession, WxMaKfSessionList};

/// 小程序客服管理接口。
#[async_trait]
pub trait WxMaKefuService: Send + Sync {
    /// 获取客服基本信息（对应 Java `kfList()`）。
    async fn kf_list(&self) -> Result<WxMaKfList, WxErrorException>;

    /// 添加客服账号（对应 Java `kfAccountAdd(WxMaKfAccountRequest)`）。
    async fn kf_account_add(
        &self,
        request: &WxMaKfAccountRequest,
    ) -> Result<bool, WxErrorException>;

    /// 修改客服账号（对应 Java `kfAccountUpdate(WxMaKfAccountRequest)`）。
    async fn kf_account_update(
        &self,
        request: &WxMaKfAccountRequest,
    ) -> Result<bool, WxErrorException>;

    /// 删除客服账号（对应 Java `kfAccountDel(String)`）。
    async fn kf_account_del(&self, kf_account: &str) -> Result<bool, WxErrorException>;

    /// 创建会话（对应 Java `kfSessionCreate(String, String)`）。
    async fn kf_session_create(
        &self,
        openid: &str,
        kf_account: &str,
    ) -> Result<bool, WxErrorException>;

    /// 关闭会话（对应 Java `kfSessionClose(String, String)`）。
    async fn kf_session_close(
        &self,
        openid: &str,
        kf_account: &str,
    ) -> Result<bool, WxErrorException>;

    /// 获取客户的会话状态（对应 Java `kfSessionGet(String)`）。
    async fn kf_session_get(&self, openid: &str) -> Result<WxMaKfSession, WxErrorException>;

    /// 获取客服的会话列表（对应 Java `kfSessionList(String)`）。
    async fn kf_session_list(
        &self,
        kf_account: &str,
    ) -> Result<WxMaKfSessionList, WxErrorException>;
}
