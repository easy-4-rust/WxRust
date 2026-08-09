//! 对应 Java `com.github.binarywang.wxpay.bean.notify` 包（生成）。

pub mod combine_notify_result;
pub mod complaint_notify_result;
pub mod merchant_violation_notify_result;
pub mod mi_pay_notify_v3_result;
pub mod origin_notify_response;
pub mod partner_subscribe_notify_result;
pub mod signature_header;
pub mod wx_pay_notify_response;
pub mod wx_pay_notify_v3_response;
pub mod wx_pay_notify_v3_result;
pub mod wx_pay_order_notify_coupon;
pub mod wx_pay_order_notify_result;
pub mod wx_pay_partner_notify_v3_result;
pub mod wx_pay_partner_refund_notify_v3_result;
pub mod wx_pay_refund_notify_result;
pub mod wx_pay_refund_notify_v3_result;
pub mod wx_pay_transfer_batches_notify_v3_result;
pub mod wx_scan_pay_notify_result;

pub use combine_notify_result::Amount;
pub use combine_notify_result::CombineNotifyResult;
pub use combine_notify_result::CombinePayerInfo;
pub use combine_notify_result::DecryptNotifyResult;
pub use combine_notify_result::GoodsDetail;
pub use combine_notify_result::PromotionDetail;
pub use combine_notify_result::SceneInfo;
pub use combine_notify_result::SubOrders;
pub use complaint_notify_result::ComplaintNotifyResult;
pub use merchant_violation_notify_result::MerchantViolationNotifyResult;
pub use mi_pay_notify_v3_result::MiPayNotifyV3Result;
pub use origin_notify_response::OriginNotifyResponse;
pub use origin_notify_response::Resource;
pub use partner_subscribe_notify_result::MessageContent;
pub use partner_subscribe_notify_result::PartnerSubscribeNotifyResult;
pub use partner_subscribe_notify_result::TopicName;
pub use signature_header::SignatureHeader;
pub use wx_pay_notify_response::WxPayNotifyResponse;
pub use wx_pay_notify_v3_response::WxPayNotifyV3Response;
pub use wx_pay_notify_v3_result::Payer;
pub use wx_pay_notify_v3_result::WxPayNotifyV3Result;
pub use wx_pay_order_notify_coupon::WxPayOrderNotifyCoupon;
pub use wx_pay_order_notify_result::WxPayOrderNotifyResult;
pub use wx_pay_partner_notify_v3_result::WxPayPartnerNotifyV3Result;
pub use wx_pay_partner_refund_notify_v3_result::WxPayPartnerRefundNotifyV3Result;
pub use wx_pay_refund_notify_result::ReqInfo;
pub use wx_pay_refund_notify_result::WxPayRefundNotifyResult;
pub use wx_pay_refund_notify_v3_result::FromItem;
pub use wx_pay_refund_notify_v3_result::WxPayRefundNotifyV3Result;
pub use wx_pay_transfer_batches_notify_v3_result::WxPayTransferBatchesNotifyV3Result;
pub use wx_scan_pay_notify_result::WxScanPayNotifyResult;

// v3 通知解密（AEAD_AES_256_GCM，`OriginNotifyResponse.resource.ciphertext`）
// 与 v2 退款通知 req_info 解密（AES-256-ECB）在 **Wave 2** 实现：
// 相关 bean 已提供 `from_xml`/`from_json` 结构解析，解密 + 验签在服务层接线。
