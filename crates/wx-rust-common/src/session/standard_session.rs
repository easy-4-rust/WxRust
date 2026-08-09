//! 标准会话实现。
//!
//! 对应 Java `me.chanjar.weixin.common.session.StandardSession`（内存实现）。
//! Java 的 StandardSession/StandardSessionFacade/InternalSession/Constants/
//! TooManyActiveSessionsException 在 Rust 中合并为单个结构体 + trait 实现。

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::wx_session::WxSession;

/// 标准微信会话（内存实现）。
///
/// 承载会话属性，支持过期（默认 30 分钟，对应 Java `StandardSession` 的
/// maxInactiveInterval 语义）。
#[derive(Debug)]
pub struct StandardSession {
    id: String,
    attributes: Mutex<HashMap<String, String>>,
    created: Instant,
    last_access: Mutex<Instant>,
    max_inactive_interval: Duration,
    valid: Mutex<bool>,
}

impl StandardSession {
    /// 构建标准会话。
    ///
    /// # 参数
    /// - `id`：会话 ID
    /// - `max_inactive_interval`：最长不活动时间；`None` 为 30 分钟默认值
    pub fn new(id: impl Into<String>, max_inactive_interval: Option<Duration>) -> Self {
        Self {
            id: id.into(),
            attributes: Mutex::new(HashMap::new()),
            created: Instant::now(),
            last_access: Mutex::new(Instant::now()),
            max_inactive_interval: max_inactive_interval.unwrap_or(Duration::from_secs(30 * 60)),
            valid: Mutex::new(true),
        }
    }
}

impl WxSession for StandardSession {
    fn id(&self) -> &str {
        &self.id
    }

    fn get_attribute(&self, name: &str) -> Option<String> {
        if !self.is_valid() {
            return None;
        }
        self.attributes.lock().unwrap().get(name).cloned()
    }

    fn set_attribute(&self, name: &str, value: String) {
        if !self.is_valid() {
            return;
        }
        self.attributes
            .lock()
            .unwrap()
            .insert(name.to_string(), value);
        *self.last_access.lock().unwrap() = Instant::now();
    }

    fn remove_attribute(&self, name: &str) {
        if !self.is_valid() {
            return;
        }
        self.attributes.lock().unwrap().remove(name);
        *self.last_access.lock().unwrap() = Instant::now();
    }

    fn is_valid(&self) -> bool {
        let valid = *self.valid.lock().unwrap();
        if !valid {
            return false;
        }
        // 检查是否超过最长不活动时间
        let last = *self.last_access.lock().unwrap();
        last.elapsed() < self.max_inactive_interval
    }

    fn invalidate(&self) {
        *self.valid.lock().unwrap() = false;
        self.attributes.lock().unwrap().clear();
    }

    fn attribute_names(&self) -> Vec<String> {
        if !self.is_valid() {
            return Vec::new();
        }
        self.attributes.lock().unwrap().keys().cloned().collect()
    }
}

impl StandardSession {
    /// 会话是否超过最长不活动时间（供管理器清理）。
    pub(crate) fn is_expired(&self) -> bool {
        !self.is_valid()
    }

    /// 标记访问结束：更新最后访问时间（对应 Java `endAccess`）。
    pub(crate) fn touch_access(&self) {
        *self.last_access.lock().unwrap() = Instant::now();
    }

    /// 返回会话的独立副本（属性深拷贝）。
    pub fn clone_session(&self) -> StandardSession {
        StandardSession {
            id: self.id.clone(),
            attributes: Mutex::new(self.attributes.lock().unwrap().clone()),
            created: self.created,
            last_access: Mutex::new(*self.last_access.lock().unwrap()),
            max_inactive_interval: self.max_inactive_interval,
            valid: Mutex::new(*self.valid.lock().unwrap()),
        }
    }
}
