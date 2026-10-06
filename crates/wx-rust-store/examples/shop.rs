use std::sync::Arc;
use wx_rust_store::api::{WxStoreService, r#impl::WxStoreServiceImpl};
use wx_rust_store::config::r#impl::WxStoreDefaultConfig;

fn main() {
    let config = Arc::new(WxStoreDefaultConfig::new("demo_appid", "demo_secret"));
    let service = WxStoreServiceImpl::new_arc(config);
    assert!(service.product_service().is_some());
    assert!(service.order_service().is_some());
    assert!(service.after_sale_service().is_some());
    assert!(service.fund_service().is_some());
    // 实际业务在异步上下文通过子服务调用；构建客户端不会访问网络。
}
