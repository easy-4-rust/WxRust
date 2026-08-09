//! 对应 Java `cn.binarywang.wx.miniapp.bean.express` 包（生成）。

pub mod request;
pub mod result;
pub mod wx_ma_express_account;
pub mod wx_ma_express_delivery;
pub mod wx_ma_express_path;
pub mod wx_ma_express_printer;

pub use request::wx_ma_express_add_order_request::WxMaExpressAddOrderRequest;
pub use request::wx_ma_express_bind_account_request::WxMaExpressBindAccountRequest;
pub use request::wx_ma_express_delivery_return_add_request::WxMaExpressDeliveryReturnAddRequest;
pub use request::wx_ma_express_get_order_request::WxMaExpressGetOrderRequest;
pub use request::wx_ma_express_order_cargo::WxMaExpressOrderCargo;
pub use request::wx_ma_express_order_cargo_detail::WxMaExpressOrderCargoDetail;
pub use request::wx_ma_express_order_insured::WxMaExpressOrderInsured;
pub use request::wx_ma_express_order_person::WxMaExpressOrderPerson;
pub use request::wx_ma_express_order_shop::WxMaExpressOrderShop;
pub use request::wx_ma_express_order_shop_detail::WxMaExpressOrderShopDetail;
pub use request::wx_ma_express_printer_update_request::WxMaExpressPrinterUpdateRequest;
pub use request::wx_ma_express_return_order::WxMaExpressReturnOrder;
pub use request::wx_ma_express_test_update_order_request::WxMaExpressTestUpdateOrderRequest;
pub use result::wx_ma_express_info_result::WxMaExpressInfoResult;
pub use result::wx_ma_express_order_info_result::WxMaExpressOrderInfoResult;
pub use result::wx_ma_express_return_info_result::WxMaExpressReturnInfoResult;
pub use wx_ma_express_account::WxMaExpressAccount;
pub use wx_ma_express_delivery::ServiceType;
pub use wx_ma_express_delivery::WxMaExpressDelivery;
pub use wx_ma_express_path::PathItem;
pub use wx_ma_express_path::WxMaExpressPath;
pub use wx_ma_express_printer::WxMaExpressPrinter;
