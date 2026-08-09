//! 企业微信第三方应用（服务商）常量。
//!
//! 对应 Java `me.chanjar.weixin.cp.constant.WxCpTpConsts`（内部类
//! `InfoType`：服务商推送消息的 InfoType 取值）。

/// 服务商推送消息的 InfoType 常量（对应 Java `WxCpTpConsts.InfoType`）。
pub mod info_type {
    /// 推送更新 suite_ticket。
    pub const SUITE_TICKET: &str = "suite_ticket";

    /// 从企业微信应用市场发起授权时，授权成功通知。
    pub const CREATE_AUTH: &str = "create_auth";

    /// 从企业微信应用市场发起授权时，变更授权通知。
    pub const CHANGE_AUTH: &str = "change_auth";

    /// 从企业微信应用市场发起授权时，取消授权通知。
    pub const CANCEL_AUTH: &str = "cancel_auth";

    /// 企业互联共享应用事件回调。
    pub const SHARE_AGENT_CHANGE: &str = "share_agent_change";

    /// 重置永久授权码通知。
    pub const RESET_PERMANENT_CODE: &str = "reset_permanent_code";

    /// 应用管理员变更通知。
    pub const CHANGE_APP_ADMIN: &str = "change_app_admin";

    /// 通讯录变更通知。
    pub const CHANGE_CONTACT: &str = "change_contact";

    /// 用户进行企业微信的注册，注册完成回调通知。
    pub const REGISTER_CORP: &str = "register_corp";

    /// 异步任务回调通知。
    pub const BATCH_JOB_RESULT: &str = "batch_job_result";

    /// 外部联系人变更通知。
    pub const CHANGE_EXTERNAL_CONTACT: &str = "change_external_contact";

    /// 下单成功通知。
    pub const OPEN_ORDER: &str = "open_order";

    /// 改单通知。
    pub const CHANGE_ORDER: &str = "change_order";

    /// 支付成功通知。
    pub const PAY_FOR_APP_SUCCESS: &str = "pay_for_app_success";

    /// 退款通知。
    pub const REFUND: &str = "refund";

    /// 付费版本变更通知。
    pub const CHANGE_EDITION: &str = "change_editon";

    /// 接口许可失效通知。
    pub const UNLICENSED_NOTIFY: &str = "unlicensed_notify";

    /// 支付成功通知。
    pub const LICENSE_PAY_SUCCESS: &str = "license_pay_success";

    /// 退款结果通知。
    pub const LICENSE_REFUND: &str = "license_refund";
}
