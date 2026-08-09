//! OA（审批/打卡/日程/会议/会议室/微盘/文档/邮件）相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Oa`。

/// 打卡（https://developer.work.weixin.qq.com/document/path/94204）。
pub const GET_CORP_CHECKIN_OPTION: &str = "/cgi-bin/checkin/getcorpcheckinoption";
/// 获取打卡记录数据。
pub const GET_CHECKIN_DATA: &str = "/cgi-bin/checkin/getcheckindata";
/// 获取打卡规则。
pub const GET_CHECKIN_OPTION: &str = "/cgi-bin/checkin/getcheckinoption";
/// 获取打卡日报数据。
pub const GET_CHECKIN_DAY_DATA: &str = "/cgi-bin/checkin/getcheckin_daydata";
/// 获取打卡月报数据。
pub const GET_CHECKIN_MONTH_DATA: &str = "/cgi-bin/checkin/getcheckin_monthdata";
/// 获取打卡人员排班信息。
pub const GET_CHECKIN_SCHEDULE_DATA: &str = "/cgi-bin/checkin/getcheckinschedulist";
/// 设置打卡人员排班信息。
pub const SET_CHECKIN_SCHEDULE_DATA: &str = "/cgi-bin/checkin/setcheckinschedulist";
/// 录入打卡人员人脸信息。
pub const ADD_CHECK_IN_USER_FACE: &str = "/cgi-bin/checkin/addcheckinuserface";

/// 审批（https://developer.work.weixin.qq.com/document/path/91956）。
pub const COPY_TEMPLATE: &str = "/cgi-bin/oa/approval/copytemplate";
/// 获取审批模板详情。
pub const GET_TEMPLATE_DETAIL: &str = "/cgi-bin/oa/gettemplatedetail";
/// 创建审批模板。
pub const CREATE_TEMPLATE: &str = "/cgi-bin/oa/approval/create_template";
/// 更新审批模板。
pub const UPDATE_TEMPLATE: &str = "/cgi-bin/oa/approval/update_template";
/// 提交审批申请。
pub const APPLY_EVENT: &str = "/cgi-bin/oa/applyevent";
/// 批量获取审批单号。
pub const GET_APPROVAL_INFO: &str = "/cgi-bin/oa/getapprovalinfo";
/// 获取审批申请详情。
pub const GET_APPROVAL_DETAIL: &str = "/cgi-bin/oa/getapprovaldetail";
/// 获取审批数据（系统审批）。
pub const GET_APPROVAL_DATA: &str = "/cgi-bin/corp/getapprovaldata";

/// 获取企业假期管理配置。
pub const GET_CORP_CONF: &str = "/cgi-bin/oa/vacation/getcorpconf";
/// 获取成员假期余额。
pub const GET_USER_VACATION_QUOTA: &str = "/cgi-bin/oa/vacation/getuservacationquota";
/// 修改成员假期余额。
pub const SET_ONE_USER_QUOTA: &str = "/cgi-bin/oa/vacation/setoneuserquota";

/// 公费电话（https://developer.work.weixin.qq.com/document/path/93662）。
pub const GET_DIAL_RECORD: &str = "/cgi-bin/dial/get_dial_record";

/// 日程（https://developer.work.weixin.qq.com/document/path/93624）。
pub const CALENDAR_ADD: &str = "/cgi-bin/oa/calendar/add";
/// 更新日历。
pub const CALENDAR_UPDATE: &str = "/cgi-bin/oa/calendar/update";
/// 获取日历详情。
pub const CALENDAR_GET: &str = "/cgi-bin/oa/calendar/get";
/// 删除日历。
pub const CALENDAR_DEL: &str = "/cgi-bin/oa/calendar/del";

/// 添加日程。
pub const SCHEDULE_ADD: &str = "/cgi-bin/oa/schedule/add";
/// 更新日程。
pub const SCHEDULE_UPDATE: &str = "/cgi-bin/oa/schedule/update";
/// 获取日程详情。
pub const SCHEDULE_GET: &str = "/cgi-bin/oa/schedule/get";
/// 删除日程。
pub const SCHEDULE_DEL: &str = "/cgi-bin/oa/schedule/del";
/// 获取日历下的日程列表。
pub const SCHEDULE_LIST: &str = "/cgi-bin/oa/schedule/get_by_calendar";

/// 会议（https://developer.work.weixin.qq.com/document/path/93626）。
pub const MEETING_ADD: &str = "/cgi-bin/meeting/create";
/// 更新会议。
pub const MEETING_UPDATE: &str = "/cgi-bin/meeting/update";
/// 取消会议。
pub const MEETING_CANCEL: &str = "/cgi-bin/meeting/cancel";
/// 获取会议详情。
pub const MEETING_DETAIL: &str = "/cgi-bin/meeting/get_info";
/// 获取成员会议 ID 列表。
pub const GET_USER_MEETING_ID: &str = "/cgi-bin/meeting/get_user_meetingid";

/// 会议室（https://developer.work.weixin.qq.com/document/path/93624）。
pub const MEETINGROOM_ADD: &str = "/cgi-bin/oa/meetingroom/add";
/// 获取会议室列表。
pub const MEETINGROOM_LIST: &str = "/cgi-bin/oa/meetingroom/list";
/// 编辑会议室。
pub const MEETINGROOM_EDIT: &str = "/cgi-bin/oa/meetingroom/edit";
/// 删除会议室。
pub const MEETINGROOM_DEL: &str = "/cgi-bin/oa/meetingroom/del";
/// 获取会议室预定信息。
pub const MEETINGROOM_GET_BOOKING_INFO: &str = "/cgi-bin/oa/meetingroom/get_booking_info";
/// 预定会议室。
pub const MEETINGROOM_BOOK: &str = "/cgi-bin/oa/meetingroom/book";
/// 按日程预定会议室。
pub const MEETINGROOM_BOOK_BY_SCHEDULE: &str = "/cgi-bin/oa/meetingroom/book_by_schedule";
/// 按会议预定会议室。
pub const MEETINGROOM_BOOK_BY_MEETING: &str = "/cgi-bin/oa/meetingroom//book_by_meeting";
/// 取消预定会议室。
pub const MEETINGROOM_CANCEL_BOOK: &str = "/cgi-bin/oa/meetingroom/cancel_book";
/// 获取预定信息。
pub const MEETINGROOM_BOOKINFO_GET: &str = "/cgi-bin/oa/meetingroom/bookinfo/get";

/// 微盘（https://developer.work.weixin.qq.com/document/path/93654）。
pub const SPACE_CREATE: &str = "/cgi-bin/wedrive/space_create";
/// 重命名空间。
pub const SPACE_RENAME: &str = "/cgi-bin/wedrive/space_rename";
/// 解散空间。
pub const SPACE_DISMISS: &str = "/cgi-bin/wedrive/space_dismiss";
/// 获取空间信息。
pub const SPACE_INFO: &str = "/cgi-bin/wedrive/space_info";
/// 添加空间成员。
pub const SPACE_ACL_ADD: &str = "/cgi-bin/wedrive/space_acl_add";
/// 移除空间成员。
pub const SPACE_ACL_DEL: &str = "/cgi-bin/wedrive/space_acl_del";
/// 设置空间信息。
pub const SPACE_SETTING: &str = "/cgi-bin/wedrive/space_setting";
/// 分享空间。
pub const SPACE_SHARE: &str = "/cgi-bin/wedrive/space_share";
/// 获取文件列表。
pub const FILE_LIST: &str = "/cgi-bin/wedrive/file_list";
/// 上传文件。
pub const FILE_UPLOAD: &str = "/cgi-bin/wedrive/file_upload";
/// 下载文件。
pub const FILE_DOWNLOAD: &str = "/cgi-bin/wedrive/file_download";
/// 重命名文件。
pub const FILE_RENAME: &str = "/cgi-bin/wedrive/file_rename";
/// 新建文件。
pub const FILE_CREATE: &str = "/cgi-bin/wedrive/file_create";
/// 移动文件。
pub const FILE_MOVE: &str = "/cgi-bin/wedrive/file_move";
/// 删除文件。
pub const FILE_DELETE: &str = "/cgi-bin/wedrive/file_delete";
/// 获取文件信息。
pub const FILE_INFO: &str = "/cgi-bin/wedrive/file_info";
/// 添加文件成员。
pub const FILE_ACL_ADD: &str = "/cgi-bin/wedrive/file_acl_add";
/// 移除文件成员。
pub const FILE_ACL_DEL: &str = "/cgi-bin/wedrive/file_acl_del";
/// 设置文件信息。
pub const FILE_SETTING: &str = "/cgi-bin/wedrive/file_setting";
/// 分享文件。
pub const FILE_SHARE: &str = "/cgi-bin/wedrive/file_share";

/// 审批流程引擎（https://developer.work.weixin.qq.com/document/path/90269）。
pub const GET_OPEN_APPROVAL_DATA: &str = "/cgi-bin/corp/getopenapprovaldata";

/// 文档（https://developer.work.weixin.qq.com/document/path/97392）。
pub const WEDOC_CREATE_DOC: &str = "/cgi-bin/wedoc/create_doc";
/// 重命名文档。
pub const WEDOC_RENAME_DOC: &str = "/cgi-bin/wedoc/rename_doc";
/// 删除文档。
pub const WEDOC_DEL_DOC: &str = "/cgi-bin/wedoc/del_doc";
/// 获取文档基础信息。
pub const WEDOC_GET_DOC_BASE_INFO: &str = "/cgi-bin/wedoc/get_doc_base_info";
/// 分享文档。
pub const WEDOC_DOC_SHARE: &str = "/cgi-bin/wedoc/doc_share";
/// 获取文档成员权限。
pub const WEDOC_DOC_GET_AUTH: &str = "/cgi-bin/wedoc/doc_get_auth";
/// 修改文档加入规则。
pub const WEDOC_MOD_DOC_JOIN_RULE: &str = "/cgi-bin/wedoc/mod_doc_join_rule";
/// 修改文档成员。
pub const WEDOC_MOD_DOC_MEMBER: &str = "/cgi-bin/wedoc/mod_doc_member";
/// 修改文档安全设置。
pub const WEDOC_MOD_DOC_SAFETY_SETTING: &str = "/cgi-bin/wedoc/mod_doc_safty_setting";
/// 修改文档安全设置（旧名，对应 Java `@Deprecated WEDOC_MOD_DOC_SAFTY_SETTING`，
/// 与 `WEDOC_MOD_DOC_SAFETY_SETTING` 同值）。
pub const WEDOC_MOD_DOC_SAFTY_SETTING: &str = WEDOC_MOD_DOC_SAFETY_SETTING;
/// 创建收集表。
pub const WEDOC_CREATE_FORM: &str = "/cgi-bin/wedoc/create_collect";
/// 更新收集表。
pub const WEDOC_MODIFY_FORM: &str = "/cgi-bin/wedoc/modify_collect";
/// 获取收集表信息。
pub const WEDOC_GET_FORM_INFO: &str = "/cgi-bin/wedoc/get_form_info";
/// 获取收集表统计信息。
pub const WEDOC_GET_FORM_STATISTIC: &str = "/cgi-bin/wedoc/get_form_statistic";
/// 获取收集表回答。
pub const WEDOC_GET_FORM_ANSWER: &str = "/cgi-bin/wedoc/get_form_answer";
/// 更新表格。
pub const WEDOC_SPREADSHEET_BATCH_UPDATE: &str = "/cgi-bin/wedoc/spreadsheet/batch_update";
/// 获取表格属性。
pub const WEDOC_SPREADSHEET_GET_SHEET_PROPERTIES: &str =
    "/cgi-bin/wedoc/spreadsheet/get_sheet_properties";
/// 获取表格数据。
pub const WEDOC_SPREADSHEET_GET_SHEET_RANGE_DATA: &str =
    "/cgi-bin/wedoc/spreadsheet/get_sheet_range_data";
/// 获取文档内容。
pub const WEDOC_GET_DOC_DATA: &str = "/cgi-bin/wedoc/get_doc_data";
/// 修改文档。
pub const WEDOC_MOD_DOC: &str = "/cgi-bin/wedoc/mod_doc";
/// 上传文档图片。
pub const WEDOC_UPLOAD_DOC_IMAGE: &str = "/cgi-bin/wedoc/upload_doc_image";
/// 添加文档管理员。
pub const WEDOC_ADD_ADMIN: &str = "/cgi-bin/wedoc/add_admin";
/// 删除文档管理员。
pub const WEDOC_DEL_ADMIN: &str = "/cgi-bin/wedoc/del_admin";
/// 获取文档管理员列表。
pub const WEDOC_GET_ADMIN_LIST: &str = "/cgi-bin/wedoc/get_admin_list";
/// 获取智能表格权限。
pub const WEDOC_SMARTSHEET_GET_SHEET_AUTH: &str = "/cgi-bin/wedoc/smartsheet/get_sheet_auth";
/// 修改智能表格权限。
pub const WEDOC_SMARTSHEET_MOD_SHEET_AUTH: &str = "/cgi-bin/wedoc/smartsheet/mod_sheet_auth";
/// 获取智能表格。
pub const WEDOC_SMARTSHEET_GET_SHEET: &str = "/cgi-bin/wedoc/smartsheet/get_sheet";
/// 新增智能表格。
pub const WEDOC_SMARTSHEET_ADD_SHEET: &str = "/cgi-bin/wedoc/smartsheet/add_sheet";
/// 删除智能表格。
pub const WEDOC_SMARTSHEET_DELETE_SHEET: &str = "/cgi-bin/wedoc/smartsheet/delete_sheet";
/// 更新智能表格。
pub const WEDOC_SMARTSHEET_UPDATE_SHEET: &str = "/cgi-bin/wedoc/smartsheet/update_sheet";
/// 获取智能表格视图。
pub const WEDOC_SMARTSHEET_GET_VIEWS: &str = "/cgi-bin/wedoc/smartsheet/get_views";
/// 新增智能表格视图。
pub const WEDOC_SMARTSHEET_ADD_VIEW: &str = "/cgi-bin/wedoc/smartsheet/add_view";
/// 删除智能表格视图。
pub const WEDOC_SMARTSHEET_DELETE_VIEWS: &str = "/cgi-bin/wedoc/smartsheet/delete_views";
/// 更新智能表格视图。
pub const WEDOC_SMARTSHEET_UPDATE_VIEW: &str = "/cgi-bin/wedoc/smartsheet/update_view";
/// 获取智能表格字段。
pub const WEDOC_SMARTSHEET_GET_FIELDS: &str = "/cgi-bin/wedoc/smartsheet/get_fields";
/// 新增智能表格字段。
pub const WEDOC_SMARTSHEET_ADD_FIELDS: &str = "/cgi-bin/wedoc/smartsheet/add_fields";
/// 删除智能表格字段。
pub const WEDOC_SMARTSHEET_DELETE_FIELDS: &str = "/cgi-bin/wedoc/smartsheet/delete_fields";
/// 更新智能表格字段。
pub const WEDOC_SMARTSHEET_UPDATE_FIELDS: &str = "/cgi-bin/wedoc/smartsheet/update_fields";
/// 获取智能表格记录。
pub const WEDOC_SMARTSHEET_GET_RECORDS: &str = "/cgi-bin/wedoc/smartsheet/get_records";
/// 新增智能表格记录。
pub const WEDOC_SMARTSHEET_ADD_RECORDS: &str = "/cgi-bin/wedoc/smartsheet/add_records";
/// 删除智能表格记录。
pub const WEDOC_SMARTSHEET_DELETE_RECORDS: &str = "/cgi-bin/wedoc/smartsheet/delete_records";
/// 更新智能表格记录。
pub const WEDOC_SMARTSHEET_UPDATE_RECORDS: &str = "/cgi-bin/wedoc/smartsheet/update_records";

/// 邮件（https://developer.work.weixin.qq.com/document/path/95486）。
pub const EXMAIL_APP_COMPOSE_SEND: &str = "/cgi-bin/exmail/app/compose_send";
