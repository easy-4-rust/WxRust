//! 微信支付 v2 接口 URL 常量。
//!
//! 对应 Java 侧散落于 `BaseWxPayServiceImpl` 等实现类中的内联路径字符串
//! （Java 无独立 URL 常量类，v3 下单 URL 见 `TradeTypeEnum`/`GlobalTradeTypeEnum`
//! 的 `*_url()` 方法）。所有路径需拼接 `WxPayService::get_pay_base_url()`
//! 返回的基地址（沙箱环境自动追加 `/xdc/apiv2sandbox`）。

/// 查询订单（对应 Java `BaseWxPayServiceImpl` 中 `"/pay/orderquery"`）。
pub const ORDER_QUERY_URL: &str = "/pay/orderquery";
/// 关闭订单（对应 Java `"/pay/closeorder"`）。
pub const CLOSE_ORDER_URL: &str = "/pay/closeorder";
/// 统一下单（对应 Java `"/pay/unifiedorder"`）。
pub const UNIFIED_ORDER_URL: &str = "/pay/unifiedorder";
/// 申请退款（需证书，对应 Java `"/secapi/pay/refund"`）。
pub const REFUND_URL: &str = "/secapi/pay/refund";
/// 申请退款 v2（支持单品，需证书，对应 Java `"/secapi/pay/refundv2"`）。
pub const REFUND_V2_URL: &str = "/secapi/pay/refundv2";
/// 查询退款（对应 Java `"/pay/refundquery"`）。
pub const REFUND_QUERY_URL: &str = "/pay/refundquery";
/// 查询退款 v2（对应 Java `"/pay/refundqueryv2"`）。
pub const REFUND_QUERY_V2_URL: &str = "/pay/refundqueryv2";
/// 下载对账单（对应 Java `"/pay/downloadbill"`）。
pub const DOWNLOAD_BILL_URL: &str = "/pay/downloadbill";
/// 下载资金账单（对应 Java `"/pay/downloadfundflow"`）。
pub const DOWNLOAD_FUND_FLOW_URL: &str = "/pay/downloadfundflow";
/// 交易保障上报（对应 Java `"/payitil/report"`）。
pub const REPORT_URL: &str = "/payitil/report";
/// 付款码支付/刷卡支付（对应 Java `"/pay/micropay"`）。
pub const MICROPAY_URL: &str = "/pay/micropay";
/// 撤销订单（需证书，对应 Java `"/secapi/pay/reverse"`）。
pub const REVERSE_URL: &str = "/secapi/pay/reverse";
/// 人脸支付（对应 Java `"/pay/facepay"`）。
pub const FACE_PAY_URL: &str = "/pay/facepay";
/// 汇率查询（对应 Java `"/pay/queryexchagerate"`）。
pub const QUERY_EXCHANGE_RATE_URL: &str = "/pay/queryexchagerate";
/// 发放代金券（对应 Java `"/mmpaymkttransfers/send_coupon"`）。
pub const SEND_COUPON_URL: &str = "/mmpaymkttransfers/send_coupon";
/// 查询代金券批次（对应 Java `"/mmpaymkttransfers/query_coupon_stock"`）。
pub const QUERY_COUPON_STOCK_URL: &str = "/mmpaymkttransfers/query_coupon_stock";
/// 查询代金券信息（对应 Java `"/mmpaymkttransfers/querycouponsinfo"`）。
pub const QUERY_COUPON_INFO_URL: &str = "/mmpaymkttransfers/querycouponsinfo";
/// 转换短链接（对应 Java `"/tools/shorturl"`）。
pub const SHORT_URL: &str = "/tools/shorturl";
/// 授权码查询 openid（对应 Java `"/tools/authcodetoopenid"`）。
pub const AUTH_CODE_TO_OPENID_URL: &str = "/tools/authcodetoopenid";
/// 拉取订单评价数据（对应 Java `"/billcommentsp/batchquerycomment"`）。
pub const QUERY_COMMENT_URL: &str = "/billcommentsp/batchquerycomment";
/// 沙箱环境基地址前缀（对应 Java `getPayBaseUrl()` 的 `"/xdc/apiv2sandbox"`）。
pub const SANDBOX_BASE_URL_SUFFIX: &str = "/xdc/apiv2sandbox";
/// 沙箱签名 key 获取（绝对地址，对应 Java `BaseWxPayServiceImpl` 中
/// `"https://api.mch.weixin.qq.com/xdc/apiv2getsignkey/sign/getsignkey"`）。
pub const GET_SANDBOX_SIGN_KEY_URL: &str =
    "https://api.mch.weixin.qq.com/xdc/apiv2getsignkey/sign/getsignkey";
