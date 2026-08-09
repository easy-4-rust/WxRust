//! 对应 Java `com.github.binarywang.wxpay.bean.mipay` 包（生成）。

pub mod enums;
pub mod med_ins_orders_request;
pub mod med_ins_orders_result;
pub mod med_ins_refund_notify_request;

pub use med_ins_orders_request::CashAddEntity;
pub use med_ins_orders_request::CashReduceEntity;
pub use med_ins_orders_request::MedInsOrdersRequest;
pub use med_ins_orders_request::PersonIdentification;
pub use med_ins_orders_result::MedInsOrdersResult;
pub use med_ins_refund_notify_request::MedInsRefundNotifyRequest;
