//! 企业微信常量。
//!
//! 对应 Java `me.chanjar.weixin.cp.constant.WxCpConsts` 全部静态常量
//! （`@UtilityClass` 内部类 → Rust 子模块）。API URL 常量在
//! `crate::enums::url_*`（对应 Java `WxCpApiPathConsts`）。

/// 企业微信端推送过来的事件类型（对应 Java `WxCpConsts.EventType`）。
/// 参考文档：https://work.weixin.qq.com/api/doc#12974
pub mod event_type {
    /// 成员关注事件。
    pub const SUBSCRIBE: &str = "subscribe";
    /// 成员取消关注事件。
    pub const UNSUBSCRIBE: &str = "unsubscribe";
    /// 进入应用事件。
    pub const ENTER_AGENT: &str = "enter_agent";
    /// 上报地理位置。
    pub const LOCATION: &str = "LOCATION";
    /// 异步任务完成事件推送。
    pub const BATCH_JOB_RESULT: &str = "batch_job_result";
    /// 企业微信通讯录变更事件。
    pub const CHANGE_CONTACT: &str = "change_contact";
    /// 企业微信模板卡片事件推送。
    pub const TEMPLATE_CARD_EVENT: &str = "template_card_event";
    /// 点击菜单拉取消息的事件推送。
    pub const CLICK: &str = "click";
    /// 点击菜单跳转链接的事件推送。
    pub const VIEW: &str = "view";
    /// 扫码推事件的事件推送。
    pub const SCANCODE_PUSH: &str = "scancode_push";
    /// 扫码推事件且弹出“消息接收中”提示框的事件推送。
    pub const SCANCODE_WAITMSG: &str = "scancode_waitmsg";
    /// 弹出系统拍照发图的事件推送。
    pub const PIC_SYSPHOTO: &str = "pic_sysphoto";
    /// 弹出拍照或者相册发图的事件推送。
    pub const PIC_PHOTO_OR_ALBUM: &str = "pic_photo_or_album";
    /// 弹出微信相册发图器的事件推送。
    pub const PIC_WEIXIN: &str = "pic_weixin";
    /// 弹出地理位置选择器的事件推送。
    pub const LOCATION_SELECT: &str = "location_select";
    /// 任务卡片事件推送。
    pub const TASKCARD_CLICK: &str = "taskcard_click";
    /// 企业互联共享应用事件回调。
    pub const SHARE_AGENT_CHANGE: &str = "share_agent_change";
    /// 上下游共享应用事件回调。
    pub const SHARE_CHAIN_CHANGE: &str = "share_chain_change";
    /// 通用模板卡片右上角菜单事件推送。
    pub const TEMPLATE_CARD_MENU_EVENT: &str = "template_card_menu_event";
    /// 长期未使用应用临时停用事件。
    pub const CLOSE_INACTIVE_AGENT: &str = "close_inactive_agent";
    /// 长期未使用应用重新启用事件。
    pub const REOPEN_INACTIVE_AGENT: &str = "reopen_inactive_agent";
    /// 企业成员添加外部联系人事件推送 & 会话存档客户同意进行聊天内容存档事件回调事件。
    pub const CHANGE_EXTERNAL_CONTACT: &str = "change_external_contact";
    /// 客户群事件推送。
    pub const CHANGE_EXTERNAL_CHAT: &str = "change_external_chat";
    /// 企业客户标签事件推送。
    pub const CHANGE_EXTERNAL_TAG: &str = "change_external_tag";
    /// 企业微信审批事件推送（自建应用审批）。
    /// https://developer.work.weixin.qq.com/document/path/90269
    pub const OPEN_APPROVAL_CHANGE: &str = "open_approval_change";
    /// 企业微信审批事件推送（系统审批）。
    pub const SYS_APPROVAL_CHANGE: &str = "sys_approval_change";
    /// 修改日历事件。
    pub const MODIFY_CALENDAR: &str = "modify_calendar";
    /// 删除日历事件。
    pub const DELETE_CALENDAR: &str = "delete_calendar";
    /// 添加日程事件。
    pub const ADD_SCHEDULE: &str = "add_schedule";
    /// 修改日程事件。
    pub const MODIFY_SCHEDULE: &str = "modify_schedule";
    /// 删除日程事件。
    pub const DELETE_SCHEDULE: &str = "delete_schedule";
    /// 日程回执事件。
    pub const RESPOND_SCHEDULE: &str = "respond_schedule";
    /// 会议室预定事件。
    pub const BOOK_MEETING_ROOM: &str = "book_meeting_room";
    /// 会议室取消事件。
    pub const CANCEL_MEETING_ROOM: &str = "cancel_meeting_room";
    /// 家校通讯录事件。
    pub const CHANGE_SCHOOL_CONTACT: &str = "change_school_contact";
    /// 产生会话回调事件。
    pub const MSGAUDIT_NOTIFY: &str = "msgaudit_notify";
    /// 直播回调事件。
    pub const LIVING_STATUS_CHANGE: &str = "living_status_change";
    /// 微信客服消息事件。
    pub const KF_MSG_OR_EVENT: &str = "kf_msg_or_event";
    /// 客服账号授权变更事件。
    pub const KF_ACCOUNT_AUTH_CHANGE: &str = "kf_account_auth_change";
    /// 获客助手事件通知。
    pub const CUSTOMER_ACQUISITION: &str = "customer_acquisition";
    /// 异步上传临时素材结果回调通知。
    /// https://developer.work.weixin.qq.com/document/path/96488#%E5%9B%9E%E8%B0%83%E5%BC%82%E6%AD%A5%E4%BB%BB%E5%8A%A1%E7%BB%93%E6%9E%9C
    pub const UPLOAD_MEDIA_JOB_FINISH: &str = "upload_media_job_finish";
}

/// 获客助手事件通知 CHANGE_TYPE（对应 Java
/// `WxCpConsts.CustomerAcquisitionChangeType`）。
/// https://developer.work.weixin.qq.com/document/path/97299
pub mod customer_acquisition_change_type {
    /// 获客额度即将耗尽事件。
    pub const BALANCE_LOW: &str = "balance_low";
    /// 使用量已经耗尽事件。
    pub const BALANCE_EXHAUSTED: &str = "balance_exhausted";
    /// 获客链接不可用事件。
    pub const LINK_UNAVAILABLE: &str = "link_unavailable";
    /// 微信客户发起会话事件。
    pub const CUSTOMER_START_CHAT: &str = "customer_start_chat";
    /// 删除获客链接事件。
    pub const DELETE_LINK: &str = "delete_link";
    /// 通过获客链接申请好友事件。
    pub const FRIEND_REQUEST: &str = "friend_request";
}

/// 会话存档事件 CHANGE_TYPE（对应 Java `WxCpConsts.MsgAuditChangeType`）。
/// https://developer.work.weixin.qq.com/document/path/92005
pub mod msg_audit_change_type {
    /// 会话存档客户同意进行聊天内容存档事件回调。
    pub const MSG_AUDIT_APPROVED: &str = "msg_audit_approved";
}

/// 会话存档媒体类型（对应 Java `WxCpConsts.MsgAuditMediaType`）。
/// https://developer.work.weixin.qq.com/document/path/91774
pub mod msg_audit_media_type {
    /// 图片。
    pub const IMAGE: &str = "image";
    /// 语音。
    pub const VOICE: &str = "voice";
    /// 视频。
    pub const VIDEO: &str = "video";
    /// 表情。
    pub const EMOTION: &str = "emotion";
    /// 文件。
    pub const FILE: &str = "file";
    /// 音频存档消息。
    pub const MEETING_VOICE_CALL: &str = "meeting_voice_call";
    /// 音频共享文档消息。
    pub const VOIP_DOC_SHARE: &str = "voip_doc_share";

    /// 会话存档媒体文件后缀（对应 Java
    /// `WxCpConsts.MsgAuditMediaType.MsgAuditSuffix`）。
    pub mod msg_audit_suffix {
        /// jpg 后缀。
        pub const JPG: &str = ".jpg";
        /// png 后缀。
        pub const PNG: &str = ".png";
        /// gif 后缀。
        pub const GIF: &str = ".gif";
        /// mp4 后缀。
        pub const MP4: &str = ".mp4";
        /// amr 后缀。
        pub const AMR: &str = ".amr";
    }
}

/// 家校通讯录变更事件 CHANGE_TYPE（对应 Java
/// `WxCpConsts.SchoolContactChangeType`）。
pub mod school_contact_change_type {
    /// 部门变更事件。https://developer.work.weixin.qq.com/document/path/92052
    pub const CREATE_DEPARTMENT: &str = "create_department";
    /// 部门变更事件。
    pub const UPDATE_DEPARTMENT: &str = "update_department";
    /// 部门变更事件。
    pub const DELETE_DEPARTMENT: &str = "delete_department";
    /// 成员变更事件。https://developer.work.weixin.qq.com/document/path/92032
    pub const CREATE_STUDENT: &str = "create_student";
    /// 成员变更事件。
    pub const UPDATE_STUDENT: &str = "update_student";
    /// 成员变更事件。
    pub const DELETE_STUDENT: &str = "delete_student";
    /// 成员变更事件。
    pub const CREATE_PARENT: &str = "create_parent";
    /// 成员变更事件。
    pub const UPDATE_PARENT: &str = "update_parent";
    /// 成员变更事件。
    pub const DELETE_PARENT: &str = "delete_parent";
    /// 成员变更事件。
    pub const SUBSCRIBE: &str = "subscribe";
    /// 成员变更事件。
    pub const UNSUBSCRIBE: &str = "unsubscribe";
}

/// 企业外部联系人变更事件的 CHANGE_TYPE（对应 Java
/// `WxCpConsts.ExternalContactChangeType`）。
pub mod external_contact_change_type {
    /// 新增外部联系人。
    pub const ADD_EXTERNAL_CONTACT: &str = "add_external_contact";
    /// 编辑外部联系人。
    pub const EDIT_EXTERNAL_CONTACT: &str = "edit_external_contact";
    /// 删除外部联系人。
    pub const DEL_EXTERNAL_CONTACT: &str = "del_external_contact";
    /// 外部联系人免验证添加成员事件。
    pub const ADD_HALF_EXTERNAL_CONTACT: &str = "add_half_external_contact";
    /// 删除跟进成员事件。
    pub const DEL_FOLLOW_USER: &str = "del_follow_user";
    /// 客户接替失败事件。
    pub const TRANSFER_FAIL: &str = "transfer_fail";

    /// 客户接替失败原因（对应 Java
    /// `WxCpConsts.ExternalContactChangeType.ExternalContactTransferFailReason`）。
    pub mod external_contact_transfer_fail_reason {
        /// 客户拒绝。
        pub const CUSTOMER_REFUSED: &str = "customer_refused";
        /// 接替成员的客户数达到上限。
        pub const CUSTOMER_LIMIT_EXCEED: &str = "customer_limit_exceed";
    }
}

/// 客户群变更事件 CHANGE_TYPE（对应 Java `WxCpConsts.ExternalChatChangeType`）。
pub mod external_chat_change_type {
    /// 客户群变更事件。
    pub const CREATE: &str = "create";
    /// 客户群变更事件。
    pub const UPDATE: &str = "update";
    /// 客户群解散事件。
    pub const DISMISS: &str = "dismiss";

    /// 客户群变更详情（对应 Java
    /// `WxCpConsts.ExternalChatChangeType.ExternalChatUpdateDetail`）。
    pub mod external_chat_update_detail {
        /// 成员入群。
        pub const ADD_MEMBER: &str = "add_member";
        /// 成员退群。
        pub const DEL_MEMBER: &str = "del_member";
        /// 群主变更。
        pub const CHANGE_OWNER: &str = "change_owner";
        /// 群名变更。
        pub const CHANGE_NAME: &str = "change_name";
        /// 群公告变更。
        pub const CHANGE_NOTICE: &str = "change_notice";
    }
}

/// 企业客户标签变更事件 CHANGE_TYPE（对应 Java
/// `WxCpConsts.ExternalTagChangeType`）。
pub mod external_tag_change_type {
    /// 创建企业客户标签。
    pub const CREATE: &str = "create";
    /// 变更企业客户标签。
    pub const UPDATE: &str = "update";
    /// 删除企业客户标签。
    pub const DELETE: &str = "delete";
    /// 重排企业客户标签。
    pub const SHUFFLE: &str = "shuffle";
}

/// 客户标签类型（对应 Java `WxCpConsts.TageType`，Java 拼写如此）。
pub mod tage_type {
    /// 标签。
    pub const TAG: &str = "tag";
    /// 标签组。
    pub const TAG_GROUP: &str = "tag_group";
}

/// 企业微信通讯录变更事件 CHANGE_TYPE（对应 Java
/// `WxCpConsts.ContactChangeType`）。
pub mod contact_change_type {
    /// 新增成员事件。
    pub const CREATE_USER: &str = "create_user";
    /// 更新成员事件。
    pub const UPDATE_USER: &str = "update_user";
    /// 删除成员事件。
    pub const DELETE_USER: &str = "delete_user";
    /// 新增部门事件。
    pub const CREATE_PARTY: &str = "create_party";
    /// 更新部门事件。
    pub const UPDATE_PARTY: &str = "update_party";
    /// 删除部门事件。
    pub const DELETE_PARTY: &str = "delete_party";
    /// 标签成员变更事件。
    pub const UPDATE_TAG: &str = "update_tag";
}

/// 互联企业发送应用消息的消息类型（对应 Java `WxCpConsts.LinkedCorpMsgType`）。
pub mod linked_corp_msg_type {
    /// 文本消息。
    pub const TEXT: &str = "text";
    /// 图片消息。
    pub const IMAGE: &str = "image";
    /// 视频消息。
    pub const VIDEO: &str = "video";
    /// 图文消息（点击跳转到外链）。
    pub const NEWS: &str = "news";
    /// 图文消息（点击跳转到图文消息页面）。
    pub const MPNEWS: &str = "mpnews";
    /// markdown 消息。
    /// （目前仅支持 markdown 语法的子集，微工作台（原企业号）不支持展示 markdown 消息）
    pub const MARKDOWN: &str = "markdown";
    /// 发送文件。
    pub const FILE: &str = "file";
    /// 文本卡片消息。
    pub const TEXTCARD: &str = "textcard";
    /// 小程序通知消息。
    pub const MINIPROGRAM_NOTICE: &str = "miniprogram_notice";
}

/// 群机器人的消息类型（对应 Java `WxCpConsts.GroupRobotMsgType`）。
pub mod group_robot_msg_type {
    /// 文本消息。
    pub const TEXT: &str = "text";
    /// 图片消息。
    pub const IMAGE: &str = "image";
    /// markdown 消息。
    pub const MARKDOWN: &str = "markdown";
    /// markdown_v2 消息。
    pub const MARKDOWN_V2: &str = "markdown_v2";
    /// 图文消息（点击跳转到外链）。
    pub const NEWS: &str = "news";
    /// 文件类型消息。
    pub const FILE: &str = "file";
    /// 语音类型消息。
    pub const VOICE: &str = "voice";
    /// 模版类型消息。
    pub const TEMPLATE_CARD: &str = "template_card";
}

/// 应用推送消息的消息类型（对应 Java `WxCpConsts.AppChatMsgType`）。
pub mod app_chat_msg_type {
    /// 文本消息。
    pub const TEXT: &str = "text";
    /// 图片消息。
    pub const IMAGE: &str = "image";
    /// 语音消息。
    pub const VOICE: &str = "voice";
    /// 视频消息。
    pub const VIDEO: &str = "video";
    /// 发送文件（CP 专用）。
    pub const FILE: &str = "file";
    /// 文本卡片消息（CP 专用）。
    pub const TEXTCARD: &str = "textcard";
    /// 图文消息（点击跳转到外链）。
    pub const NEWS: &str = "news";
    /// 图文消息（点击跳转到图文消息页面）。
    pub const MPNEWS: &str = "mpnews";
    /// markdown 消息。
    pub const MARKDOWN: &str = "markdown";
}

/// 工作台类型（对应 Java `WxCpConsts.WorkBenchType`）。
pub mod work_bench_type {
    /// 关键数据型。
    pub const KEYDATA: &str = "keydata";
    /// 图片型。
    pub const IMAGE: &str = "image";
    /// 列表型。
    pub const LIST: &str = "list";
    /// webview 型。
    pub const WEBVIEW: &str = "webview";
}

/// 欢迎语消息类型（对应 Java `WxCpConsts.WelcomeMsgType`）。
pub mod welcome_msg_type {
    /// 图片消息。
    pub const IMAGE: &str = "image";
    /// 图文消息。
    pub const LINK: &str = "link";
    /// 视频消息。
    pub const VIDEO: &str = "video";
    /// 小程序消息。
    pub const MINIPROGRAM: &str = "miniprogram";
    /// 文件消息。
    pub const FILE: &str = "file";
}

/// 商品附件类型（对应 Java `WxCpConsts.ProductAttachmentType`）。
pub mod product_attachment_type {
    /// 图片消息。
    pub const IMAGE: &str = "image";
}
