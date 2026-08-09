//! 对应 Java `me.chanjar.weixin.channel.bean.delivery` 包（生成）。

pub mod delivery_company_info;
pub mod delivery_company_response;
pub mod delivery_info;
pub mod delivery_send_param;
pub mod freight_product_info;
pub mod fresh_inspect_param;
pub mod package_audit_info;

pub use delivery_company_info::DeliveryCompanyInfo;
pub use delivery_company_response::DeliveryCompanyResponse;
pub use delivery_info::DeliveryInfo;
pub use delivery_send_param::DeliverySendParam;
pub use freight_product_info::FreightProductInfo;
pub use fresh_inspect_param::FreshInspectParam;
pub use package_audit_info::PackageAuditInfo;
