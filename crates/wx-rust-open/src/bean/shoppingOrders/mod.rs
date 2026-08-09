//! 对应 Java `me.chanjar.weixin.open.bean.shoppingOrders` 包（生成）。

pub mod combined_shipping_info;
pub mod combined_shopping_info;
pub mod contact_bean;
pub mod order_key_bean;
pub mod payer_bean;
pub mod shipping_info;
pub mod shipping_list_bean;
pub mod shopping_info;
pub mod shopping_info_verify_upload;
pub mod wx_open_shopping_info_verify_upload_result;
pub mod wx_open_shopping_orders_confirm_result;

pub use combined_shipping_info::CombinedShippingInfo;
pub use combined_shipping_info::SubOrderListBean;
pub use combined_shopping_info::CombinedShoppingInfo;
pub use contact_bean::ContactBean;
pub use order_key_bean::OrderKeyBean;
pub use payer_bean::PayerBean;
pub use shipping_info::ShippingInfo;
pub use shipping_list_bean::ShippingItemListBean;
pub use shipping_list_bean::ShippingListBean;
pub use shopping_info::OrderDetailBean;
pub use shopping_info::OrderItemListBean;
pub use shopping_info::OrderListBean;
pub use shopping_info::ShoppingInfo;
pub use shopping_info_verify_upload::ShoppingInfoVerifyUpload;
pub use wx_open_shopping_info_verify_upload_result::WxOpenShoppingInfoVerifyUploadResult;
pub use wx_open_shopping_orders_confirm_result::WxOpenShoppingOrdersConfirmResult;
