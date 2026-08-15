//! 公众号客服服务。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpKefuService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::kefu::WxMpKefuMessage;
use crate::bean::kefu::request::WxMpKfAccountRequest;
use crate::bean::kefu::request::WxMpKfSessionRequest;
use crate::bean::kefu::result::{
    WxMpKfList, WxMpKfMsgList, WxMpKfOnlineList, WxMpKfSessionGetResult, WxMpKfSessionList,
    WxMpKfSessionWaitCaseList,
};

/// 客服服务。
#[async_trait]
pub trait WxMpKefuService: Send + Sync {
    /// 发送客服消息（对应 Java `sendKefuMessage`）。
    async fn send_kefu_message(
        &self,
        message: &WxMpKefuMessage,
    ) -> Result<String, WxErrorException>;

    /// 获取客服账号列表（对应 Java `kfList`）。
    async fn kf_list(&self) -> Result<WxMpKfList, WxErrorException>;

    /// 获取在线客服列表（对应 Java `kfOnlineList`）。
    async fn kf_online_list(&self) -> Result<WxMpKfOnlineList, WxErrorException>;

    /// 添加客服账号（对应 Java `kfAccountAdd`）。
    async fn kf_account_add(
        &self,
        request: &WxMpKfAccountRequest,
    ) -> Result<bool, WxErrorException>;

    /// 更新客服账号（对应 Java `kfAccountUpdate`）。
    async fn kf_account_update(
        &self,
        request: &WxMpKfAccountRequest,
    ) -> Result<bool, WxErrorException>;

    /// 邀请绑定客服（对应 Java `kfAccountInviteWorker`）。
    async fn kf_account_invite_worker(
        &self,
        request: &WxMpKfAccountRequest,
    ) -> Result<bool, WxErrorException>;

    /// 删除客服账号（对应 Java `kfAccountDel`）。
    async fn kf_account_del(&self, kf_account: &str) -> Result<bool, WxErrorException>;

    /// 创建客服会话（对应 Java `kfSessionCreate`）。
    async fn kf_session_create(
        &self,
        request: &WxMpKfSessionRequest,
    ) -> Result<bool, WxErrorException>;

    /// 关闭客服会话（对应 Java `kfSessionClose`）。
    async fn kf_session_close(
        &self,
        request: &WxMpKfSessionRequest,
    ) -> Result<bool, WxErrorException>;

    /// 获取客服会话（对应 Java `kfSessionGet`）。
    async fn kf_session_get(
        &self,
        openid: &str,
    ) -> Result<WxMpKfSessionGetResult, WxErrorException>;

    /// 获取客服会话列表（对应 Java `kfSessionList`）。
    async fn kf_session_list(
        &self,
        kf_account: &str,
    ) -> Result<WxMpKfSessionList, WxErrorException>;

    /// 获取未接入会话列表（对应 Java `kfSessionGetWaitCase`）。
    async fn kf_session_get_wait_case(&self)
    -> Result<WxMpKfSessionWaitCaseList, WxErrorException>;

    /// 获取客服聊天记录（对应 Java `kfMsgList`）。
    async fn kf_msg_list(
        &self,
        start_time: i64,
        end_time: i64,
        msg_id: i64,
        number: i32,
    ) -> Result<WxMpKfMsgList, WxErrorException>;
}
