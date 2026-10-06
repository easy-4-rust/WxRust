use std::sync::Arc;
use wx_rust_store::api::{WxStoreService, r#impl::WxStoreServiceImpl};
use wx_rust_store::config::r#impl::WxStoreDefaultConfig;
#[test]
fn all_business_services_are_assembled() {
    let service = WxStoreServiceImpl::new_arc(Arc::new(WxStoreDefaultConfig::new("app", "secret")));
    assert!(service.address_service().is_some());
    assert!(service.after_sale_service().is_some());
    assert!(service.basic_service().is_some());
    assert!(service.brand_service().is_some());
    assert!(service.category_service().is_some());
    assert!(service.compass_shop_service().is_some());
    assert!(service.cooperation_service().is_some());
    assert!(service.coupon_service().is_some());
    assert!(service.ewaybill_service().is_some());
    assert!(service.favorite_service().is_some());
    assert!(service.freight_template_service().is_some());
    assert!(service.fund_service().is_some());
    assert!(service.gift_service().is_some());
    assert!(service.home_page_service().is_some());
    assert!(service.kf_service().is_some());
    assert!(service.limited_discount_service().is_some());
    assert!(service.order_service().is_some());
    assert!(service.product_assistant_service().is_some());
    assert!(service.product_service().is_some());
    assert!(service.product_stock_service().is_some());
    assert!(service.qic_service().is_some());
    assert!(service.sharer_service().is_some());
    assert!(service.supplier_service().is_some());
    assert!(service.vip_service().is_some());
    assert!(service.warehouse_service().is_some());
    assert!(service.talent_service().is_some());
}
