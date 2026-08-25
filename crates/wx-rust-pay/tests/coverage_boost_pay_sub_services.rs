//! Coverage boost: pay sub-service implementations (svc() error path).

use std::sync::{Arc, Weak};

use wx_rust_pay::api::EcommerceService;
use wx_rust_pay::api::ProfitSharingService;
use wx_rust_pay::api::WxPayService;
use wx_rust_pay::api::r#impl::WxPayServiceImpl;
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;

fn dangling_weak() -> Weak<dyn WxPayService> {
    let mut cfg = WxPayDefaultConfig::new();
    cfg.set_app_id("test")
        .set_mch_id("test")
        .set_mch_key("test");
    let arc: Arc<dyn WxPayService> = WxPayServiceImpl::new_arc(Arc::new(cfg));
    let weak = Arc::downgrade(&arc);
    drop(arc);
    weak
}

#[tokio::test]
async fn ecommerce_svc_error_applyment_id() {
    let svc = wx_rust_pay::api::r#impl::ecommerce_service_impl::EcommerceServiceImpl::new(
        dangling_weak(),
    );
    let result = svc.query_apply_status_by_applyment_id("123").await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("WxPayService"));
}

#[tokio::test]
async fn ecommerce_svc_error_out_request_no() {
    let svc = wx_rust_pay::api::r#impl::ecommerce_service_impl::EcommerceServiceImpl::new(
        dangling_weak(),
    );
    let result = svc.query_apply_status_by_out_request_no("req_123").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn profit_sharing_svc_error() {
    let svc = wx_rust_pay::api::r#impl::profit_sharing_service_impl::ProfitSharingServiceImpl::new(
        dangling_weak(),
    );
    use wx_rust_pay::bean::profitsharing::request::profit_sharing_merchant_ratio_query_request::ProfitSharingMerchantRatioQueryRequest;
    let req = ProfitSharingMerchantRatioQueryRequest::default();
    let result = svc.profit_sharing_merchant_ratio_query(&req).await;
    assert!(result.is_err());
}
