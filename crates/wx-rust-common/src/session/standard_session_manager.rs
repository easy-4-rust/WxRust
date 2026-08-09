//! 标准会话管理器。
//!
//! 对应 Java `me.chanjar.weixin.common.session.StandardSessionManager`（内存实现）。

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::standard_session::StandardSession;
use super::wx_session::WxSession;
use super::wx_session_manager::WxSessionManager;

/// 标准会话管理器（内存实现）。
///
/// 对应 Java `StandardSessionManager`：维护 sessionId → 会话的映射，
/// 支持 `getSession(id)` 与 `getSession(id, create)` 语义。
/// 同一 sessionId 返回同一会话对象（`Arc` 共享，对应 Java 引用同一性）。
#[derive(Debug, Default)]
pub struct StandardSessionManager {
    sessions: Mutex<HashMap<String, Arc<StandardSession>>>,
    /// 会话最长不活动时间
    max_inactive_interval: Duration,
}

impl StandardSessionManager {
    /// 构建默认配置的会话管理器（30 分钟不活动过期）。
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            max_inactive_interval: Duration::from_secs(30 * 60),
        }
    }

    /// 构建自定义不活动时间的会话管理器。
    ///
    /// # 参数
    /// - `max_inactive_interval`：最长不活动时间
    pub fn with_max_inactive(max_inactive_interval: Duration) -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            max_inactive_interval,
        }
    }

    /// 返回当前活跃会话数（未失效会话）。
    ///
    /// 对应 Java `InternalSessionManager.getActiveSessions`。
    pub fn active_sessions(&self) -> usize {
        let map = self.sessions.lock().unwrap();
        map.values().filter(|s| s.is_valid()).count()
    }

    /// 清理超过最长不活动时间的会话。
    ///
    /// 对应 Java `StandardSessionManager` 后台处理器（`backgroundProcessorDelay`）
    /// 的清理动作；Rust 以显式调用表达（ADAPTED：无后台线程）。
    pub fn expire_inactive_sessions(&self) {
        let mut map = self.sessions.lock().unwrap();
        map.retain(|_, s| !s.is_expired());
    }

    /// 标记会话访问结束（对应 Java `InternalSession.endAccess`）。
    ///
    /// 更新会话的最后访问时间，使不活动计时从此刻重新开始。
    pub fn end_access(&self, session_id: &str) {
        let map = self.sessions.lock().unwrap();
        if let Some(s) = map.get(session_id) {
            s.touch_access();
        }
    }

    /// 返回最长不活动时间（测试断言用）。
    pub fn max_inactive_interval(&self) -> Duration {
        self.max_inactive_interval
    }
}

impl WxSessionManager for StandardSessionManager {
    fn get_session(&self, session_id: &str) -> Arc<dyn WxSession> {
        let mut map = self.sessions.lock().unwrap();
        // 已存在且有效则复用；失效（invalidate/超时）则重建（对应 Java 语义）
        let need_new = match map.get(session_id) {
            Some(s) => !s.is_valid(),
            None => true,
        };
        if need_new {
            map.insert(
                session_id.to_string(),
                Arc::new(StandardSession::new(
                    session_id,
                    Some(self.max_inactive_interval),
                )),
            );
        }
        map.get(session_id).unwrap().clone()
    }

    fn get_session_or_create(&self, session_id: &str, create: bool) -> Option<Arc<dyn WxSession>> {
        let mut map = self.sessions.lock().unwrap();
        if let Some(s) = map.get(session_id) {
            if s.is_valid() {
                return Some(s.clone());
            }
            map.remove(session_id);
        }
        if create {
            let s = Arc::new(StandardSession::new(
                session_id,
                Some(self.max_inactive_interval),
            ));
            map.insert(session_id.to_string(), s);
            return Some(map.get(session_id).unwrap().clone());
        }
        None
    }
}
