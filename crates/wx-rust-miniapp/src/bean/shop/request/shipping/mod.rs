//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop/request/shipping` 包（生成）。

pub mod contact_bean;
pub mod order_key_bean;
pub mod payer_bean;
pub mod shipping_list_bean;
pub mod wx_ma_order_combined_shipping_info_upload_request;
pub mod wx_ma_order_shipping_info_get_list_request;
pub mod wx_ma_order_shipping_info_get_request;
pub mod wx_ma_order_shipping_info_notify_confirm_request;
pub mod wx_ma_order_shipping_info_upload_request;

pub use contact_bean::ContactBean;
pub use order_key_bean::OrderKeyBean;
pub use payer_bean::PayerBean;
pub use shipping_list_bean::ShippingListBean;
pub use wx_ma_order_combined_shipping_info_upload_request::SubOrderBean;
pub use wx_ma_order_combined_shipping_info_upload_request::WxMaOrderCombinedShippingInfoUploadRequest;
pub use wx_ma_order_shipping_info_get_list_request::PayTimeRange;
pub use wx_ma_order_shipping_info_get_list_request::WxMaOrderShippingInfoGetListRequest;
pub use wx_ma_order_shipping_info_get_request::WxMaOrderShippingInfoGetRequest;
pub use wx_ma_order_shipping_info_notify_confirm_request::WxMaOrderShippingInfoNotifyConfirmRequest;
pub use wx_ma_order_shipping_info_upload_request::WxMaOrderShippingInfoUploadRequest;
