//! Coverage boost: `wx_pay_default_config_impl.rs` (130 missed, 59.5%).
//!
//! Exercises all setter/getter pairs on `WxPayDefaultConfig` and the
//! `WxPayConfig` trait implementation.

use wx_rust_pay::config::WxPayConfig;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;

#[test]
fn new_has_defaults() {
    let cfg = WxPayDefaultConfig::new();
    assert_eq!(cfg.api_host_url(), Some("https://api.mch.weixin.qq.com"));
    assert_eq!(cfg.http_connection_timeout(), 5000);
    assert_eq!(cfg.http_timeout(), 10000);
    assert_eq!(cfg.max_conn_total(), 20);
    assert_eq!(cfg.max_conn_per_route(), 10);
    assert_eq!(cfg.cert_auto_update_time(), 60);
    assert!(!cfg.use_sandbox_env());
    assert!(!cfg.if_save_api_data());
    assert!(cfg.strictly_need_wechat_pay_serial());
    assert!(cfg.full_public_key_model());
    // Optional fields are None
    assert!(cfg.app_id().is_none());
    assert!(cfg.mch_id().is_none());
    assert!(cfg.mch_key().is_none());
}

#[test]
fn default_trait() {
    let cfg = WxPayDefaultConfig::default();
    // api_host_url is empty string in default (not the production URL)
    assert_eq!(cfg.api_host_url(), Some(""));
}

#[test]
fn all_setters_and_getters() {
    let mut cfg = WxPayDefaultConfig::new();

    cfg.set_api_host_url("https://custom.api.com");
    assert_eq!(cfg.api_host_url(), Some("https://custom.api.com"));

    cfg.set_api_host_url_path("/v3");
    assert_eq!(cfg.api_host_url_path(), Some("/v3"));

    cfg.set_http_connection_timeout(3000);
    assert_eq!(cfg.http_connection_timeout(), 3000);

    cfg.set_http_timeout(6000);
    assert_eq!(cfg.http_timeout(), 6000);

    cfg.set_app_id("app_1");
    assert_eq!(cfg.app_id(), Some("app_1"));

    cfg.set_sub_app_id("sub_app_1");
    assert_eq!(cfg.sub_app_id(), Some("sub_app_1"));

    cfg.set_mch_id("mch_1");
    assert_eq!(cfg.mch_id(), Some("mch_1"));

    cfg.set_mch_key("key_1");
    assert_eq!(cfg.mch_key(), Some("key_1"));

    cfg.set_ent_pay_key("ent_key_1");
    assert_eq!(cfg.ent_pay_key(), Some("ent_key_1"));

    cfg.set_sub_mch_id("sub_mch_1");
    assert_eq!(cfg.sub_mch_id(), Some("sub_mch_1"));

    cfg.set_notify_url("https://notify.example.com");
    assert_eq!(cfg.notify_url(), Some("https://notify.example.com"));

    cfg.set_refund_notify_url("https://refund.example.com");
    assert_eq!(cfg.refund_notify_url(), Some("https://refund.example.com"));

    cfg.set_trade_type("JSAPI");
    assert_eq!(cfg.trade_type(), Some("JSAPI"));

    cfg.set_sign_type("MD5");
    assert_eq!(cfg.sign_type(), Some("MD5"));

    cfg.set_key_string("base64_p12");
    assert_eq!(cfg.key_string(), Some("base64_p12"));

    cfg.set_key_path("/path/to/cert.p12");
    assert_eq!(cfg.key_path(), Some("/path/to/cert.p12"));

    cfg.set_key_content(vec![1, 2, 3]);
    assert_eq!(cfg.key_content(), Some([1u8, 2, 3].as_slice()));

    cfg.set_private_key_string("base64_key");
    assert_eq!(cfg.private_key_string(), Some("base64_key"));

    cfg.set_private_key_path("/path/to/key.pem");
    assert_eq!(cfg.private_key_path(), Some("/path/to/key.pem"));

    cfg.set_private_key_content(vec![4, 5, 6]);
    assert_eq!(cfg.private_key_content(), Some([4u8, 5, 6].as_slice()));

    cfg.set_private_cert_string("base64_cert");
    assert_eq!(cfg.private_cert_string(), Some("base64_cert"));

    cfg.set_private_cert_path("/path/to/cert.pem");
    assert_eq!(cfg.private_cert_path(), Some("/path/to/cert.pem"));

    cfg.set_private_cert_content(vec![7, 8, 9]);
    assert_eq!(cfg.private_cert_content(), Some([7u8, 8, 9].as_slice()));

    cfg.set_public_key_id("key_id_1");
    assert_eq!(cfg.public_key_id(), Some("key_id_1"));

    cfg.set_public_key_string("base64_pub");
    assert_eq!(cfg.public_key_string(), Some("base64_pub"));

    cfg.set_public_key_path("/path/to/pub.pem");
    assert_eq!(cfg.public_key_path(), Some("/path/to/pub.pem"));

    cfg.set_public_key_content(vec![10, 11, 12]);
    assert_eq!(cfg.public_key_content(), Some([10u8, 11, 12].as_slice()));

    cfg.set_api_v3_key("v3_key");
    assert_eq!(cfg.api_v3_key(), Some("v3_key"));

    cfg.set_cert_serial_no("serial_123");
    assert_eq!(cfg.cert_serial_no(), Some("serial_123"));

    cfg.set_private_key("pem_private_key");
    assert_eq!(cfg.private_key(), Some("pem_private_key"));

    cfg.set_service_id("svc_1");
    assert_eq!(cfg.service_id(), Some("svc_1"));

    cfg.set_pay_score_notify_url("https://score.example.com");
    assert_eq!(cfg.pay_score_notify_url(), Some("https://score.example.com"));

    cfg.set_pay_score_permission_notify_url("https://perm.example.com");
    assert_eq!(cfg.pay_score_permission_notify_url(), Some("https://perm.example.com"));

    cfg.set_max_conn_total(50);
    assert_eq!(cfg.max_conn_total(), 50);

    cfg.set_max_conn_per_route(20);
    assert_eq!(cfg.max_conn_per_route(), 20);

    cfg.set_cert_auto_update_time(30);
    assert_eq!(cfg.cert_auto_update_time(), 30);

    cfg.set_use_sandbox_env(true);
    assert!(cfg.use_sandbox_env());

    cfg.set_if_save_api_data(true);
    assert!(cfg.if_save_api_data());

    cfg.set_http_proxy_host("proxy.example.com");
    assert_eq!(cfg.http_proxy_host(), Some("proxy.example.com"));

    cfg.set_http_proxy_port(8080);
    assert_eq!(cfg.http_proxy_port(), Some(8080));

    cfg.set_http_proxy_username("proxy_user");
    assert_eq!(cfg.http_proxy_username(), Some("proxy_user"));

    cfg.set_http_proxy_password("proxy_pass");
    assert_eq!(cfg.http_proxy_password(), Some("proxy_pass"));

    cfg.set_strictly_need_wechat_pay_serial(false);
    assert!(!cfg.strictly_need_wechat_pay_serial());

    cfg.set_full_public_key_model(false);
    assert!(!cfg.full_public_key_model());
}

#[test]
fn builder_chaining() {
    let mut cfg = WxPayDefaultConfig::new();
    cfg.set_app_id("a")
        .set_mch_id("m")
        .set_mch_key("k")
        .set_api_v3_key("v3")
        .set_cert_serial_no("s")
        .set_private_key("pk")
        .set_notify_url("n");
    assert_eq!(cfg.app_id(), Some("a"));
    assert_eq!(cfg.mch_id(), Some("m"));
    assert_eq!(cfg.mch_key(), Some("k"));
    assert_eq!(cfg.api_v3_key(), Some("v3"));
}

#[test]
fn clone_works() {
    let mut cfg = WxPayDefaultConfig::new();
    cfg.set_app_id("app");
    let cloned = cfg.clone();
    assert_eq!(cloned.app_id(), Some("app"));
}

#[test]
fn debug_format() {
    let cfg = WxPayDefaultConfig::new();
    let dbg = format!("{cfg:?}");
    assert!(dbg.contains("WxPayDefaultConfig"));
}
