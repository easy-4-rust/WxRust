//! 微信支付常量类。
//!
//! 对应 Java `com.github.binarywang.wxpay.constant.WxPayConstants`：Java 的
//! 静态内部类（`CurrencyType`/`TradeType`/`SignType`/...）逐一对映为同名的
//! Rust 模块，字符串常量一一镜像。

/// 拉取订单评价数据接口的参数中日期格式（对应 Java
/// `QUERY_COMMENT_DATE_FORMAT`，`FastDateFormat` "yyyyMMddHHmmss"）。
pub const QUERY_COMMENT_DATE_FORMAT: &str = "yyyyMMddHHmmss";

/// 币种类型（对应 Java `WxPayConstants.CurrencyType`）。
pub mod currency_type {
    /// 人民币。
    pub const CNY: &str = "CNY";
}

/// 校验用户姓名选项，企业付款时使用（对应 Java `WxPayConstants.CheckNameOption`）。
pub mod check_name_option {
    /// 不校验真实姓名。
    pub const NO_CHECK: &str = "NO_CHECK";
    /// 强校验真实姓名。
    pub const FORCE_CHECK: &str = "FORCE_CHECK";
}

/// 压缩账单的类型（对应 Java `WxPayConstants.TarType`）。
pub mod tar_type {
    /// 固定值：GZIP，返回格式为 .gzip 的压缩包账单。
    pub const GZIP: &str = "GZIP";
}

/// 账单类型（对应 Java `WxPayConstants.BillType`）。
pub mod bill_type {
    /// 查询红包时使用：通过商户订单号获取红包信息。
    pub const MCHT: &str = "MCHT";
    /// 返回当日所有订单信息，默认值。
    pub const ALL: &str = "ALL";
    /// 返回当日成功支付的订单。
    pub const SUCCESS: &str = "SUCCESS";
    /// 返回当日退款订单。
    pub const REFUND: &str = "REFUND";
    /// 返回当日充值退款订单（相比其他对账单多一栏"返还手续费"）。
    pub const RECHARGE_REFUND: &str = "RECHARGE_REFUND";
}

/// 交易类型（对应 Java `WxPayConstants.TradeType`）。
pub mod trade_type {
    /// 原生扫码支付。
    pub const NATIVE: &str = "NATIVE";
    /// App 支付。
    pub const APP: &str = "APP";
    /// 公众号支付/小程序支付。
    pub const JSAPI: &str = "JSAPI";
    /// H5 支付。
    pub const MWEB: &str = "MWEB";
    /// 刷卡支付（有单独的支付接口，不调用统一下单接口）。
    pub const MICROPAY: &str = "MICROPAY";
}

/// 指定交易方式（对应 Java `WxPayConstants.TradeType.Specific<R>` 抽象类与
/// 静态实例 `NATIVE`/`APP`/`JSAPI`/`MWEB`/`MICROPAY`）。
///
/// `ADAPTED`：Java 以泛型抽象类 + 匿名子类携带结果类型（`R`）与交易类型
/// 字符串；Rust 以枚举表达，交易类型字符串由 `type_str()` 提供，结果类型
/// 由 `WxPayService::create_order_with_specific` 的返回值（Wave 1 起为
/// `bean::order` 具体类型）承接。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WxPaySpecificTradeType {
    /// 原生扫码支付（对应 Java `Specific.NATIVE`）
    Native,
    /// App 支付（对应 Java `Specific.APP`）
    App,
    /// 公众号/小程序支付（对应 Java `Specific.JSAPI`）
    Jsapi,
    /// H5 支付（对应 Java `Specific.MWEB`）
    Mweb,
    /// 刷卡支付（对应 Java `Specific.MICROPAY`）
    Micropay,
}

impl WxPaySpecificTradeType {
    /// 交易类型字符串（对应 Java `Specific.getType()`）。
    pub fn type_str(&self) -> &'static str {
        match self {
            WxPaySpecificTradeType::Native => trade_type::NATIVE,
            WxPaySpecificTradeType::App => trade_type::APP,
            WxPaySpecificTradeType::Jsapi => trade_type::JSAPI,
            WxPaySpecificTradeType::Mweb => trade_type::MWEB,
            WxPaySpecificTradeType::Micropay => trade_type::MICROPAY,
        }
    }
}

/// 签名类型（对应 Java `WxPayConstants.SignType`）。
pub mod sign_type {
    /// HMAC-SHA256 签名。
    pub const HMAC_SHA256: &str = "HMAC-SHA256";
    /// MD5 签名。
    pub const MD5: &str = "MD5";
    /// 全部支持的签名类型列表。
    pub const ALL_SIGN_TYPES: [&str; 2] = [HMAC_SHA256, MD5];
}

/// 限定支付方式（对应 Java `WxPayConstants.LimitPay`）。
pub mod limit_pay {
    /// no_credit——指定不能使用信用卡支付。
    pub const NO_CREDIT: &str = "no_credit";
}

/// 业务结果代码（对应 Java `WxPayConstants.ResultCode`）。
pub mod result_code {
    /// 成功。
    pub const SUCCESS: &str = "SUCCESS";
    /// 失败。
    pub const FAIL: &str = "FAIL";
}

/// 退款资金来源（对应 Java `WxPayConstants.RefundAccountSource`）。
pub mod refund_account_source {
    /// 可用余额退款/基本账户。
    pub const RECHARGE_FUNDS: &str = "REFUND_SOURCE_RECHARGE_FUNDS";
    /// 未结算资金退款。
    pub const UNSETTLED_FUNDS: &str = "REFUND_SOURCE_UNSETTLED_FUNDS";
}

/// 退款渠道（对应 Java `WxPayConstants.RefundChannel`）。
pub mod refund_channel {
    /// 原路退款。
    pub const ORIGINAL: &str = "ORIGINAL";
    /// 退回到余额。
    pub const BALANCE: &str = "BALANCE";
    /// 原账户异常退到其他余额账户。
    pub const OTHER_BALANCE: &str = "OTHER_BALANCE";
    /// 原银行卡异常退到其他银行卡。
    pub const OTHER_BANKCARD: &str = "OTHER_BANKCARD";
}

/// 交易状态（对应 Java `WxPayConstants.WxpayTradeStatus`）。
pub mod wxpay_trade_status {
    /// 支付成功。
    pub const SUCCESS: &str = "SUCCESS";
    /// 支付失败（其他原因，如银行返回失败）。
    pub const PAY_ERROR: &str = "PAYERROR";
    /// 用户支付中。
    pub const USER_PAYING: &str = "USERPAYING";
    /// 已关闭。
    pub const CLOSED: &str = "CLOSED";
    /// 未支付。
    pub const NOTPAY: &str = "NOTPAY";
    /// 转入退款。
    pub const REFUND: &str = "REFUND";
    /// 已撤销（刷卡支付）。
    pub const REVOKED: &str = "REVOKED";
}

/// 退款状态（对应 Java `WxPayConstants.RefundStatus`）。
pub mod refund_status {
    /// 退款成功。
    pub const SUCCESS: &str = "SUCCESS";
    /// v2 退款关闭。
    pub const REFUND_CLOSE: &str = "REFUNDCLOSE";
    /// 退款处理中。
    pub const PROCESSING: &str = "PROCESSING";
    /// v2 退款异常（退款到银行发现用户的卡作废或冻结，需商户平台手动处理）。
    pub const CHANGE: &str = "CHANGE";
    /// v3 退款关闭。
    pub const CLOSED: &str = "CLOSED";
    /// v3 退款异常。
    pub const ABNORMAL: &str = "ABNORMAL";
}

/// 分账接收方类型（对应 Java `WxPayConstants.ReceiverType`）。
pub mod receiver_type {
    /// 商户 id。
    pub const MERCHANT_ID: &str = "MERCHANT_ID";
    /// 个人微信号。
    pub const PERSONAL_WECHATID: &str = "PERSONAL_WECHATID";
    /// 个人 openid。
    pub const PERSONAL_OPENID: &str = "PERSONAL_OPENID";
    /// 个人 sub_openid。
    pub const PERSONAL_SUB_OPENID: &str = "PERSONAL_SUB_OPENID";
}

/// 微信商户转账订单状态（对应 Java `WxPayConstants.TransformBillState`）。
pub mod transform_bill_state {
    /// 转账已受理。
    pub const ACCEPTED: &str = "ACCEPTED";
    /// 转账处理中，转账结果尚未明确。
    pub const PROCESSING: &str = "PROCESSING";
    /// 待收款用户确认，可拉起微信收款确认页面进行收款确认。
    pub const WAIT_USER_CONFIRM: &str = "WAIT_USER_CONFIRM";
    /// 转账结果尚未明确，可拉起微信收款确认页面再次重试确认收款。
    pub const TRANSFERING: &str = "TRANSFERING";
    /// 转账成功。
    pub const SUCCESS: &str = "SUCCESS";
    /// 转账失败。
    pub const FAIL: &str = "FAIL";
    /// 商户撤销请求受理成功，该笔转账正在撤销中。
    pub const CANCELING: &str = "CANCELING";
    /// 转账撤销完成。
    pub const CANCELLED: &str = "CANCELLED";
}

/// 用户授权状态（对应 Java `WxPayConstants.AuthorizationState`）。
pub mod authorization_state {
    /// 未授权。
    pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
    /// 已授权。
    pub const AUTHORIZED: &str = "AUTHORIZED";
}

/// 预约转账批次状态（对应 Java `WxPayConstants.ReservationBatchState`）。
pub mod reservation_batch_state {
    /// 批次已受理。
    pub const ACCEPTED: &str = "ACCEPTED";
    /// 批次处理中。
    pub const PROCESSING: &str = "PROCESSING";
    /// 批次处理完成。
    pub const FINISHED: &str = "FINISHED";
    /// 批次已关闭。
    pub const CLOSED: &str = "CLOSED";
}

/// 预约转账批次关闭原因（对应 Java `WxPayConstants.ReservationBatchCloseReason`）。
pub mod reservation_batch_close_reason {
    /// 商户主动撤销。
    pub const MERCHANT_REVOCATION: &str = "MERCHANT_REVOCATION";
    /// 系统超时关闭。
    pub const OVERDUE_CLOSE: &str = "OVERDUE_CLOSE";
}

/// 【转账场景 ID】该笔转账使用的转账场景（对应 Java `WxPayConstants.TransformSceneId`）。
pub mod transform_scene_id {
    /// 现金营销。
    pub const CASH_MARKETING: &str = "1001";
}

/// 【运营工具转账场景 ID】运营工具专用转账场景
/// （对应 Java `WxPayConstants.OperationSceneId`）。
pub mod operation_scene_id {
    /// 运营工具现金营销。
    pub const OPERATION_CASH_MARKETING: &str = "2001";
    /// 运营工具佣金报酬。
    pub const OPERATION_COMMISSION: &str = "2002";
    /// 运营工具推广奖励。
    pub const OPERATION_PROMOTION: &str = "2003";
}

/// 用户收款感知（对应 Java `WxPayConstants.UserRecvPerception`）。
pub mod user_recv_perception {
    /// 转账场景 现金营销（对应 Java `CASH_MARKETING` 内部类）。
    pub mod cash_marketing {
        /// 默认展示。
        pub const ACTIVITY: &str = "活动奖励";
        /// 需主动传入"现金奖励"才可展示。
        pub const CASH: &str = "现金奖励";
    }
}

/// 收款授权模式（对应 Java `WxPayConstants.ReceiptAuthorizationMode`）。
pub mod receipt_authorization_mode {
    /// 需确认收款授权模式（默认值），用户需要手动确认才能收款。
    pub const CONFIRM_RECEIPT_AUTHORIZATION: &str = "CONFIRM_RECEIPT_AUTHORIZATION";
    /// 免确认收款授权模式，用户授权后收款不需要确认，转账直接到账。
    pub const NO_CONFIRM_RECEIPT_AUTHORIZATION: &str = "NO_CONFIRM_RECEIPT_AUTHORIZATION";
}
