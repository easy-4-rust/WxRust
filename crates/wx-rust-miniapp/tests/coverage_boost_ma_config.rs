//! Coverage boost: `wx_ma_default_config_impl.rs` (67 missed, 52% covered).

use wx_rust_common::config::WxConfigStorage;
use wx_rust_common::enums::TicketType;
use wx_rust_miniapp::config::WxMaConfig;
use wx_rust_miniapp::config::r#impl::WxMaDefaultConfig;

#[test]
fn new_with_app_id_and_secret() {
    let cfg = WxMaDefaultConfig::new("app_1", "secret_1");
    assert_eq!(cfg.token(), None);
    assert_eq!(cfg.aes_key(), None);
    assert_eq!(cfg.original_id(), None);
    assert_eq!(cfg.cloud_env(), None);
    assert_eq!(cfg.msg_data_format(), None);
    assert_eq!(cfg.retry_sleep_millis(), 1000);
    assert_eq!(cfg.max_retry_times(), 5);
}

#[test]
fn setters_roundtrip() {
    let mut cfg = WxMaDefaultConfig::new("app", "sec");
    cfg.set_token("tok");
    assert_eq!(cfg.token(), Some("tok"));
    cfg.set_aes_key("aes");
    assert_eq!(cfg.aes_key(), Some("aes"));
    cfg.set_original_id("orig");
    assert_eq!(cfg.original_id(), Some("orig"));
    cfg.set_cloud_env("env");
    assert_eq!(cfg.cloud_env(), Some("env"));
    cfg.set_msg_data_format("JSON");
    assert_eq!(cfg.msg_data_format(), Some("JSON"));
    cfg.set_retry_sleep_millis(500);
    assert_eq!(cfg.retry_sleep_millis(), 500);
    cfg.set_max_retry_times(3);
    assert_eq!(cfg.max_retry_times(), 3);
}

#[test]
fn api_signature_setters() {
    let mut cfg = WxMaDefaultConfig::new("app", "sec");
    cfg.set_api_signature_rsa_private_key("rsa_key");
    assert_eq!(cfg.api_signature_rsa_private_key(), Some("rsa_key".into()));
    cfg.set_api_signature_aes_key("aes_key");
    assert_eq!(cfg.api_signature_aes_key(), Some("aes_key".into()));
    cfg.set_api_signature_rsa_private_key_sn("rsa_sn");
    assert_eq!(cfg.api_signature_rsa_private_key_sn(), Some("rsa_sn".into()));
    cfg.set_api_signature_aes_key_sn("aes_sn");
    assert_eq!(cfg.api_signature_aes_key_sn(), Some("aes_sn".into()));
    cfg.set_wechat_mp_appid("wx_mp");
    assert_eq!(cfg.wechat_mp_appid(), Some("wx_mp".into()));
}

#[test]
fn access_token_lifecycle() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    assert!(cfg.access_token().is_none());
    assert!(cfg.is_access_token_expired());
    cfg.update_access_token("new_token", 7200);
    assert_eq!(cfg.access_token(), Some("new_token".into()));
    assert!(!cfg.is_access_token_expired());
    cfg.expire_access_token();
    assert!(cfg.is_access_token_expired());
    assert!(cfg.access_token().is_none());
}

#[test]
fn ticket_lifecycle() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    assert!(cfg.ticket(TicketType::Jsapi).is_none());
    assert!(cfg.is_ticket_expired(TicketType::Jsapi));
    cfg.update_ticket(TicketType::Jsapi, "ticket_1", 7200);
    assert_eq!(cfg.ticket(TicketType::Jsapi), Some("ticket_1".into()));
    assert!(!cfg.is_ticket_expired(TicketType::Jsapi));
    cfg.expire_ticket(TicketType::Jsapi);
    assert!(cfg.is_ticket_expired(TicketType::Jsapi));
}

#[test]
fn card_ticket_lifecycle() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    assert!(cfg.ticket(TicketType::WxCard).is_none());
    cfg.update_ticket(TicketType::WxCard, "card_ticket_1", 7200);
    assert_eq!(cfg.ticket(TicketType::WxCard), Some("card_ticket_1".into()));
    cfg.expire_ticket(TicketType::WxCard);
}

#[test]
fn sdk_ticket_lifecycle() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    assert!(cfg.ticket(TicketType::Sdk).is_none());
    cfg.update_ticket(TicketType::Sdk, "sdk_key", 7200);
    assert_eq!(cfg.ticket(TicketType::Sdk), Some("sdk_key".into()));
    cfg.expire_ticket(TicketType::Sdk);
}

#[test]
fn use_stable_access_token() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    cfg.use_stable_access_token(true);
    cfg.use_stable_access_token(false);
}

#[test]
fn zero_expiry_is_expired() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    cfg.update_access_token("tok", 0);
    assert!(cfg.is_access_token_expired());
}

#[test]
fn builder_chaining() {
    let mut cfg = WxMaDefaultConfig::new("app", "sec");
    cfg.set_token("t").set_aes_key("a").set_original_id("o");
    assert_eq!(cfg.token(), Some("t"));
    assert_eq!(cfg.aes_key(), Some("a"));
    assert_eq!(cfg.original_id(), Some("o"));
}

#[test]
fn debug_format() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    let dbg = format!("{cfg:?}");
    assert!(dbg.contains("WxMaDefaultConfig"));
}

#[test]
fn host_config() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    let host = cfg.host_config();
    let _ = host;
}

#[test]
fn access_token_url() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    let _ = cfg.access_token_url();
}

#[test]
fn is_stable_access_token_and_auto_refresh() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    let _ = cfg.is_stable_access_token();
    let _ = cfg.auto_refresh_token();
}

#[tokio::test]
async fn access_token_lock() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    let lock = cfg.access_token_lock();
    let _guard = lock.lock().await;
}

#[tokio::test]
async fn ticket_lock() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    let lock = cfg.ticket_lock(TicketType::Jsapi);
    let _guard = lock.lock().await;
}

#[test]
fn multiple_ticket_types() {
    let cfg = WxMaDefaultConfig::new("app", "sec");
    cfg.update_ticket(TicketType::Jsapi, "js", 7200);
    cfg.update_ticket(TicketType::WxCard, "cd", 7200);
    cfg.update_ticket(TicketType::Sdk, "sd", 7200);
    assert_eq!(cfg.ticket(TicketType::Jsapi), Some("js".into()));
    assert_eq!(cfg.ticket(TicketType::WxCard), Some("cd".into()));
    assert_eq!(cfg.ticket(TicketType::Sdk), Some("sd".into()));
}
