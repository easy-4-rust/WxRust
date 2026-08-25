//! Coverage boost: `wx_cp_tp_service_impl.rs` (104 lines, 0%).
//!
//! Exercises `WxCpTpServiceImpl::new_arc` and all trait method implementations.

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
    WxCpTpServiceImpl::new_arc(Arc::new(config))
}

#[test]
fn new_arc_creates_service() {
    let svc = make_service();
    let config = svc.wx_cp_tp_config_storage();
    assert_eq!(config.corp_id(), "corp_1");
}

#[test]
fn set_config_storage() {
    let svc = make_service();
    let new_config = WxCpTpDefaultConfig::new();
    new_config.set_corp_id("new_corp");
    svc.set_wx_cp_tp_config_storage(Arc::new(new_config));
    let config = svc.wx_cp_tp_config_storage();
    assert_eq!(config.corp_id(), "new_corp");
}

#[test]
fn http_client() {
    let svc = make_service();
    let _client = svc.http_client();
}

#[test]
fn retry_params() {
    let svc = make_service();
    assert_eq!(svc.retry_sleep_millis(), 1000);
    assert_eq!(svc.max_retry_times(), 5);
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
fn sub_service_getters() {
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
