//! 外部联系人相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.ExternalContact`。

/// 获取外部联系人详情（旧地址，对应 Java `@Deprecated GET_EXTERNAL_CONTACT`）。
pub const GET_EXTERNAL_CONTACT: &str = "/cgi-bin/crm/get_external_contact?external_userid=";

/// 配置客户联系「联系我」方式。
pub const ADD_CONTACT_WAY: &str = "/cgi-bin/externalcontact/add_contact_way";
/// 获取企业已配置的「联系我」方式。
pub const GET_CONTACT_WAY: &str = "/cgi-bin/externalcontact/get_contact_way";
/// 获取企业已配置的「联系我」方式列表。
pub const LIST_CONTACT_WAY: &str = "/cgi-bin/externalcontact/list_contact_way";
/// 更新企业已配置的「联系我」方式。
pub const UPDATE_CONTACT_WAY: &str = "/cgi-bin/externalcontact/update_contact_way";
/// 删除企业已配置的「联系我」方式。
pub const DEL_CONTACT_WAY: &str = "/cgi-bin/externalcontact/del_contact_way";
/// 结束临时会话。
pub const CLOSE_TEMP_CHAT: &str = "/cgi-bin/externalcontact/close_temp_chat";
/// 获取配置了客户联系功能的成员列表。
pub const GET_FOLLOW_USER_LIST: &str = "/cgi-bin/externalcontact/get_follow_user_list";
/// 获取客户详情（`external_userid` 拼在路径后）。
pub const GET_CONTACT_DETAIL: &str = "/cgi-bin/externalcontact/get?external_userid=";
/// 外部联系人 openid 转换。
pub const CONVERT_TO_OPENID: &str = "/cgi-bin/externalcontact/convert_to_openid";
/// unionid 转 external_userid。
pub const UNIONID_TO_EXTERNAL_USERID: &str = "/cgi-bin/externalcontact/unionid_to_external_userid";
/// unionid 转 external_userid（第三方应用）。
pub const UNIONID_TO_EXTERNAL_USERID_3RD: &str =
    "/cgi-bin/service/externalcontact/unionid_to_external_userid_3rd";
/// 获取新外部联系人 userid。
pub const GET_NEW_EXTERNAL_USERID: &str = "/cgi-bin/externalcontact/get_new_external_userid";
/// 转服务商主体外部联系人 userid。
pub const TO_SERVICE_EXTERNAL_USERID: &str = "/cgi-bin/externalcontact/to_service_external_userid";
/// 服务商主体外部联系人 userid 转换回企业主体。
pub const FROM_SERVICE_EXTERNAL_USERID: &str =
    "/cgi-bin/externalcontact/from_service_external_userid";
/// 完成外部联系人 userid 迁移。
pub const FINISH_EXTERNAL_USERID_MIGRATION: &str =
    "/cgi-bin/externalcontact/finish_external_userid_migration";
/// 批量获取客户详情。
pub const GET_CONTACT_DETAIL_BATCH: &str = "/cgi-bin/externalcontact/batch/get_by_user?";
/// 获取客户列表。
pub const GET_CONTACT_LIST: &str = "/cgi-bin/externalcontact/contact_list?";
/// 修改客户备注信息。
pub const UPDATE_REMARK: &str = "/cgi-bin/externalcontact/remark";
/// 获取客户列表（`userid` 拼在路径后）。
pub const LIST_EXTERNAL_CONTACT: &str = "/cgi-bin/externalcontact/list?userid=";
/// 获取离职成员的客户列表。
pub const LIST_UNASSIGNED_CONTACT: &str = "/cgi-bin/externalcontact/get_unassigned_list";
/// 分配离职成员的客户（旧地址，对应 Java `@Deprecated TRANSFER_UNASSIGNED_CONTACT`）。
pub const TRANSFER_UNASSIGNED_CONTACT: &str = "/cgi-bin/externalcontact/transfer";
/// 分配在职成员的客户。
pub const TRANSFER_CUSTOMER: &str = "/cgi-bin/externalcontact/transfer_customer";
/// 查询客户接替结果。
pub const TRANSFER_RESULT: &str = "/cgi-bin/externalcontact/transfer_result";
/// 分配离职成员的客户。
pub const RESIGNED_TRANSFER_CUSTOMER: &str = "/cgi-bin/externalcontact/resigned/transfer_customer";
/// 查询离职成员客户接替结果。
pub const RESIGNED_TRANSFER_RESULT: &str = "/cgi-bin/externalcontact/resigned/transfer_result";
/// 获取客户群列表。
pub const GROUP_CHAT_LIST: &str = "/cgi-bin/externalcontact/groupchat/list";
/// 获取客户群详情。
pub const GROUP_CHAT_INFO: &str = "/cgi-bin/externalcontact/groupchat/get";
/// openid 转群 chatid。
pub const OPENID_TO_CHATID: &str = "/cgi-bin/externalcontact/opengid_to_chatid";
/// 客户群交接。
pub const GROUP_CHAT_TRANSFER: &str = "/cgi-bin/externalcontact/groupchat/transfer";
/// 在职成员客户群交接。
pub const GROUP_CHAT_ONJOB_TRANSFER: &str = "/cgi-bin/externalcontact/groupchat/onjob_transfer";
/// 获取成员联系客户的行为数据。
pub const LIST_USER_BEHAVIOR_DATA: &str = "/cgi-bin/externalcontact/get_user_behavior_data";
/// 获取客户群统计数据。
pub const LIST_GROUP_CHAT_DATA: &str = "/cgi-bin/externalcontact/groupchat/statistic";
/// 配置客户群进群方式。
pub const ADD_JOIN_WAY: &str = "/cgi-bin/externalcontact/groupchat/add_join_way";
/// 获取客户群进群方式配置。
pub const GET_JOIN_WAY: &str = "/cgi-bin/externalcontact/groupchat/get_join_way";
/// 更新客户群进群方式配置。
pub const UPDATE_JOIN_WAY: &str = "/cgi-bin/externalcontact/groupchat/update_join_way";
/// 删除客户群进群方式配置。
pub const DEL_JOIN_WAY: &str = "/cgi-bin/externalcontact/groupchat/del_join_way";
/// 添加企业群发消息模板。
pub const ADD_MSG_TEMPLATE: &str = "/cgi-bin/externalcontact/add_msg_template";
/// 提醒成员群发。
pub const REMIND_GROUP_MSG_SEND: &str = "/cgi-bin/externalcontact/remind_groupmsg_send";
/// 停止企业群发。
pub const CANCEL_GROUP_MSG_SEND: &str = "/cgi-bin/externalcontact/cancel_groupmsg_send";
/// 发送新客户欢迎语。
pub const SEND_WELCOME_MSG: &str = "/cgi-bin/externalcontact/send_welcome_msg";
/// 获取企业标签库。
pub const GET_CORP_TAG_LIST: &str = "/cgi-bin/externalcontact/get_corp_tag_list";
/// 添加企业客户标签。
pub const ADD_CORP_TAG: &str = "/cgi-bin/externalcontact/add_corp_tag";
/// 编辑企业客户标签。
pub const EDIT_CORP_TAG: &str = "/cgi-bin/externalcontact/edit_corp_tag";
/// 删除企业客户标签。
pub const DEL_CORP_TAG: &str = "/cgi-bin/externalcontact/del_corp_tag";
/// 编辑客户企业标签。
pub const MARK_TAG: &str = "/cgi-bin/externalcontact/mark_tag";
/// 创建企业朋友圈任务。
pub const ADD_MOMENT_TASK: &str = "/cgi-bin/externalcontact/add_moment_task";
/// 获取企业朋友圈任务创建结果。
pub const GET_MOMENT_TASK_RESULT: &str = "/cgi-bin/externalcontact/get_moment_task_result";
/// 停止发表企业朋友圈。
pub const CANCEL_MOMENT_TASK: &str = "/cgi-bin/externalcontact/cancel_moment_task";
/// 获取企业朋友圈列表。
pub const GET_MOMENT_LIST: &str = "/cgi-bin/externalcontact/get_moment_list";
/// 获取企业朋友圈任务。
pub const GET_MOMENT_TASK: &str = "/cgi-bin/externalcontact/get_moment_task";
/// 获取企业朋友圈客户列表。
pub const GET_MOMENT_CUSTOMER_LIST: &str = "/cgi-bin/externalcontact/get_moment_customer_list";
/// 获取企业朋友圈发表结果。
pub const GET_MOMENT_SEND_RESULT: &str = "/cgi-bin/externalcontact/get_moment_send_result";
/// 获取企业朋友圈评论。
pub const GET_MOMENT_COMMENTS: &str = "/cgi-bin/externalcontact/get_moment_comments";
/// 获取群发成员发送结果。
pub const GET_GROUP_MSG_SEND_RESULT: &str = "/cgi-bin/externalcontact/get_groupmsg_send_result";
/// 获取群发任务。
pub const GET_GROUP_MSG_TASK: &str = "/cgi-bin/externalcontact/get_groupmsg_task";
/// 获取群发记录列表（v2）。
pub const GET_GROUP_MSG_LIST_V2: &str = "/cgi-bin/externalcontact/get_groupmsg_list_v2";
/// 获取群发记录结果。
pub const GET_GROUP_MSG_RESULT: &str = "/cgi-bin/externalcontact/get_group_msg_result";
/// 获取商品图册。
pub const GET_PRODUCT_ALBUM: &str = "/cgi-bin/externalcontact/get_product_album";
/// 获取商品图册列表。
pub const GET_PRODUCT_ALBUM_LIST: &str = "/cgi-bin/externalcontact/get_product_album_list";
/// 添加商品图册。
pub const ADD_PRODUCT_ALBUM: &str = "/cgi-bin/externalcontact/add_product_album";
/// 更新商品图册。
pub const UPDATE_PRODUCT_ALBUM: &str = "/cgi-bin/externalcontact/update_product_album";
/// 删除商品图册。
pub const DELETE_PRODUCT_ALBUM: &str = "/cgi-bin/externalcontact/delete_product_album";
/// 添加群欢迎语模板。
pub const GROUP_WELCOME_TEMPLATE_ADD: &str = "/cgi-bin/externalcontact/group_welcome_template/add";
/// 编辑群欢迎语模板。
pub const GROUP_WELCOME_TEMPLATE_EDIT: &str =
    "/cgi-bin/externalcontact/group_welcome_template/edit";
/// 获取群欢迎语模板。
pub const GROUP_WELCOME_TEMPLATE_GET: &str = "/cgi-bin/externalcontact/group_welcome_template/get";
/// 删除群欢迎语模板。
pub const GROUP_WELCOME_TEMPLATE_DEL: &str = "/cgi-bin/externalcontact/group_welcome_template/del";
/// 上传附件资源。
pub const UPLOAD_ATTACHMENT: &str = "/cgi-bin/media/upload_attachment";
/// 获取「联系客户」二维码。
pub const GET_SUBSCRIBE_QR_CODE: &str = "/cgi-bin/externalcontact/get_subscribe_qr_code";
/// 设置成员对外信息。
pub const SET_SUBSCRIBE_MODE: &str = "/cgi-bin/externalcontact/set_subscribe_mode";
/// 获取成员对外信息。
pub const GET_SUBSCRIBE_MODE: &str = "/cgi-bin/externalcontact/get_subscribe_mode";
/// 获取外部联系人详情（`external_userid` 拼在路径后）。
pub const EXTERNAL_CONTACT_GET: &str = "/cgi-bin/externalcontact/get?external_userid=";
/// 添加敏感词规则。
pub const ADD_INTERCEPT_RULE: &str = "/cgi-bin/externalcontact/add_intercept_rule";
/// 更新敏感词规则。
pub const UPDATE_INTERCEPT_RULE: &str = "/cgi-bin/externalcontact/update_intercept_rule";
/// 删除敏感词规则。
pub const DEL_INTERCEPT_RULE: &str = "/cgi-bin/externalcontact/del_intercept_rule";
/// 获取敏感词规则列表。
pub const GET_INTERCEPT_RULE_LIST: &str = "/cgi-bin/externalcontact/get_intercept_rule_list";
/// 获取敏感词规则详情。
pub const GET_INTERCEPT_RULE: &str = "/cgi-bin/externalcontact/get_intercept_rule";
/// 获取当前仍然有效的获客链接。
pub const CUSTOMER_ACQUISITION_LINK_LIST: &str =
    "/cgi-bin/externalcontact/customer_acquisition/list_link";
/// 获取获客链接详情。
pub const CUSTOMER_ACQUISITION_LINK_GET: &str = "/cgi-bin/externalcontact/customer_acquisition/get";
/// 创建获客链接。
pub const CUSTOMER_ACQUISITION_LINK_CREATE: &str =
    "/cgi-bin/externalcontact/customer_acquisition/create_link";
/// 编辑获客链接。
pub const CUSTOMER_ACQUISITION_LINK_UPDATE: &str =
    "/cgi-bin/externalcontact/customer_acquisition/update_link";
/// 删除获客链接。
pub const CUSTOMER_ACQUISITION_LINK_DELETE: &str =
    "/cgi-bin/externalcontact/customer_acquisition/delete_link";
/// 获取获客客户列表。
pub const CUSTOMER_ACQUISITION_CUSTOMER: &str =
    "/cgi-bin/externalcontact/customer_acquisition/customer";
/// 查询剩余使用量。
pub const CUSTOMER_ACQUISITION_QUOTA: &str = "/cgi-bin/externalcontact/customer_acquisition_quota";
/// 查询链接使用详情。
pub const CUSTOMER_ACQUISITION_STATISTIC: &str =
    "/cgi-bin/externalcontact/customer_acquisition/statistic";
