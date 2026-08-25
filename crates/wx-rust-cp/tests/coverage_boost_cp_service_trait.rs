//! Coverage boost: `wx_cp_service.rs` (142 missed, 39.8%).

use std::sync::Arc;

use wx_rust_cp::api::WxCpService;
use wx_rust_cp::api::r#impl::WxCpServiceImpl;
use wx_rust_cp::config::r#impl::WxCpDefaultConfig;

fn make_service() -> Arc<WxCpServiceImpl> {
    let config = WxCpDefaultConfig::new("corp_1", "secret_1");
    WxCpServiceImpl::new_arc(Arc::new(config))
}

#[test]
fn sub_service_getters_all_some() {
    let svc = make_service();
    assert!(svc.department_service().is_some());
    assert!(svc.media_service().is_some());
    assert!(svc.menu_service().is_some());
    assert!(svc.tag_service().is_some());
    assert!(svc.user_service().is_some());
    assert!(svc.external_contact_service().is_some());
    assert!(svc.chat_service().is_some());
    assert!(svc.task_card_service().is_some());
    assert!(svc.message_service().is_some());
    assert!(svc.oa_service().is_some());
    assert!(svc.school_service().is_some());
    assert!(svc.school_user_service().is_some());
    assert!(svc.school_health_service().is_some());
    assert!(svc.living_service().is_some());
    assert!(svc.oa_we_drive_service().is_some());
    assert!(svc.oa_we_doc_service().is_some());
}

#[test]
fn config_storage() {
    let svc = make_service();
    let config = svc.wx_cp_config_storage();
    assert_eq!(config.app_id(), "corp_1");
}

#[test]
fn check_signature_invalid() {
    let svc = make_service();
    assert!(!svc.check_signature("bad_sig", "t1", "n1", "d1"));
}

#[test]
fn build_qr_connect_url() {
    let svc = make_service();
    let url = svc.build_qr_connect_url("https://redirect.example.com", "state_1");
    assert!(!url.is_empty());
}

#[test]
fn init_http_noop() {
    let svc = make_service();
    svc.init_http();
}

#[test]
fn set_retry_and_max() {
    let svc = make_service();
    svc.set_retry_sleep_millis(500);
    assert_eq!(svc.retry_sleep_millis(), 500);
    svc.set_max_retry_times(3);
    assert_eq!(svc.max_retry_times(), 3);
}

#[test]
fn session_manager() {
    let svc = make_service();
    let _sm = svc.session_manager();
}

#[test]
fn http_client() {
    let svc = make_service();
    let _client = svc.http_client();
}

#[test]
fn set_config_storage() {
    let svc = make_service();
    let new_config = WxCpDefaultConfig::new("new_corp", "new_secret");
    svc.set_wx_cp_config_storage(Arc::new(new_config));
    let config = svc.wx_cp_config_storage();
    assert_eq!(config.app_id(), "new_corp");
}

#[tokio::test]
async fn access_token_expired() {
    let svc = make_service();
    let result = svc.get_access_token().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn jsapi_ticket_expired() {
    let svc = make_service();
    let result = svc.get_jsapi_ticket().await;
    assert!(result.is_err());
}
