//! Coverage boost: `wx_cp_tp_default_config_impl.rs` (300 lines, 0% covered).
//!
//! Exercises all public methods on `WxCpTpDefaultConfig`:
//! - setters (corp_id, suite_id, suite_secret, token, encoding_aes_key, proxy, tmp_dir)
//! - `WxCpTpConfigStorage` trait methods (token lifecycle, auth_corp maps, locks)

// (unused import removed)

use wx_rust_cp::config::WxCpTpConfigStorage;
use wx_rust_cp::config::r#impl::WxCpTpDefaultConfig;

#[test]
fn new_default_has_empty_strings() {
    let cfg = WxCpTpDefaultConfig::new();
    assert_eq!(cfg.corp_id(), "");
    assert_eq!(cfg.suite_id(), "");
    assert_eq!(cfg.suite_secret(), "");
    assert_eq!(cfg.corp_secret(), "");
    assert_eq!(cfg.provider_secret(), "");
    assert!(cfg.token().is_none());
    assert!(cfg.encoding_aes_key().is_none());
    assert!(cfg.suite_access_token().is_none());
    assert!(cfg.suite_ticket().is_none());
    assert!(cfg.provider_token().is_none());
    assert!(cfg.base_api_url().is_none());
    assert!(cfg.http_proxy_host().is_none());
    assert_eq!(cfg.http_proxy_port(), 0);
    assert!(cfg.http_proxy_username().is_none());
    assert!(cfg.http_proxy_password().is_none());
    assert!(cfg.tmp_dir_file().is_none());
}

#[test]
fn default_trait_equals_new() {
    let cfg = WxCpTpDefaultConfig::default();
    assert_eq!(cfg.corp_id(), "");
}

#[test]
fn setters_roundtrip() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.set_corp_id("corp_1");
    assert_eq!(cfg.corp_id(), "corp_1");

    cfg.set_suite_id("suite_1");
    assert_eq!(cfg.suite_id(), "suite_1");

    cfg.set_suite_secret("secret_1");
    assert_eq!(cfg.suite_secret(), "secret_1");

    cfg.set_token("token_1");
    assert_eq!(cfg.token(), Some("token_1".into()));

    cfg.set_encoding_aes_key("aes_key_1");
    assert_eq!(cfg.encoding_aes_key(), Some("aes_key_1".into()));

    cfg.set_http_proxy_host("proxy.example.com");
    assert_eq!(cfg.http_proxy_host(), Some("proxy.example.com".into()));

    cfg.set_http_proxy_port(8080);
    assert_eq!(cfg.http_proxy_port(), 8080);

    cfg.set_http_proxy_username("user");
    assert_eq!(cfg.http_proxy_username(), Some("user".into()));

    cfg.set_http_proxy_password("pass");
    assert_eq!(cfg.http_proxy_password(), Some("pass".into()));

    cfg.set_tmp_dir_file("/tmp/wx");
    assert_eq!(cfg.tmp_dir_file(), Some("/tmp/wx".into()));
}

#[test]
fn provider_secret_setter() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.set_provider_secret("ps_123");
    assert_eq!(cfg.provider_secret(), "ps_123");
    // corp_secret() returns provider_secret
    assert_eq!(cfg.corp_secret(), "ps_123");
}

#[test]
fn base_api_url_setter() {
    let cfg = WxCpTpDefaultConfig::new();
    assert!(cfg.base_api_url().is_none());
    cfg.set_base_api_url("https://api.example.com");
    assert_eq!(cfg.base_api_url(), Some("https://api.example.com".into()));
}

// ---- suite_access_token lifecycle ----

#[test]
fn suite_access_token_initially_none() {
    let cfg = WxCpTpDefaultConfig::new();
    assert!(cfg.suite_access_token().is_none());
    assert!(cfg.is_suite_access_token_expired());
}

#[test]
fn set_suite_access_token_direct() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.set_suite_access_token("token_value");
    assert_eq!(cfg.suite_access_token(), Some("token_value".into()));
    // Direct set has no expires_at, so entity should have expires_in based on now
    let entity = cfg.suite_access_token_entity();
    assert_eq!(entity.access_token, "token_value");
}

#[test]
fn update_suite_access_token_with_expiry() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_suite_access_token("new_token", 7200);
    assert_eq!(cfg.suite_access_token(), Some("new_token".into()));
    // Not expired (7200 - 200 reserve = 5200 seconds remaining)
    assert!(!cfg.is_suite_access_token_expired());
    let entity = cfg.suite_access_token_entity();
    assert!(entity.expires_in > 0);
}

#[test]
fn expire_suite_access_token() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_suite_access_token("token", 7200);
    assert!(!cfg.is_suite_access_token_expired());
    cfg.expire_suite_access_token();
    assert!(cfg.is_suite_access_token_expired());
    assert!(cfg.suite_access_token().is_none());
}

// ---- suite_ticket lifecycle ----

#[test]
fn suite_ticket_initially_none() {
    let cfg = WxCpTpDefaultConfig::new();
    assert!(cfg.suite_ticket().is_none());
    assert!(cfg.is_suite_ticket_expired());
}

#[test]
fn set_suite_ticket_direct() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.set_suite_ticket("ticket_value");
    assert_eq!(cfg.suite_ticket(), Some("ticket_value".into()));
}

#[test]
fn update_suite_ticket_with_expiry() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_suite_ticket("ticket123", 1800);
    assert_eq!(cfg.suite_ticket(), Some("ticket123".into()));
    assert!(!cfg.is_suite_ticket_expired());
}

#[test]
fn expire_suite_ticket() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_suite_ticket("t", 1800);
    cfg.expire_suite_ticket();
    assert!(cfg.is_suite_ticket_expired());
}

// ---- provider_token lifecycle (no 200s reserve) ----

#[test]
fn provider_token_initially_none() {
    let cfg = WxCpTpDefaultConfig::new();
    assert!(cfg.provider_token().is_none());
    assert!(cfg.is_provider_token_expired());
}

#[test]
fn update_provider_token_no_reserve() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_provider_token("prov_token", 7200);
    assert_eq!(cfg.provider_token(), Some("prov_token".into()));
    assert!(!cfg.is_provider_token_expired());
    let entity = cfg.suite_access_token_entity(); // just make sure it doesn't panic
    let _ = entity;
}

#[test]
fn expire_provider_token() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_provider_token("pt", 7200);
    cfg.expire_provider_token();
    assert!(cfg.is_provider_token_expired());
}

// ---- auth_corp access token (per-corp cache) ----

#[test]
fn auth_corp_access_token_initially_none() {
    let cfg = WxCpTpDefaultConfig::new();
    assert!(cfg.access_token("corp_a").is_none());
    assert!(cfg.is_access_token_expired("corp_a"));
}

#[test]
fn auth_corp_access_token_update_and_get() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_access_token("corp_a", "at_123", 7200);
    assert_eq!(cfg.access_token("corp_a"), Some("at_123".into()));
    assert!(!cfg.is_access_token_expired("corp_a"));
    let entity = cfg.access_token_entity("corp_a");
    assert_eq!(entity.access_token, "at_123");
}

#[test]
fn auth_corp_access_token_expire() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_access_token("corp_a", "at", 7200);
    cfg.expire_access_token("corp_a");
    assert!(cfg.is_access_token_expired("corp_a"));
    assert!(cfg.access_token("corp_a").is_none());
}

#[test]
fn auth_corp_access_token_multiple_corps_independent() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_access_token("corp_a", "at_a", 7200);
    cfg.update_access_token("corp_b", "at_b", 7200);
    assert_eq!(cfg.access_token("corp_a"), Some("at_a".into()));
    assert_eq!(cfg.access_token("corp_b"), Some("at_b".into()));
    cfg.expire_access_token("corp_a");
    assert!(cfg.access_token("corp_a").is_none());
    assert_eq!(cfg.access_token("corp_b"), Some("at_b".into()));
}

// ---- auth_corp js_api_ticket ----

#[test]
fn auth_corp_js_api_ticket_initially_none() {
    let cfg = WxCpTpDefaultConfig::new();
    assert!(cfg.auth_corp_js_api_ticket("corp_a").is_none());
    assert!(cfg.is_auth_corp_js_api_ticket_expired("corp_a"));
}

#[test]
fn auth_corp_js_api_ticket_update_and_expire() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_auth_corp_js_api_ticket("corp_a", "ticket_1", 7200);
    assert_eq!(
        cfg.auth_corp_js_api_ticket("corp_a"),
        Some("ticket_1".into())
    );
    assert!(!cfg.is_auth_corp_js_api_ticket_expired("corp_a"));
    cfg.expire_auth_corp_js_api_ticket("corp_a");
    assert!(cfg.is_auth_corp_js_api_ticket_expired("corp_a"));
}

// ---- auth_suite js_api_ticket ----

#[test]
fn auth_suite_js_api_ticket_initially_none() {
    let cfg = WxCpTpDefaultConfig::new();
    assert!(cfg.auth_suite_js_api_ticket("corp_a").is_none());
    assert!(cfg.is_auth_suite_js_api_ticket_expired("corp_a"));
}

#[test]
fn auth_suite_js_api_ticket_update_and_expire() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_auth_suite_js_api_ticket("corp_a", "sticket_1", 7200);
    assert_eq!(
        cfg.auth_suite_js_api_ticket("corp_a"),
        Some("sticket_1".into())
    );
    assert!(!cfg.is_auth_suite_js_api_ticket_expired("corp_a"));
    cfg.expire_auth_suite_js_api_ticket("corp_a");
    assert!(cfg.is_auth_suite_js_api_ticket_expired("corp_a"));
}

// ---- locks ----

#[tokio::test]
async fn provider_access_token_lock_is_some() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.set_suite_id("s1");
    cfg.set_corp_id("c1");
    let lock = cfg.provider_access_token_lock();
    let _guard = lock.lock().await;
    // lock acquired successfully
}

#[tokio::test]
async fn suite_access_token_lock_is_some() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.set_suite_id("s1");
    let lock = cfg.suite_access_token_lock();
    let _guard = lock.lock().await;
}

#[tokio::test]
async fn access_token_lock_is_some() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.set_suite_id("s1");
    let lock = cfg.access_token_lock("corp_a");
    let _guard = lock.lock().await;
}

#[tokio::test]
async fn auth_corp_jsapi_ticket_lock_is_some() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.set_suite_id("s1");
    let lock = cfg.auth_corp_jsapi_ticket_lock("corp_a");
    let _guard = lock.lock().await;
}

#[tokio::test]
async fn suite_jsapi_ticket_lock_is_some() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.set_suite_id("s1");
    let lock = cfg.suite_jsapi_ticket_lock("corp_a");
    let _guard = lock.lock().await;
}

// ---- Debug trait ----

#[test]
fn debug_format() {
    let cfg = WxCpTpDefaultConfig::new();
    let dbg = format!("{cfg:?}");
    assert!(dbg.contains("WxCpTpDefaultConfig"));
}

// ---- access_token_entity for non-existent corp ----

#[test]
fn access_token_entity_empty_for_unknown_corp() {
    let cfg = WxCpTpDefaultConfig::new();
    let entity = cfg.access_token_entity("unknown");
    assert_eq!(entity.access_token, "");
    // expires_in = (0 - now) + 200, which should be negative for any recent timestamp
}

// ---- short expiry (within 200s reserve) ----

#[test]
fn suite_access_token_short_expiry_is_expired() {
    let cfg = WxCpTpDefaultConfig::new();
    // 100 seconds - after 200s reserve, expires_at = now - 100
    cfg.update_suite_access_token("short_lived", 100);
    assert!(cfg.is_suite_access_token_expired());
}

#[test]
fn zero_expiry_provider_token() {
    let cfg = WxCpTpDefaultConfig::new();
    cfg.update_provider_token("zero", 0);
    // No reserve for provider token, expires_at = now + 0 = now → should be expired
    assert!(cfg.is_provider_token_expired());
}
