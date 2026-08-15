//! G2 组子服务实现注册（外部联系人/客服/OA/会话存档/导出/会议/企业互联/
//! 智能机器人/人事助手）。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl` 包中 Wave 2b I2 组的 9 个子
//! 服务实现。模块文件位于 `api/impl/` 根目录（`wx_cp_<域>_service_impl.rs`，
//! 与任务文件布局一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以
//! `#[path]` 显式指回根目录文件（`api/impl/mod.rs` 由 I1 独占，注册行由
//! 协调者合并）。
//!
//! 共享测试基础设施（`test_support`，`#[cfg(test)]`）供各 impl 内嵌
//! `#[cfg(test)] mod tests` 复用：MockServer（极简 TCP HTTP 服务器）、
//! token 路由、配置与服务构建。

#[path = "wx_cp_corp_group_service_impl.rs"]
pub mod wx_cp_corp_group_service_impl;
#[path = "wx_cp_export_service_impl.rs"]
pub mod wx_cp_export_service_impl;
#[path = "wx_cp_external_contact_service_impl.rs"]
pub mod wx_cp_external_contact_service_impl;
#[path = "wx_cp_hr_service_impl.rs"]
pub mod wx_cp_hr_service_impl;
#[path = "wx_cp_intelligent_robot_service_impl.rs"]
pub mod wx_cp_intelligent_robot_service_impl;
#[path = "wx_cp_kf_service_impl.rs"]
pub mod wx_cp_kf_service_impl;
#[path = "wx_cp_meeting_service_impl.rs"]
pub mod wx_cp_meeting_service_impl;
#[path = "wx_cp_msg_audit_service_impl.rs"]
pub mod wx_cp_msg_audit_service_impl;
#[path = "wx_cp_oa_service_impl.rs"]
pub mod wx_cp_oa_service_impl;

pub use wx_cp_corp_group_service_impl::WxCpCorpGroupServiceImpl;
pub use wx_cp_export_service_impl::WxCpExportServiceImpl;
pub use wx_cp_external_contact_service_impl::WxCpExternalContactServiceImpl;
pub use wx_cp_hr_service_impl::WxCpHrServiceImpl;
pub use wx_cp_intelligent_robot_service_impl::WxCpIntelligentRobotServiceImpl;
pub use wx_cp_kf_service_impl::WxCpKfServiceImpl;
pub use wx_cp_meeting_service_impl::WxCpMeetingServiceImpl;
pub use wx_cp_msg_audit_service_impl::WxCpMsgAuditServiceImpl;
pub use wx_cp_oa_service_impl::WxCpOaServiceImpl;

// ---------------------------------------------------------------------------
// 共享测试基础设施（仅测试编译）
// ---------------------------------------------------------------------------

/// 共享测试支持（MockServer + 服务构建，供各 impl 内嵌测试模块复用）。
/// 镜像 Java `ApiTestModule` + `TestConfigStorage` 的职责：Guice 注入
/// 配置 → Rust 直接构造服务与 MockServer（模式照抄 qidian/miniapp
/// tests/ 目录）。
#[cfg(test)]
#[allow(dead_code)]
pub mod test_support {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex, Weak};

    use crate::api::WxCpService;
    use crate::api::r#impl::WxCpServiceImpl;
    use crate::config::r#impl::WxCpDefaultConfig;
    use crate::config::{WxCpConfigStorage, WxCpHostConfig};

    /// 极简 mock HTTP 服务器：按请求方法/路径返回 body，记录最近一次
    /// 请求的方法/路径（含 query）/请求体与请求计数（照抄 qidian tests/
    /// common 模式）。
    pub struct MockServer {
        addr: std::net::SocketAddr,
        requests: Arc<AtomicUsize>,
        last_method: Arc<Mutex<String>>,
        last_path: Arc<Mutex<String>>,
        last_body: Arc<Mutex<String>>,
        token_hits: Arc<AtomicUsize>,
        path_hits: Arc<Mutex<std::collections::HashMap<String, usize>>>,
        stop: Arc<AtomicBool>,
    }

    impl MockServer {
        /// 启动服务器（`handler(method, path) -> (content_type, body)`）。
        pub async fn start<F>(handler: F) -> Self
        where
            F: Fn(&str, &str) -> (String, String) + Send + Sync + 'static,
        {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("绑定端口");
            let addr = listener.local_addr().expect("获取地址");
            let requests = Arc::new(AtomicUsize::new(0));
            let last_method = Arc::new(Mutex::new(String::new()));
            let last_path = Arc::new(Mutex::new(String::new()));
            let last_body = Arc::new(Mutex::new(String::new()));
            let token_hits = Arc::new(AtomicUsize::new(0));
            let path_hits = Arc::new(Mutex::new(std::collections::HashMap::new()));
            let stop = Arc::new(AtomicBool::new(false));
            let handler = Arc::new(handler);

            let requests_clone = requests.clone();
            let last_method_clone = last_method.clone();
            let last_path_clone = last_path.clone();
            let last_body_clone = last_body.clone();
            let token_hits_clone = token_hits.clone();
            let path_hits_clone = path_hits.clone();
            let stop_clone = stop.clone();
            tokio::spawn(async move {
                loop {
                    if stop_clone.load(Ordering::SeqCst) {
                        break;
                    }
                    let Ok((mut socket, _)) = listener.accept().await else {
                        continue;
                    };
                    requests_clone.fetch_add(1, Ordering::SeqCst);
                    let handler = handler.clone();
                    let last_method_clone = last_method_clone.clone();
                    let last_path_clone = last_path_clone.clone();
                    let last_body_clone = last_body_clone.clone();
                    let token_hits_clone = token_hits_clone.clone();
                    let path_hits_clone = path_hits_clone.clone();
                    tokio::spawn(async move {
                        use tokio::io::{AsyncReadExt, AsyncWriteExt};
                        let mut buf = [0u8; 65536];
                        let n = socket.read(&mut buf).await.unwrap_or(0);
                        let request = String::from_utf8_lossy(&buf[..n]).to_string();
                        // 记录请求方法、路径（含 query）与请求体
                        let mut lines = request.lines();
                        let mut path = String::new();
                        if let Some(request_line) = lines.next() {
                            let mut parts = request_line.split_whitespace();
                            if let Some(method) = parts.next() {
                                *last_method_clone.lock().unwrap() = method.to_string();
                            }
                            if let Some(p) = parts.next() {
                                path = p.to_string();
                                *last_path_clone.lock().unwrap() = p.to_string();
                            }
                        }
                        if let Some(idx) = request.find("\r\n\r\n") {
                            let body = request[idx + 4..].to_string();
                            *last_body_clone.lock().unwrap() = body;
                        }
                        // token 接口命中计数（并发刷新去重断言用）
                        if path.contains("/cgi-bin/gettoken") {
                            token_hits_clone.fetch_add(1, Ordering::SeqCst);
                        }
                        {
                            let mut hits = path_hits_clone.lock().unwrap();
                            *hits.entry(path.clone()).or_insert(0) += 1;
                        }
                        let method = request
                            .lines()
                            .next()
                            .and_then(|l| l.split_whitespace().next())
                            .unwrap_or("GET")
                            .to_string();
                        let (content_type, body) = handler(&method, &path);
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            body.len(),
                            body
                        );
                        let _ = socket.write_all(response.as_bytes()).await;
                    });
                }
            });

            Self {
                addr,
                requests,
                last_method,
                last_path,
                last_body,
                token_hits,
                path_hits,
                stop,
            }
        }

        /// mock 服务器地址拼接（`path` 为空则仅主机）。
        pub fn url(&self, path: &str) -> String {
            format!("http://{}{}", self.addr, path)
        }

        /// 请求总计数。
        pub fn request_count(&self) -> usize {
            self.requests.load(Ordering::SeqCst)
        }

        /// token 接口命中计数。
        pub fn token_hits(&self) -> usize {
            self.token_hits.load(Ordering::SeqCst)
        }

        /// 最近一次请求方法。
        pub fn last_method(&self) -> String {
            self.last_method.lock().unwrap().clone()
        }

        /// 最近一次请求路径（含 query）。
        pub fn last_path(&self) -> String {
            self.last_path.lock().unwrap().clone()
        }

        /// 最近一次请求体。
        pub fn last_body(&self) -> String {
            self.last_body.lock().unwrap().clone()
        }

        /// 统计包含指定子串的请求路径命中次数。
        pub fn path_hits(&self, path_contains: &str) -> usize {
            self.path_hits
                .lock()
                .unwrap()
                .iter()
                .filter(|(p, _)| p.contains(path_contains))
                .map(|(_, n)| n)
                .sum()
        }
    }

    impl Drop for MockServer {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::SeqCst);
        }
    }

    /// JSON 响应快捷构造。
    pub fn json(body: &str) -> (String, String) {
        ("application/json".to_string(), body.to_string())
    }

    /// 通用路由 handler：token 请求先应答（按 corpsecret 区分普通/通讯录
    /// 同步/会话存档三类 token），业务路径按 contains 分派。
    pub fn dispatch(
        handler: impl Fn(&str) -> (String, String) + Send + Sync + 'static,
    ) -> impl Fn(&str, &str) -> (String, String) + Send + Sync + 'static {
        move |_method: &str, path: &str| {
            if path.contains("/cgi-bin/gettoken") {
                if path.contains("corpsecret=contact_secret") {
                    return json(r#"{"access_token":"CONTACT_TOKEN","expires_in":7200}"#);
                }
                if path.contains("corpsecret=msg_audit_secret") {
                    return json(r#"{"access_token":"MSG_AUDIT_TOKEN","expires_in":7200}"#);
                }
                return json(r#"{"access_token":"MOCK_TOKEN","expires_in":7200}"#);
            }
            handler(path)
        }
    }

    /// 指向 mock 服务器的主机配置（api_host 覆盖）。
    pub fn host_config(host: &str) -> WxCpHostConfig {
        let mut config = WxCpHostConfig::new();
        config.api_host = host.to_string();
        config
    }

    /// 构建指向 mock 服务器的默认配置（corpid=corpid, secret=secret,
    /// token=token123, agentid=101，含通讯录同步/会话存档 secret 与
    /// 会话存档私钥占位）。
    pub fn config_with_host(host: &str) -> Arc<dyn WxCpConfigStorage> {
        let mut config = WxCpDefaultConfig::new("corpid", "secret");
        config.set_token("token123");
        config.set_agent_id(Some(101));
        config.set_contact_secret("contact_secret");
        config.set_msg_audit_secret("msg_audit_secret");
        config.set_msg_audit_pri_key("PLACEHOLDER_PRI_KEY");
        config.set_host_config(host_config(host));
        Arc::new(config)
    }

    /// 构建指向 mock 服务器的门面服务。
    pub fn service_with_host(host: &str) -> Arc<WxCpServiceImpl> {
        WxCpServiceImpl::new_arc(config_with_host(host))
    }

    /// 门面服务转 Weak 引用（子服务构造参数）。
    pub fn weak_service(service: &Arc<WxCpServiceImpl>) -> Weak<dyn WxCpService> {
        let svc: Arc<dyn WxCpService> = service.clone();
        Arc::downgrade(&svc)
    }
}
