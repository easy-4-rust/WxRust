//! Coverage boost: `wx_cp_corp_group_default_config_impl.rs` (156 lines, 0%).
//!
//! Exercises all public methods on `WxCpCorpGroupDefaultConfig`:
//! - new/default, setters (corp_id, agent_id, proxy, base_api_url)
//! - corp_access_token lifecycle (update/get/entity/is_expired/expire)
//! - corp_access_token_lock
//! - generate_access_token_key with various agent_id combinations

use wx_rust_cp::config::WxCpCorpGroupConfigStorage;
use wx_rust_cp::config::r#impl::WxCpCorpGroupDefaultConfig;

#[test]
fn new_default_values() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    assert!(cfg.corp_id().is_none());
    assert!(cfg.agent_id().is_none());
    assert!(cfg.base_api_url().is_none());
    assert!(cfg.http_proxy_host().is_none());
    assert_eq!(cfg.http_proxy_port(), 0);
    assert!(cfg.http_proxy_username().is_none());
    assert!(cfg.http_proxy_password().is_none());
}

#[test]
fn default_trait() {
    let cfg = WxCpCorpGroupDefaultConfig::default();
    assert!(cfg.corp_id().is_none());
}

#[test]
fn setters_roundtrip() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    cfg.set_corp_id("corp_1");
    assert_eq!(cfg.corp_id(), Some("corp_1".into()));

    cfg.set_agent_id(Some(1000002));
    assert_eq!(cfg.agent_id(), Some(1000002));

    cfg.set_base_api_url("https://api.example.com");
    assert_eq!(cfg.base_api_url(), Some("https://api.example.com".into()));

    cfg.set_http_proxy_host("proxy.example.com");
    assert_eq!(cfg.http_proxy_host(), Some("proxy.example.com".into()));

    cfg.set_http_proxy_port(8080);
    assert_eq!(cfg.http_proxy_port(), 8080);

    cfg.set_http_proxy_username("user");
    assert_eq!(cfg.http_proxy_username(), Some("user".into()));

    cfg.set_http_proxy_password("pass");
    assert_eq!(cfg.http_proxy_password(), Some("pass".into()));
}

#[test]
fn agent_id_none_setter() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    cfg.set_agent_id(Some(42));
    assert_eq!(cfg.agent_id(), Some(42));
    cfg.set_agent_id(None);
    assert!(cfg.agent_id().is_none());
}

// ---- corp_access_token lifecycle ----

#[test]
fn corp_access_token_initially_none() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    assert!(cfg.corp_access_token("req_corp", Some(1)).is_none());
    assert!(cfg.is_corp_access_token_expired("req_corp", Some(1)));
}

#[test]
fn corp_access_token_update_and_get() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    cfg.set_corp_id("my_corp");
    cfg.set_agent_id(Some(100));
    cfg.update_corp_access_token("req_corp", Some(200), "token_abc", 7200);
    assert_eq!(
        cfg.corp_access_token("req_corp", Some(200)),
        Some("token_abc".into())
    );
    assert!(!cfg.is_corp_access_token_expired("req_corp", Some(200)));
}

#[test]
fn corp_access_token_entity() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    cfg.set_corp_id("c1");
    cfg.update_corp_access_token("r1", Some(1), "tok", 7200);
    let entity = cfg.corp_access_token_entity("r1", Some(1));
    assert_eq!(entity.access_token, "tok");
    assert!(entity.expires_in > 0);
}

#[test]
fn corp_access_token_expire() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    cfg.update_corp_access_token("r1", Some(1), "tok", 7200);
    assert!(!cfg.is_corp_access_token_expired("r1", Some(1)));
    cfg.expire_corp_access_token("r1", Some(1));
    assert!(cfg.is_corp_access_token_expired("r1", Some(1)));
    assert!(cfg.corp_access_token("r1", Some(1)).is_none());
}

#[test]
fn corp_access_token_different_keys() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    cfg.set_corp_id("c");
    cfg.set_agent_id(Some(1));
    cfg.update_corp_access_token("r1", Some(10), "tok_a", 7200);
    cfg.update_corp_access_token("r2", Some(20), "tok_b", 7200);
    assert_eq!(cfg.corp_access_token("r1", Some(10)), Some("tok_a".into()));
    assert_eq!(cfg.corp_access_token("r2", Some(20)), Some("tok_b".into()));
}

#[test]
fn corp_access_token_key_with_none_agents() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    cfg.set_corp_id("c");
    // Both agent_ids None → key includes "null" for both
    cfg.update_corp_access_token("r", None, "tok_none", 7200);
    assert_eq!(cfg.corp_access_token("r", None), Some("tok_none".into()));
}

#[test]
fn corp_access_token_entity_empty_for_unknown() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    let entity = cfg.corp_access_token_entity("unknown", None);
    assert_eq!(entity.access_token, "");
}

#[test]
fn corp_access_token_short_expiry_expired() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    // 100s - after 200s reserve → expires_at in the past
    cfg.update_corp_access_token("r", Some(1), "short", 100);
    assert!(cfg.is_corp_access_token_expired("r", Some(1)));
}

#[tokio::test]
async fn corp_access_token_lock_works() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    cfg.set_corp_id("c");
    let lock = cfg.corp_access_token_lock("r", Some(1));
    let _guard = lock.lock().await;
}

#[test]
fn debug_format() {
    let cfg = WxCpCorpGroupDefaultConfig::new();
    let dbg = format!("{cfg:?}");
    assert!(dbg.contains("WxCpCorpGroupDefaultConfig"));
}
