//! 微信支付错误码常量类。
//!
//! 对应 Java `com.github.binarywang.wxpay.constant.WxPayErrorCode`：Java 静态
//! 内部类（`UnifiedOrder`/`OrderClose`/`Refund`/`RefundQuery`/`DownloadBill`）
//! 逐一对映为同名 Rust 模块，字符串常量一一镜像（值即微信返回的错误码原文）。

/// UnifiedOrder 错误码（对应 Java `WxPayErrorCode.UnifiedOrder`）。
pub mod unified_order {
    /// NOAUTH
    pub const NOAUTH: &str = "NOAUTH";
    /// NOTENOUGH
    pub const NOTENOUGH: &str = "NOTENOUGH";
    /// ORDERPAID
    pub const ORDERPAID: &str = "ORDERPAID";
    /// ORDERCLOSED
    pub const ORDERCLOSED: &str = "ORDERCLOSED";
    /// SYSTEMERROR
    pub const SYSTEMERROR: &str = "SYSTEMERROR";
    /// APPID_NOT_EXIST
    pub const APPID_NOT_EXIST: &str = "APPID_NOT_EXIST";
    /// MCHID_NOT_EXIST
    pub const MCHID_NOT_EXIST: &str = "MCHID_NOT_EXIST";
    /// APPID_MCHID_NOT_MATCH
    pub const APPID_MCHID_NOT_MATCH: &str = "APPID_MCHID_NOT_MATCH";
    /// LACK_PARAMS
    pub const LACK_PARAMS: &str = "LACK_PARAMS";
    /// OUT_TRADE_NO_USED
    pub const OUT_TRADE_NO_USED: &str = "OUT_TRADE_NO_USED";
    /// SIGNERROR
    pub const SIGNERROR: &str = "SIGNERROR";
    /// XML_FORMAT_ERROR
    pub const XML_FORMAT_ERROR: &str = "XML_FORMAT_ERROR";
    /// REQUIRE_POST_METHOD
    pub const REQUIRE_POST_METHOD: &str = "REQUIRE_POST_METHOD";
    /// POST_DATA_EMPTY
    pub const POST_DATA_EMPTY: &str = "POST_DATA_EMPTY";
    /// NOT_UTF8
    pub const NOT_UTF8: &str = "NOT_UTF8";
}

/// OrderClose 错误码（对应 Java `WxPayErrorCode.OrderClose`）。
pub mod order_close {
    /// ORDERPAID
    pub const ORDER_PAID: &str = "ORDERPAID";
    /// SYSTEMERROR
    pub const SYSTEM_ERROR: &str = "SYSTEMERROR";
    /// ORDERNOTEXIST
    pub const ORDER_NOT_EXIST: &str = "ORDERNOTEXIST";
    /// ORDERCLOSED
    pub const ORDER_CLOSED: &str = "ORDERCLOSED";
    /// SIGNERROR
    pub const SIGN_ERROR: &str = "SIGNERROR";
    /// REQUIRE_POST_METHOD
    pub const REQUIRE_POST_METHOD: &str = "REQUIRE_POST_METHOD";
    /// XML_FORMAT_ERROR
    pub const XML_FORMAT_ERROR: &str = "XML_FORMAT_ERROR";
    /// TRADE_STATE_ERROR
    pub const TRADE_STATE_ERROR: &str = "TRADE_STATE_ERROR";
}

/// Refund 错误码（对应 Java `WxPayErrorCode.Refund`）。
pub mod refund {
    /// SYSTEMERROR
    pub const SYSTEMERROR: &str = "SYSTEMERROR";
    /// BIZERR_NEED_RETRY
    pub const BIZERR_NEED_RETRY: &str = "BIZERR_NEED_RETRY";
    /// TRADE_OVERDUE
    pub const TRADE_OVERDUE: &str = "TRADE_OVERDUE";
    /// ERROR
    pub const ERROR: &str = "ERROR";
    /// USER_ACCOUNT_ABNORMAL
    pub const USER_ACCOUNT_ABNORMAL: &str = "USER_ACCOUNT_ABNORMAL";
    /// INVALID_REQ_TOO_MUCH
    pub const INVALID_REQ_TOO_MUCH: &str = "INVALID_REQ_TOO_MUCH";
    /// NOTENOUGH
    pub const NOTENOUGH: &str = "NOTENOUGH";
    /// INVALID_TRANSACTIONID
    pub const INVALID_TRANSACTIONID: &str = "INVALID_TRANSACTIONID";
    /// PARAM_ERROR
    pub const PARAM_ERROR: &str = "PARAM_ERROR";
    /// APPID_NOT_EXIST
    pub const APPID_NOT_EXIST: &str = "APPID_NOT_EXIST";
    /// MCHID_NOT_EXIST
    pub const MCHID_NOT_EXIST: &str = "MCHID_NOT_EXIST";
    /// ORDERNOTEXIST
    pub const ORDERNOTEXIST: &str = "ORDERNOTEXIST";
    /// REQUIRE_POST_METHOD
    pub const REQUIRE_POST_METHOD: &str = "REQUIRE_POST_METHOD";
    /// SIGNERROR
    pub const SIGNERROR: &str = "SIGNERROR";
    /// XML_FORMAT_ERROR
    pub const XML_FORMAT_ERROR: &str = "XML_FORMAT_ERROR";
    /// FREQUENCY_LIMITED
    pub const FREQUENCY_LIMITED: &str = "FREQUENCY_LIMITED";
}

/// RefundQuery 错误码（对应 Java `WxPayErrorCode.RefundQuery`）。
pub mod refund_query {
    /// SYSTEMERROR
    pub const SYSTEMERROR: &str = "SYSTEMERROR";
    /// REFUNDNOTEXIST
    pub const REFUNDNOTEXIST: &str = "REFUNDNOTEXIST";
    /// INVALID_TRANSACTIONID
    pub const INVALID_TRANSACTIONID: &str = "INVALID_TRANSACTIONID";
    /// PARAM_ERROR
    pub const PARAM_ERROR: &str = "PARAM_ERROR";
    /// APPID_NOT_EXIST
    pub const APPID_NOT_EXIST: &str = "APPID_NOT_EXIST";
    /// MCHID_NOT_EXIST
    pub const MCHID_NOT_EXIST: &str = "MCHID_NOT_EXIST";
    /// REQUIRE_POST_METHOD
    pub const REQUIRE_POST_METHOD: &str = "REQUIRE_POST_METHOD";
    /// SIGNERROR
    pub const SIGNERROR: &str = "SIGNERROR";
    /// XML_FORMAT_ERROR
    pub const XML_FORMAT_ERROR: &str = "XML_FORMAT_ERROR";
}

/// DownloadBill 错误码（对应 Java `WxPayErrorCode.DownloadBill`）。
pub mod download_bill {
    /// SYSTEMERROR
    pub const SYSTEMERROR: &str = "SYSTEMERROR";
    /// invalid bill_type
    pub const INVALID_BILL_TYPE: &str = "invalid bill_type";
    /// data format error
    pub const DATA_FORMAT_ERROR: &str = "data format error";
    /// missing parameter
    pub const MISSING_PARAMETER: &str = "missing parameter";
    /// SIGN ERROR
    pub const SIGN_ERROR: &str = "SIGN ERROR";
    /// No Bill Exist
    #[allow(non_upper_case_globals)]
    pub const NO_Bill_Exist: &str = "No Bill Exist";
    /// Bill Creating
    pub const BILL_CREATING: &str = "Bill Creating";
    /// CompressGZip Error
    pub const COMPRESSG_ZIP_ERROR: &str = "CompressGZip Error";
    /// UnCompressGZip Error
    pub const UN_COMPRESSG_ZIP_ERROR: &str = "UnCompressGZip Error";
}
