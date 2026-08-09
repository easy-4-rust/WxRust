//! 微信会话管理器。
//!
//! 对应 Java `me.chanjar.weixin.common.session.WxSessionManager`。

use std::sync::Arc;

use super::wx_session::WxSession;

/// 微信会话管理器。
pub trait WxSessionManager: Send + Sync {
    /// 获取某个 sessionId 对应的 session；如果不存在则新建一个并返回。
    ///
    /// # 参数
    /// - `session_id`：会话 ID
    ///
    /// # 返回
    /// 会话对象（不存在时新建）。同一 sessionId 多次调用返回同一对象
    /// （对应 Java `session1 == session2` 语义）。
    fn get_session(&self, session_id: &str) -> Arc<dyn WxSession>;

    /// 获取某个 sessionId 对应的 session。
    ///
    /// # 参数
    /// - `session_id`：会话 ID
    /// - `create`：为 `true` 时不存在则新建；为 `false` 时不存在返回 `None`
    ///
    /// # 返回
    /// 会话对象；`create=false` 且不存在时返回 `None`。
    fn get_session_or_create(&self, session_id: &str, create: bool) -> Option<Arc<dyn WxSession>>;

    /// 标记会话访问结束（对应 Java `InternalSession.endAccess`）。
    ///
    /// 由消息路由器在规则处理完毕后调用，使不活动计时从此刻重新开始。
    /// 默认无操作；具体管理器可按需实现。
    fn end_access(&self, _session_id: &str) {}
}
