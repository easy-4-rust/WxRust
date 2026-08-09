//! 微信公众号事件常量。
//!
//! 对应 Java `me.chanjar.weixin.mp.constant.WxMpEventConstants`。

/// 门店审核事件推送。
pub const POI_CHECK_NOTIFY: &str = "poi_check_notify";
/// 会员卡内容更新事件。
pub const SUBMIT_MEMBERCARD_USER_INFO: &str = "submit_membercard_user_info";
/// 摇一摇事件。
pub const SHAKEAROUND_USER_SHAKE: &str = "ShakearoundUserShake";
/// 卡券审核通过事件。
pub const CARD_PASS_CHECK: &str = "card_pass_check";
/// 卡券审核未通过事件。
pub const CARD_NOT_PASS_CHECK: &str = "card_not_pass_check";
/// 领取卡券事件。
pub const USER_GET_CARD: &str = "user_get_card";
/// 删除卡券事件。
pub const USER_DEL_CARD: &str = "user_del_card";
/// 核销卡券事件。
pub const USER_CONSUME_CARD: &str = "user_consume_card";
/// 买单事件。
pub const USER_PAY_FROM_PAY_CELL: &str = "user_pay_from_pay_cell";
/// 进入会员卡事件。
pub const USER_VIEW_CARD: &str = "user_view_card";
/// 从卡券进入公众号会话事件。
pub const USER_ENTER_SESSION_FROM_CARD: &str = "user_enter_session_from_card";
/// 卡券转赠事件。
pub const USER_GIFTING_CARD: &str = "user_gifting_card";
/// 库存报警事件。
pub const CARD_SKU_REMIND: &str = "card_sku_remind";
/// 会员卡内容更新事件。
pub const UPDATE_MEMBER_CARD: &str = "update_member_card";
/// 卡券买单事件。
pub const CARD_PAY_ORDER: &str = "card_pay_order";
/// 券点充值成功事件。
pub const GIFTCARD_PAY_DONE: &str = "giftcard_pay_done";
/// 券点转赠事件。
pub const GIFTCARD_SEND_TO_FRIEND: &str = "giftcard_send_to_friend";
/// 券点转赠接受事件。
pub const GIFTCARD_USER_ACCEPT: &str = "giftcard_user_accept";
/// 客服会话创建事件。
pub const KF_CREATE_SESSION: &str = "kf_create_session";
/// 客服会话关闭事件。
pub const KF_CLOSE_SESSION: &str = "kf_close_session";
/// 客服会话转接事件。
pub const KF_SWITCH_SESSION: &str = "kf_switch_session";
/// 资质认证成功事件。
pub const QUALIFICATION_VERIFY_SUCCESS: &str = "qualification_verify_success";
/// 资质认证失败事件。
pub const QUALIFICATION_VERIFY_FAIL: &str = "qualification_verify_fail";
/// 名称认证成功事件。
pub const NAMING_VERIFY_SUCCESS: &str = "naming_verify_success";
/// 名称认证失败事件。
pub const NAMING_VERIFY_FAIL: &str = "naming_verify_fail";
/// 年审通知事件。
pub const ANNUAL_RENEW: &str = "annual_renew";
/// 认证过期失效通知事件。
pub const VERIFY_EXPIRED: &str = "verify_expired";
/// 用户授权发票事件。
pub const USER_AUTHORIZE_INVOICE: &str = "user_authorize_invoice";
/// 云发票开票结果事件。
pub const CLOUD_INVOICE_INVOICERESULT_EVENT: &str = "cloud_invoice_invoiceresult_event";
/// 顾问邀请结果事件。
pub const GUIDE_INVITE_RESULT_EVENT: &str = "guide_invite_result_event";
