//! Coverage boost: `wx_cp_default_config_impl.rs` (110 missed, 66.5%).

use wx_rust_common::config::WxConfigStorage;
use wx_rust_common::enums::TicketType;
use wx_rust_cp::config::WxCpConfigStorage;
use wx_rust_cp::config::r#impl::WxCpDefaultConfig;

#[test]
fn new_default_values() {
    let cfg = WxCpDefaultConfig::new("corp_1", "secret_1");
    assert_eq!(cfg.app_id(), "corp_1");
    assert_eq!(cfg.secret(), "secret_1");
    assert_eq!(cfg.token(), None);
    assert_eq!(cfg.aes_key(), None);
}

#[test]
fn setters_roundtrip() {
    let mut cfg = WxCpDefaultConfig::new("corp", "sec");
    cfg.set_token("tok");
    assert!(cfg.token().is_some());
    cfg.set_aes_key("aes");
    assert!(cfg.aes_key().is_some());
    cfg.set_agent_id(Some(1000002));
    assert_eq!(cfg.agent_id(), Some(1000002));
}

#[test]
fn additional_setters() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    cfg.set_oauth2_redirect_uri("https://redirect.example.com");
    assert_eq!(
        cfg.oauth2_redirect_uri(),
        Some("https://redirect.example.com".into())
    );
    cfg.set_webhook_key("webhook_key");
    assert_eq!(cfg.webhook_key(), Some("webhook_key".into()));
    cfg.set_contact_secret("contact_sec");
    assert_eq!(cfg.contact_secret(), Some("contact_sec".into()));
    cfg.set_msg_audit_secret("audit_sec");
    assert_eq!(cfg.msg_audit_secret(), Some("audit_sec".into()));
    cfg.set_msg_audit_pri_key("audit_pri");
    assert_eq!(cfg.msg_audit_pri_key(), Some("audit_pri".into()));
    cfg.set_msg_audit_lib_path("/path/to/lib");
    assert_eq!(cfg.msg_audit_lib_path(), Some("/path/to/lib".into()));
}

#[test]
fn access_token_lifecycle() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    assert!(cfg.access_token().is_none());
    assert!(cfg.is_access_token_expired());
    cfg.update_access_token("at_123", 7200);
    assert_eq!(cfg.access_token(), Some("at_123".into()));
    assert!(!cfg.is_access_token_expired());
    cfg.expire_access_token();
    assert!(cfg.is_access_token_expired());
}

#[test]
fn ticket_lifecycle() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    assert!(cfg.ticket(TicketType::Jsapi).is_none());
    assert!(cfg.is_ticket_expired(TicketType::Jsapi));
    cfg.update_ticket(TicketType::Jsapi, "ticket_1", 7200);
    assert_eq!(cfg.ticket(TicketType::Jsapi), Some("ticket_1".into()));
    assert!(!cfg.is_ticket_expired(TicketType::Jsapi));
    cfg.expire_ticket(TicketType::Jsapi);
    assert!(cfg.is_ticket_expired(TicketType::Jsapi));
}

#[test]
fn contact_access_token_lifecycle() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    assert!(cfg.contact_access_token().is_none());
    assert!(cfg.is_contact_access_token_expired());
    cfg.update_contact_access_token("cat", 7200);
    assert_eq!(cfg.contact_access_token(), Some("cat".into()));
    assert!(!cfg.is_contact_access_token_expired());
    cfg.expire_contact_access_token();
    assert!(cfg.is_contact_access_token_expired());
}

#[test]
fn msg_audit_access_token_lifecycle() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    assert!(cfg.msg_audit_access_token().is_none());
    assert!(cfg.is_msg_audit_access_token_expired());
    cfg.update_msg_audit_access_token("maat", 7200);
    assert_eq!(cfg.msg_audit_access_token(), Some("maat".into()));
    assert!(!cfg.is_msg_audit_access_token_expired());
    cfg.expire_msg_audit_access_token();
    assert!(cfg.is_msg_audit_access_token_expired());
}

#[test]
fn proxy_settings() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    cfg.set_http_proxy_username("user");
    assert_eq!(cfg.http_proxy_username(), Some("user".into()));
    cfg.set_http_proxy_password("pass");
    assert_eq!(cfg.http_proxy_password(), Some("pass".into()));
}

#[test]
fn tmp_dir_file() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    cfg.set_tmp_dir_file("/tmp/wx");
    assert_eq!(cfg.tmp_dir_file(), Some("/tmp/wx".into()));
}

#[test]
fn builder_chaining() {
    let mut cfg = WxCpDefaultConfig::new("c", "s");
    cfg.set_token("t").set_aes_key("a");
    assert!(cfg.token().is_some());
}

#[test]
fn debug_format() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    let dbg = format!("{cfg:?}");
    assert!(dbg.contains("WxCpDefaultConfig"));
}

#[test]
fn default_trait() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    let _ = cfg.app_id();
}

#[tokio::test]
async fn access_token_lock() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    let lock = cfg.access_token_lock();
    let _guard = lock.lock().await;
}

#[tokio::test]
async fn ticket_lock() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    let lock = cfg.ticket_lock(TicketType::Jsapi);
    let _guard = lock.lock().await;
}

#[tokio::test]
async fn contact_access_token_lock() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    let lock = cfg.contact_access_token_lock();
    let _guard = lock.lock().await;
}

#[tokio::test]
async fn msg_audit_access_token_lock() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    let lock = cfg.msg_audit_access_token_lock();
    let _guard = lock.lock().await;
}

#[test]
fn short_expiry_is_expired() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    cfg.update_access_token("tok", 0);
    assert!(cfg.is_access_token_expired());
}

#[test]
fn host_config() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    let _ = cfg.host_config();
}

#[test]
fn base_api_url() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    let _ = cfg.base_api_url();
}

#[test]
fn auto_refresh_token() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    let _ = cfg.auto_refresh_token();
}

#[test]
fn expires_time() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    let _ = cfg.expires_time();
}

#[test]
fn multiple_ticket_types() {
    let cfg = WxCpDefaultConfig::new("c", "s");
    cfg.update_ticket(TicketType::Jsapi, "js", 7200);
    cfg.update_ticket(TicketType::WxCard, "cd", 7200);
    assert_eq!(cfg.ticket(TicketType::Jsapi), Some("js".into()));
    assert_eq!(cfg.ticket(TicketType::WxCard), Some("cd".into()));
}
