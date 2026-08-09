//! 小程序常量。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.constant.WxMaConstants`。
//! 仅提取本批次（门面骨架）所需的常用字符串/数值常量，其余随业务子域批次补齐。

/// 默认的 env_version 值（对应 Java `DEFAULT_ENV_VERSION`）。
pub const DEFAULT_ENV_VERSION: &str = "release";

/// 素材类型（对应 Java `WxMaConstants.MediaType`）。
pub mod media_type {
    /// 图片。
    pub const IMAGE: &str = "image";
}

/// 消息格式（对应 Java `WxMaConstants.MsgDataFormat`）。
pub mod msg_data_format {
    /// XML 格式。
    pub const XML: &str = "XML";
    /// JSON 格式。
    pub const JSON: &str = "JSON";
}

/// 客服消息的消息类型（对应 Java `WxMaConstants.KefuMsgType`）。
pub mod kefu_msg_type {
    /// 文本消息。
    pub const TEXT: &str = "text";
    /// 图片消息。
    pub const IMAGE: &str = "image";
    /// 图文链接。
    pub const LINK: &str = "link";
    /// 小程序卡片消息。
    pub const MA_PAGE: &str = "miniprogrampage";
}

/// 内容安全检测的媒体类型（对应 Java `WxMaConstants.SecCheckMediaType`）。
pub mod sec_check_media_type {
    /// 音频。
    pub const VOICE: i32 = 1;
    /// 图片。
    pub const IMAGE: i32 = 2;
}

/// 快递账号绑定类型（对应 Java `WxMaConstants.BindAccountType`）。
pub mod bind_account_type {
    /// 绑定。
    pub const BIND: &str = "bind";
    /// 解绑。
    pub const UNBIND: &str = "unbind";
}

/// 快递下单订单来源（对应 Java `WxMaConstants.OrderAddSource`）。
pub mod order_add_source {
    /// 小程序。
    pub const MINI_PROGRAM: i32 = 0;
    /// APP 或 H5。
    pub const APP_OR_H5: i32 = 2;
}

/// 小程序订阅消息跳转小程序类型（对应 Java `WxMaConstants.MiniProgramState`）。
pub mod mini_program_state {
    /// 开发版。
    pub const DEVELOPER: &str = "developer";
    /// 体验版。
    pub const TRIAL: &str = "trial";
    /// 正式版（默认）。
    pub const FORMAL: &str = "formal";
}

/// 进入小程序查看的语言类型（对应 Java `WxMaConstants.MiniProgramLang`）。
pub mod mini_program_lang {
    /// 简体中文（默认）。
    pub const ZH_CN: &str = "zh_CN";
    /// 英文。
    pub const EN_US: &str = "en_US";
    /// 繁体中文（香港）。
    pub const ZH_HK: &str = "zh_HK";
    /// 繁体中文（台湾）。
    pub const ZH_TW: &str = "zh_TW";
}

/// 小程序代码审核状态（对应 Java `WxMaConstants.AuditStatus`）。
pub mod audit_status {
    /// 无效。
    pub const INVALID: i32 = 0;
    /// 审核中。
    pub const ONGOING: i32 = 1;
    /// 已驳回。
    pub const REJECTED: i32 = 2;
    /// 已通过。
    pub const APPROVED: i32 = 3;
    /// 已撤回（重新提交）。
    pub const RECOMMIT: i32 = 4;
}
