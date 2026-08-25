//! Coverage boost: `wx_open_default_config_impl.rs` (140 missed, 58.2%).

use wx_rust_open::config::WxOpenConfigStorage;
use wx_rust_open::config::r#impl::WxOpenDefaultConfig;

#[test]
fn new_default_values() {
    let cfg = WxOpenDefaultConfig::new();
    assert!(cfg.component_app_id().is_none());
    assert!(cfg.component_app_secret().is_none());
    assert!(cfg.component_verify_ticket().is_none());
    assert!(cfg.component_access_token().is_none());
    assert!(cfg.is_component_access_token_expired());
}

#[test]
fn setters_roundtrip() {
    let mut cfg = WxOpenDefaultConfig::new();
    cfg.set_component_app_id("app_id");
    assert_eq!(cfg.component_app_id(), Some("app_id".into()));
    cfg.set_component_app_secret("secret");
    assert_eq!(cfg.component_app_secret(), Some("secret".into()));
    cfg.set_component_token("token");
    assert_eq!(cfg.component_token(), Some("token".into()));
    cfg.set_component_aes_key("aes_key");
    assert_eq!(cfg.component_aes_key(), Some("aes_key".into()));
    cfg.set_component_verify_ticket("ticket");
    assert_eq!(cfg.component_verify_ticket(), Some("ticket".into()));
    cfg.set_http_proxy_host("proxy.example.com");
    assert_eq!(cfg.http_proxy_host(), Some("proxy.example.com".into()));
    cfg.set_http_proxy_port(8080);
    assert_eq!(cfg.http_proxy_port(), 8080);
    cfg.set_http_proxy_username("user");
    assert_eq!(cfg.http_proxy_username(), Some("user".into()));
    cfg.set_http_proxy_password("pass");
    assert_eq!(cfg.http_proxy_password(), Some("pass".into()));
    cfg.set_retry_sleep_millis(500);
    assert_eq!(cfg.retry_sleep_millis(), 500);
    cfg.set_max_retry_times(3);
    assert_eq!(cfg.max_retry_times(), 3);
}

#[test]
fn component_access_token_lifecycle() {
    let cfg = WxOpenDefaultConfig::new();
    assert!(cfg.component_access_token().is_none());
    assert!(cfg.is_component_access_token_expired());
    cfg.update_component_access_token_with_expiry("token_abc", 7200);
    assert_eq!(cfg.component_access_token(), Some("token_abc".into()));
    assert!(!cfg.is_component_access_token_expired());
    cfg.expire_component_access_token();
    assert!(cfg.is_component_access_token_expired());
}

#[test]
fn authorizer_access_token_lifecycle() {
    let cfg = WxOpenDefaultConfig::new();
    assert!(cfg.authorizer_access_token("auth_app_1").is_none());
    assert!(cfg.is_authorizer_access_token_expired("auth_app_1"));
    cfg.update_authorizer_access_token_with_expiry("auth_app_1", "at_123", 7200);
    assert_eq!(
        cfg.authorizer_access_token("auth_app_1"),
        Some("at_123".into())
    );
    assert!(!cfg.is_authorizer_access_token_expired("auth_app_1"));
    cfg.expire_authorizer_access_token("auth_app_1");
    assert!(cfg.is_authorizer_access_token_expired("auth_app_1"));
}

#[test]
fn authorizer_refresh_token() {
    let cfg = WxOpenDefaultConfig::new();
    assert!(cfg.authorizer_refresh_token("auth_app_1").is_none());
    cfg.update_authorizer_refresh_token("auth_app_1", "rt_123");
    assert_eq!(
        cfg.authorizer_refresh_token("auth_app_1"),
        Some("rt_123".into())
    );
}

#[test]
fn jsapi_ticket_lifecycle() {
    let cfg = WxOpenDefaultConfig::new();
    assert!(cfg.jsapi_ticket("auth_app_1").is_none());
    assert!(cfg.is_jsapi_ticket_expired("auth_app_1"));
    cfg.update_jsapi_ticket("auth_app_1", "ticket_1", 7200);
    assert_eq!(cfg.jsapi_ticket("auth_app_1"), Some("ticket_1".into()));
    assert!(!cfg.is_jsapi_ticket_expired("auth_app_1"));
    cfg.expire_jsapi_ticket("auth_app_1");
    assert!(cfg.is_jsapi_ticket_expired("auth_app_1"));
}

#[test]
fn card_api_ticket_lifecycle() {
    let cfg = WxOpenDefaultConfig::new();
    assert!(cfg.card_api_ticket("auth_app_1").is_none());
    assert!(cfg.is_card_api_ticket_expired("auth_app_1"));
    cfg.update_card_api_ticket("auth_app_1", "card_ticket_1", 7200);
    assert_eq!(
        cfg.card_api_ticket("auth_app_1"),
        Some("card_ticket_1".into())
    );
    assert!(!cfg.is_card_api_ticket_expired("auth_app_1"));
    cfg.expire_card_api_ticket("auth_app_1");
    assert!(cfg.is_card_api_ticket_expired("auth_app_1"));
}

#[test]
fn multiple_authorizers_independent() {
    let cfg = WxOpenDefaultConfig::new();
    cfg.update_authorizer_access_token_with_expiry("app_a", "at_a", 7200);
    cfg.update_authorizer_access_token_with_expiry("app_b", "at_b", 7200);
    assert_eq!(cfg.authorizer_access_token("app_a"), Some("at_a".into()));
    assert_eq!(cfg.authorizer_access_token("app_b"), Some("at_b".into()));
    cfg.expire_authorizer_access_token("app_a");
    assert!(cfg.authorizer_access_token("app_a").is_none());
    assert_eq!(cfg.authorizer_access_token("app_b"), Some("at_b".into()));
}

#[test]
fn api_host_url_and_access_token_url() {
    let mut cfg = WxOpenDefaultConfig::new();
    let _ = cfg.api_host_url();
    cfg.set_api_host_url("https://custom.api.com");
    assert_eq!(cfg.api_host_url(), Some("https://custom.api.com".into()));
    let _ = cfg.access_token_url();
    cfg.set_access_token_url("https://custom.token.com");
    assert_eq!(
        cfg.access_token_url(),
        Some("https://custom.token.com".into())
    );
}

#[test]
fn effective_api_host_url() {
    let cfg = WxOpenDefaultConfig::new();
    let url = cfg.effective_api_host_url();
    assert!(!url.is_empty());
}

#[test]
fn short_expiry_is_expired() {
    let cfg = WxOpenDefaultConfig::new();
    cfg.update_component_access_token_with_expiry("tok", 0);
    assert!(cfg.is_component_access_token_expired());
}

#[test]
fn builder_chaining() {
    let cfg = WxOpenDefaultConfig::new();
    cfg.set_component_app_id("a");
    cfg.set_component_app_secret("s");
    cfg.set_component_token("t");
    cfg.set_component_aes_key("k");
    assert_eq!(cfg.component_app_id(), Some("a".into()));
}

#[test]
fn debug_format() {
    let cfg = WxOpenDefaultConfig::new();
    let dbg = format!("{cfg:?}");
    assert!(dbg.contains("WxOpenDefaultConfig"));
}

#[tokio::test]
async fn component_access_token_lock() {
    let cfg = WxOpenDefaultConfig::new();
    let lock = cfg.component_access_token_lock();
    let _guard = lock.lock().await;
}

#[tokio::test]
async fn lock_by_key() {
    let cfg = WxOpenDefaultConfig::new();
    let lock = cfg.lock_by_key("test_key");
    let _guard = lock.lock().await;
}

#[test]
fn component_api_signature_fields() {
    let cfg = WxOpenDefaultConfig::new();
    let _ = cfg.component_api_signature_rsa_private_key();
    let _ = cfg.component_api_signature_aes_key();
    let _ = cfg.component_api_signature_rsa_private_key_sn();
    let _ = cfg.component_api_signature_aes_key_sn();
}

#[test]
fn update_authorizer_refresh_token() {
    let cfg = WxOpenDefaultConfig::new();
    cfg.update_authorizer_refresh_token("app_1", "rt_1");
    cfg.update_authorizer_refresh_token("app_1", "rt_2");
    assert_eq!(cfg.authorizer_refresh_token("app_1"), Some("rt_2".into()));
}
