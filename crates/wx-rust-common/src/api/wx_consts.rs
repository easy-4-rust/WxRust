//! 微信常量集合。
//!
//! 对应 Java `me.chanjar.weixin.common.api.WxConsts`，
//! 由 `scripts/gen_wx_consts.py` 从 Java 源码自动生成。

/// access_token 相关错误代码。
///
/// 发生以下情况时尝试刷新 access_token：
/// - 40001：获取 access_token 时 AppSecret 错误，或者 access_token 无效
/// - 40014：不合法的 access_token
/// - 42001：access_token 超时
pub const ACCESS_TOKEN_ERROR_CODES: &[i32] = &[40001, 40014, 42001];

/// 微信接口返回的参数 `errcode`。
pub const ERR_CODE: &str = "errcode";

/// 微信消息/参数类型常量组（对应 Java `WxConsts.XmlMsgType`）。
pub mod xml_msg_type {

    pub const TEXT: &str = "text";
    pub const IMAGE: &str = "image";
    pub const VOICE: &str = "voice";
    pub const SHORTVIDEO: &str = "shortvideo";
    pub const VIDEO: &str = "video";
    pub const NEWS: &str = "news";
    pub const MUSIC: &str = "music";
    pub const LOCATION: &str = "location";
    pub const LINK: &str = "link";
    pub const EVENT: &str = "event";
    pub const DEVICE_TEXT: &str = "device_text";
    pub const DEVICE_EVENT: &str = "device_event";
    pub const DEVICE_STATUS: &str = "device_status";
    pub const HARDWARE: &str = "hardware";
    pub const TRANSFER_CUSTOMER_SERVICE: &str = "transfer_customer_service";
    pub const TRANSFER_BIZ_AI_IVR: &str = "transfer_biz_ai_ivr";
    pub const UPDATE_TASKCARD: &str = "update_taskcard";
    pub const UPDATE_BUTTON: &str = "update_button";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.KefuMsgType`）。
pub mod kefu_msg_type {

    pub const TEXT: &str = "text";
    pub const IMAGE: &str = "image";
    pub const VOICE: &str = "voice";
    pub const VIDEO: &str = "video";
    pub const MUSIC: &str = "music";
    pub const NEWS: &str = "news";
    pub const MPNEWS: &str = "mpnews";
    pub const MARKDOWN: &str = "markdown";
    pub const FILE: &str = "file";
    pub const TEXTCARD: &str = "textcard";
    pub const WXCARD: &str = "wxcard";
    pub const TRANSFER_CUSTOMER_SERVICE: &str = "transfer_customer_service";
    pub const MINIPROGRAMPAGE: &str = "miniprogrampage";
    pub const TASKCARD: &str = "taskcard";
    pub const MSGMENU: &str = "msgmenu";
    pub const MINIPROGRAM_NOTICE: &str = "miniprogram_notice";
    pub const TEMPLATE_CARD: &str = "template_card";
    pub const MP_NEWS_ARTICLE: &str = "mpnewsarticle";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.SchoolContactMsgType`）。
pub mod school_contact_msg_type {

    pub const TEXT: &str = "text";
    pub const IMAGE: &str = "image";
    pub const VOICE: &str = "voice";
    pub const VIDEO: &str = "video";
    pub const FILE: &str = "file";
    pub const NEWS: &str = "news";
    pub const MPNEWS: &str = "mpnews";
    pub const MINIPROGRAM: &str = "miniprogram";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.TemplateCardType`）。
pub mod template_card_type {

    pub const TEXT_NOTICE: &str = "text_notice";
    pub const NEWS_NOTICE: &str = "news_notice";
    pub const BUTTON_INTERACTION: &str = "button_interaction";
    pub const VOTE_INTERACTION: &str = "vote_interaction";
    pub const MULTIPLE_INTERACTION: &str = "multiple_interaction";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.KefuMsgSafe`）。
pub mod kefu_msg_safe {

    pub const NO: &str = "0";
    pub const YES: &str = "1";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.MassMsgType`）。
pub mod mass_msg_type {

    pub const MPNEWS: &str = "mpnews";
    pub const TEXT: &str = "text";
    pub const VOICE: &str = "voice";
    pub const IMAGE: &str = "image";
    pub const IMAGES: &str = "images";
    pub const MPVIDEO: &str = "mpvideo";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.MassMsgStatus`）。
pub mod mass_msg_status {

    pub const SEND_SUCCESS: &str = "send success";
    pub const SEND_FAIL: &str = "send fail";
    pub const ERR_10001: &str = "err(10001)";
    pub const ERR_20001: &str = "err(20001)";
    pub const ERR_20004: &str = "err(20004)";
    pub const ERR_20002: &str = "err(20002)";
    pub const ERR_20006: &str = "err(20006)";
    pub const ERR_20008: &str = "err(20008)";
    pub const ERR_20013: &str = "err(20013)";
    pub const ERR_22000: &str = "err(22000)";
    pub const ERR_21000: &str = "err(21000)";
    pub const ERR_30001: &str = "err(30001)";
    pub const ERR_30002: &str = "err(30002)";
    pub const ERR_30003: &str = "err(30003)";
    pub const ERR_40001: &str = "err(40001)";
    pub const ERR_40002: &str = "err(40002)";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.EventType`）。
pub mod event_type {

    pub const SUBSCRIBE: &str = "subscribe";
    pub const UNSUBSCRIBE: &str = "unsubscribe";
    pub const SCAN: &str = "SCAN";
    pub const LOCATION: &str = "LOCATION";
    pub const CLICK: &str = "CLICK";
    pub const VIEW: &str = "VIEW";
    pub const MASS_SEND_JOB_FINISH: &str = "MASSSENDJOBFINISH";
    pub const SYS_APPROVAL_CHANGE: &str = "sys_approval_change";
    pub const SCANCODE_PUSH: &str = "scancode_push";
    pub const SCANCODE_WAITMSG: &str = "scancode_waitmsg";
    pub const PIC_SYSPHOTO: &str = "pic_sysphoto";
    pub const PIC_PHOTO_OR_ALBUM: &str = "pic_photo_or_album";
    pub const PIC_WEIXIN: &str = "pic_weixin";
    pub const LOCATION_SELECT: &str = "location_select";
    pub const USER_INFO_MODIFIED: &str = "user_info_modified";
    pub const USER_AUTHORIZATION_REVOKE: &str = "user_authorization_revoke";
    pub const TEMPLATE_SEND_JOB_FINISH: &str = "TEMPLATESENDJOBFINISH";
    pub const MERCHANT_ORDER: &str = "merchant_order";
    pub const CARD_PASS_CHECK: &str = "card_pass_check";
    pub const CARD_NOT_PASS_CHECK: &str = "card_not_pass_check";
    pub const CARD_USER_GET_CARD: &str = "user_get_card";
    pub const CARD_USER_GIFTING_CARD: &str = "user_gifting_card";
    pub const WXA_MEDIA_CHECK: &str = "wxa_media_check";
    pub const CARD_USER_CONSUME_CARD: &str = "user_consume_card";
    pub const CARD_USER_PAY_FROM_PAY_CELL: &str = "user_pay_from_pay_cell";
    pub const CARD_SUBMIT_MEMBERCARD_USER_INFO: &str = "submit_membercard_user_info";
    pub const CARD_USER_VIEW_CARD: &str = "user_view_card";
    pub const CARD_USER_DEL_CARD: &str = "user_del_card";
    pub const CARD_USER_ENTER_SESSION_FROM_CARD: &str = "user_enter_session_from_card";
    pub const CARD_UPDATE_MEMBER_CARD: &str = "update_member_card";
    pub const CARD_SKU_REMIND: &str = "card_sku_remind";
    pub const CARD_PAY_ORDER: &str = "card_pay_order";
    pub const WEAPP_AUDIT_SUCCESS: &str = "weapp_audit_success";
    pub const WEAPP_AUDIT_FAIL: &str = "weapp_audit_fail";
    pub const WEAPP_AUDIT_DELAY: &str = "weapp_audit_delay";
    pub const OPEN_PRODUCT_ORDER_PAY: &str = "open_product_order_pay";
    pub const VIEW_MINIPROGRAM: &str = "view_miniprogram";
    pub const SUBSCRIBE_MSG_POPUP_EVENT: &str = "subscribe_msg_popup_event";
    pub const SUBSCRIBE_MSG_CHANGE_EVENT: &str = "subscribe_msg_change_event";
    pub const SUBSCRIBE_MSG_SENT_EVENT: &str = "subscribe_msg_sent_event";
    pub const WXA_NICKNAME_AUDIT: &str = "wxa_nickname_audit";
    pub const WXA_ILLEGAL_RECORD: &str = "wxa_illegal_record";
    pub const WXA_APPEAL_RECORD: &str = "wxa_appeal_record";
    pub const WXA_PRIVACY_APPLY: &str = "wxa_privacy_apply";
    pub const WXA_CATEGORY_AUDIT: &str = "wxa_category_audit";
    pub const WX_VERIFY_PAY_SUCC: &str = "wx_verify_pay_succ";
    pub const WX_VERIFY_DISPATCH: &str = "wx_verify_dispatch";
    pub const TRADE_MANAGE_REMIND_SHIPPING: &str = "trade_manage_remind_shipping";
    pub const TRADE_MANAGE_ORDER_SETTLEMENT: &str = "trade_manage_order_settlement";
    pub const XPAY_SUBSCRIBE_IOS_REFUND_QUERY_NOTIFY: &str =
        "xpay_subscribe_ios_refund_query_notify";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.MediaFileType`）。
pub mod media_file_type {

    pub const IMAGE: &str = "image";
    pub const VOICE: &str = "voice";
    pub const VIDEO: &str = "video";
    pub const THUMB: &str = "thumb";
    pub const FILE: &str = "file";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.MenuButtonType`）。
pub mod menu_button_type {

    pub const CLICK: &str = "click";
    pub const VIEW: &str = "view";
    pub const MINIPROGRAM: &str = "miniprogram";
    pub const SCANCODE_PUSH: &str = "scancode_push";
    pub const SCANCODE_WAITMSG: &str = "scancode_waitmsg";
    pub const PIC_SYSPHOTO: &str = "pic_sysphoto";
    pub const PIC_PHOTO_OR_ALBUM: &str = "pic_photo_or_album";
    pub const PIC_WEIXIN: &str = "pic_weixin";
    pub const LOCATION_SELECT: &str = "location_select";
    pub const MEDIA_ID: &str = "media_id";
    pub const VIEW_LIMITED: &str = "view_limited";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.OAuth2Scope`）。
pub mod oauth2_scope {

    pub const SNSAPI_BASE: &str = "snsapi_base";
    pub const SNSAPI_USERINFO: &str = "snsapi_userinfo";
    pub const SNSAPI_PRIVATEINFO: &str = "snsapi_privateinfo";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.QrConnectScope`）。
pub mod qr_connect_scope {

    pub const SNSAPI_LOGIN: &str = "snsapi_login";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.MaterialType`）。
pub mod material_type {

    pub const NEWS: &str = "news";
    pub const VOICE: &str = "voice";
    pub const IMAGE: &str = "image";
    pub const VIDEO: &str = "video";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.NetCheckArgs`）。
pub mod net_check_args {

    pub const ACTIONDNS: &str = "dns";
    pub const ACTIONPING: &str = "ping";
    pub const ACTIONALL: &str = "all";
    pub const OPERATORUNICOM: &str = "UNICOM";
    pub const OPERATORCHINANET: &str = "CHINANET";
    pub const OPERATORCAP: &str = "CAP";
    pub const OPERATORDEFAULT: &str = "DEFAULT";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.AppIdType`）。
pub mod app_id_type {

    pub const MP_TYPE: &str = "mp";
    pub const MINI_TYPE: &str = "mini";
}

/// 微信消息/参数类型常量组（对应 Java `WxConsts.ArticleType`）。
pub mod article_type {

    pub const NEWS: &str = "news";
    pub const NEWS_PIC: &str = "newspic";
}
