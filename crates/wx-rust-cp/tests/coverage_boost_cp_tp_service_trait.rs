//! Coverage boost: `wx_cp_tp_service.rs` (261 lines, 0%).
//!
//! Exercises default method implementations on `WxCpTpService` trait.

use std::sync::Arc;

use wx_rust_cp::config::WxCpTpConfigStorage;
use wx_rust_cp::config::r#impl::WxCpTpDefaultConfig;
use wx_rust_cp::tp::service::WxCpTpService;
use wx_rust_cp::tp::service::r#impl::WxCpTpServiceImpl;

fn make_service() -> Arc<WxCpTpServiceImpl> {
    let config = WxCpTpDefaultConfig::new();
    config.set_corp_id("corp_1");
    config.set_suite_id("suite_1");
    config.set_suite_secret("secret_1");
    config.set_provider_secret("prov_sec");
    config.set_token("test_token");
    WxCpTpServiceImpl::new_arc(Arc::new(config))
}

// ---- check_signature ----

#[test]
fn check_signature_valid() {
    let svc = make_service();
    let sig =
        wx_rust_common::util::crypto::Sha1::digest(&["test_token", "t1", "n1", "d1"]).unwrap();
    assert!(svc.check_signature(&sig, "t1", "n1", "d1"));
}

#[test]
fn check_signature_invalid() {
    let svc = make_service();
    assert!(!svc.check_signature("bad_sig", "t1", "n1", "d1"));
}

// ---- get_suite_access_token ----

#[tokio::test]
async fn get_suite_access_token_not_implemented() {
    let svc = make_service();
    let result = svc.get_suite_access_token_with_force(false).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn get_suite_access_token_entity_not_implemented() {
    let svc = make_service();
    let result = svc.get_suite_access_token_entity_with_force(false).await;
    assert!(result.is_err());
}

// ---- get_suite_ticket ----

#[tokio::test]
async fn get_suite_ticket_expired() {
    let svc = make_service();
    let result = svc.get_suite_ticket().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn get_suite_ticket_valid() {
    let svc = make_service();
    let config = svc.wx_cp_tp_config_storage();
    config.update_suite_ticket("ticket_abc", 7200);
    let result = svc.get_suite_ticket().await;
    assert_eq!(result.unwrap(), "ticket_abc");
}

#[tokio::test]
async fn get_suite_ticket_force_expired() {
    let svc = make_service();
    let result = svc.get_suite_ticket_with_force(true).await;
    assert!(result.is_err());
}

// ---- set_suite_ticket ----

#[test]
fn set_suite_ticket_direct() {
    let svc = make_service();
    svc.set_suite_ticket("new_ticket");
    let config = svc.wx_cp_tp_config_storage();
    assert_eq!(config.suite_ticket(), Some("new_ticket".into()));
}

#[test]
fn set_suite_ticket_with_expires() {
    let svc = make_service();
    svc.set_suite_ticket_with_expires("ticket_with_exp", 7200);
    let config = svc.wx_cp_tp_config_storage();
    assert_eq!(config.suite_ticket(), Some("ticket_with_exp".into()));
}

// ---- jsapi_ticket ----

#[tokio::test]
async fn get_auth_corp_js_api_ticket_expired() {
    let svc = make_service();
    let result = svc.get_auth_corp_js_api_ticket("auth_corp_1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn get_auth_corp_js_api_ticket_valid() {
    let svc = make_service();
    let config = svc.wx_cp_tp_config_storage();
    config.update_auth_corp_js_api_ticket("auth_corp_1", "js_ticket_1", 7200);
    let result = svc.get_auth_corp_js_api_ticket("auth_corp_1").await;
    assert_eq!(result.unwrap(), "js_ticket_1");
}

#[tokio::test]
async fn get_suite_js_api_ticket_expired() {
    let svc = make_service();
    let result = svc.get_suite_js_api_ticket("auth_corp_1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn get_suite_js_api_ticket_valid() {
    let svc = make_service();
    let config = svc.wx_cp_tp_config_storage();
    config.update_auth_suite_js_api_ticket("auth_corp_1", "suite_ticket_1", 7200);
    let result = svc.get_suite_js_api_ticket("auth_corp_1").await;
    assert_eq!(result.unwrap(), "suite_ticket_1");
}

// ---- get / post default ----

#[tokio::test]
async fn get_default_not_implemented() {
    let svc = make_service();
    let result = svc.get("https://example.com", "").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn post_default_not_implemented() {
    let svc = make_service();
    let result = svc.post("https://example.com", "body").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn get_without_suite_token_default() {
    let svc = make_service();
    let result = svc
        .get_without_suite_token("https://example.com", "", true)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn post_without_suite_token_default() {
    let svc = make_service();
    let result = svc
        .post_without_suite_token("https://example.com", "body", true)
        .await;
    assert!(result.is_err());
}

// ---- get_permanent_code ----

#[tokio::test]
async fn get_permanent_code_not_implemented() {
    let svc = make_service();
    let result = svc.get_permanent_code("auth_code").await;
    assert!(result.is_err());
}

// ---- get_corp_token ----

#[tokio::test]
async fn get_corp_token_expired() {
    let svc = make_service();
    let result = svc.get_corp_token("auth_corp_1", "permanent_code").await;
    assert!(result.is_err());
}

// ---- get_user_info_3rd ----

#[tokio::test]
async fn get_user_info_3rd_not_implemented() {
    let svc = make_service();
    let result = svc.get_user_info_3rd("code").await;
    assert!(result.is_err());
}

// ---- get_user_detail_3rd ----

#[tokio::test]
async fn get_user_detail_3rd_not_implemented() {
    let svc = make_service();
    let result = svc.get_user_detail_3rd("user_ticket").await;
    assert!(result.is_err());
}

// ---- get_login_info ----

#[tokio::test]
async fn get_login_info_not_implemented() {
    let svc = make_service();
    let result = svc.get_login_info("auth_code").await;
    assert!(result.is_err());
}

// ---- get_customized_auth_url ----

#[tokio::test]
async fn get_customized_auth_url() {
    let svc = make_service();
    let result = svc.get_customized_auth_url("state_1", &[]).await;
    assert!(result.is_err()); // not implemented in default
}

// ---- sub_service getters ----

#[test]
fn sub_service_getters_all_some() {
    let svc = make_service();
    assert!(svc.wx_cp_tp_contact_service().is_some());
    assert!(svc.wx_cp_tp_department_service().is_some());
    assert!(svc.wx_cp_tp_media_service().is_some());
    assert!(svc.wx_cp_tp_oa_service().is_some());
    assert!(svc.wx_cp_tp_user_service().is_some());
    assert!(svc.wx_cp_tp_order_service().is_some());
    assert!(svc.wx_cp_tp_edition_service().is_some());
    assert!(svc.wx_cp_tp_license_service().is_some());
    assert!(svc.wx_cp_tp_id_convert_service().is_some());
    assert!(svc.wx_cp_tp_o_auth2_service().is_some());
    assert!(svc.wx_cp_tp_customized_service().is_some());
    assert!(svc.wx_cp_tp_message_service().is_some());
    assert!(svc.wx_cp_tp_tag_service().is_some());
}

// ---- config storage ----

#[test]
fn config_storage_roundtrip() {
    let svc = make_service();
    let config = svc.wx_cp_tp_config_storage();
    assert_eq!(config.corp_id(), "corp_1");
    assert_eq!(config.suite_id(), "suite_1");
}

// ---- init_http ----

#[test]
fn init_http_noop() {
    let svc = make_service();
    svc.init_http();
}

// ---- set_retry / max ----

#[test]
fn set_retry_and_max() {
    let svc = make_service();
    svc.set_retry_sleep_millis(500);
    assert_eq!(svc.retry_sleep_millis(), 500);
    svc.set_max_retry_times(3);
    assert_eq!(svc.max_retry_times(), 3);
}

// ---- expire methods ----

#[test]
fn expire_suite_access_token() {
    let svc = make_service();
    let config = svc.wx_cp_tp_config_storage();
    config.update_suite_access_token("token", 7200);
    assert!(!config.is_suite_access_token_expired());
    svc.expire_suite_access_token();
    assert!(config.is_suite_access_token_expired());
}

#[test]
fn expire_access_token() {
    let svc = make_service();
    let config = svc.wx_cp_tp_config_storage();
    config.update_access_token("auth_corp_1", "at", 7200);
    assert!(!config.is_access_token_expired("auth_corp_1"));
    svc.expire_access_token("auth_corp_1");
    assert!(config.is_access_token_expired("auth_corp_1"));
}

#[test]
fn expire_auth_corp_js_api_ticket() {
    let svc = make_service();
    let config = svc.wx_cp_tp_config_storage();
    config.update_auth_corp_js_api_ticket("auth_corp_1", "ticket", 7200);
    assert!(!config.is_auth_corp_js_api_ticket_expired("auth_corp_1"));
    svc.expire_auth_corp_js_api_ticket("auth_corp_1");
    assert!(config.is_auth_corp_js_api_ticket_expired("auth_corp_1"));
}

#[test]
fn expire_auth_suite_js_api_ticket() {
    let svc = make_service();
    let config = svc.wx_cp_tp_config_storage();
    config.update_auth_suite_js_api_ticket("auth_corp_1", "ticket", 7200);
    assert!(!config.is_auth_suite_js_api_ticket_expired("auth_corp_1"));
    svc.expire_auth_suite_js_api_ticket("auth_corp_1");
    assert!(config.is_auth_suite_js_api_ticket_expired("auth_corp_1"));
}

#[test]
fn expire_provider_token() {
    let svc = make_service();
    let config = svc.wx_cp_tp_config_storage();
    config.update_provider_token("pt", 7200);
    assert!(!config.is_provider_token_expired());
    svc.expire_provider_token();
    assert!(config.is_provider_token_expired());
}

// ---- get_verify_decrypt ----

#[test]
fn get_verify_decrypt_invalid() {
    let svc = make_service();
    let result = svc.get_verify_decrypt("bad_echo");
    assert!(result.is_err());
}

// ---- get_wx_cp_provider_token ----

#[tokio::test]
async fn get_wx_cp_provider_token_not_implemented() {
    let svc = make_service();
    let result = svc.get_wx_cp_provider_token().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn get_wx_cp_provider_token_entity_not_implemented() {
    let svc = make_service();
    let result = svc.get_wx_cp_provider_token_entity().await;
    assert!(result.is_err());
}

// ---- do_create_wx_jsapi_signature ----

#[test]
fn do_create_wx_jsapi_signature() {
    let svc = make_service();
    let sig = svc.do_create_wx_jsapi_signature("https://example.com", "nonce", "ticket_1");
    assert!(!sig.url.is_empty());
    assert!(!sig.signature.is_empty());
}

// ---- from_encrypted_xml ----

#[test]
fn from_encrypted_xml_invalid() {
    let svc = make_service();
    let config = svc.wx_cp_tp_config_storage();
    let result = wx_rust_cp::bean::message::WxCpTpXmlMessage::from_encrypted_xml(
        "bad_xml",
        config.as_ref(),
        "t",
        "n",
        "s",
    );
    assert!(result.is_err());
}
