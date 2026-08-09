//! 外部联系人管理服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpExternalContactService`。

use async_trait::async_trait;

use chrono::{DateTime, Utc};
use wx_rust_common::bean::result::WxMediaUploadResult;
use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpAddMomentResult, WxCpAddMomentTask, WxCpBaseResp, WxCpContactWayInfo, WxCpContactWayList,
    WxCpContactWayResult, WxCpCustomerAcquisitionCreateResult, WxCpCustomerAcquisitionCustomerList,
    WxCpCustomerAcquisitionInfo, WxCpCustomerAcquisitionList, WxCpCustomerAcquisitionQuota,
    WxCpCustomerAcquisitionRequest, WxCpCustomerAcquisitionStatistic, WxCpExternalContactBatchInfo,
    WxCpExternalContactInfo, WxCpExternalContactListInfo, WxCpExternalUserIdList,
    WxCpGetMomentComments, WxCpGetMomentCustomerList, WxCpGetMomentList, WxCpGetMomentSendResult,
    WxCpGetMomentTask, WxCpGetMomentTaskResult, WxCpGroupJoinWayInfo, WxCpGroupJoinWayResult,
    WxCpGroupMsgListResult, WxCpGroupMsgResult, WxCpGroupMsgSendResult, WxCpGroupMsgTaskResult,
    WxCpGroupWelcomeTemplateResult, WxCpInterceptRule, WxCpInterceptRuleAddRequest,
    WxCpInterceptRuleInfo, WxCpInterceptRuleList, WxCpMsgTemplate, WxCpMsgTemplateAddResult,
    WxCpNewExternalUserIdList, WxCpProductAlbumInfo, WxCpProductAlbumListResult,
    WxCpProductAlbumResult, WxCpUpdateRemarkRequest, WxCpUserExternalGroupChatInfo,
    WxCpUserExternalGroupChatList, WxCpUserExternalGroupChatStatistic,
    WxCpUserExternalGroupChatTransferResp, WxCpUserExternalTagGroupInfo,
    WxCpUserExternalTagGroupList, WxCpUserExternalUnassignList,
    WxCpUserExternalUserBehaviorStatistic, WxCpUserTransferCustomerReq,
    WxCpUserTransferCustomerResp, WxCpUserTransferResultResp, WxCpWelcomeMsg,
};

/// 外部联系人管理服务。
#[async_trait]
pub trait WxCpExternalContactService: Send + Sync {
    /// 配置客户联系「联系我」方式（对应 Java
    /// `WxCpExternalContactService.addContactWay(WxCpContactWayInfo)`）。
    async fn add_contact_way(
        &self,
        info: &WxCpContactWayInfo,
    ) -> Result<WxCpContactWayResult, WxErrorException>;

    /// 获取企业已配置的「联系我」方式（对应 Java
    /// `WxCpExternalContactService.getContactWay(String)`）。
    async fn get_contact_way(
        &self,
        config_id: &str,
    ) -> Result<WxCpContactWayInfo, WxErrorException>;

    /// 获取企业已配置的「联系我」列表（对应 Java
    /// `WxCpExternalContactService.listContactWay(Long, Long, String, Long)`）。
    async fn list_contact_way(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<WxCpContactWayList, WxErrorException>;

    /// 更新企业已配置的「联系我」方式（对应 Java
    /// `WxCpExternalContactService.updateContactWay(WxCpContactWayInfo)`）。
    async fn update_contact_way(
        &self,
        info: &WxCpContactWayInfo,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 删除企业已配置的「联系我」方式（对应 Java
    /// `WxCpExternalContactService.deleteContactWay(String)`）。
    async fn delete_contact_way(&self, config_id: &str) -> Result<WxCpBaseResp, WxErrorException>;

    /// 结束临时会话（对应 Java
    /// `WxCpExternalContactService.closeTempChat(String, String)`）。
    async fn close_temp_chat(
        &self,
        user_id: &str,
        external_user_id: &str,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取外部联系人详情（对应 Java
    /// `WxCpExternalContactService.getExternalContact(String)`，Java 中已
    /// `@Deprecated`，推荐使用 `get_contact_detail`）。
    async fn get_external_contact(
        &self,
        external_user_id: &str,
    ) -> Result<WxCpExternalContactInfo, WxErrorException>;

    /// 获取客户详情（对应 Java
    /// `WxCpExternalContactService.getContactDetail(String, String)`）。
    async fn get_contact_detail(
        &self,
        external_user_id: &str,
        cursor: Option<&str>,
    ) -> Result<WxCpExternalContactInfo, WxErrorException>;

    /// external_userid 转 openid（对应 Java
    /// `WxCpExternalContactService.convertToOpenid(String)`）。
    async fn convert_to_openid(&self, external_userid: &str) -> Result<String, WxErrorException>;

    /// unionid 转 external_userid（对应 Java
    /// `WxCpExternalContactService.unionidToExternalUserid(String, String)`）。
    async fn unionid_to_external_userid(
        &self,
        unionid: &str,
        openid: &str,
    ) -> Result<String, WxErrorException>;

    /// 配置客户群进群方式（对应 Java
    /// `WxCpExternalContactService.addJoinWay(WxCpGroupJoinWayInfo)`）。
    async fn add_join_way(
        &self,
        wx_cp_group_join_way_info: &WxCpGroupJoinWayInfo,
    ) -> Result<WxCpGroupJoinWayResult, WxErrorException>;

    /// 更新客户群进群方式配置（对应 Java
    /// `WxCpExternalContactService.updateJoinWay(WxCpGroupJoinWayInfo)`；
    /// 覆盖式更新）。
    async fn update_join_way(
        &self,
        wx_cp_group_join_way_info: &WxCpGroupJoinWayInfo,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取客户群进群方式配置（对应 Java
    /// `WxCpExternalContactService.getJoinWay(String)`）。
    async fn get_join_way(&self, config_id: &str)
    -> Result<WxCpGroupJoinWayInfo, WxErrorException>;

    /// 删除客户群进群方式配置（对应 Java
    /// `WxCpExternalContactService.delJoinWay(String)`）。
    async fn del_join_way(&self, config_id: &str) -> Result<WxCpBaseResp, WxErrorException>;

    /// 代开发应用 external_userid 转换（对应 Java
    /// `WxCpExternalContactService.toServiceExternalUserid(String)`）。
    async fn to_service_external_userid(
        &self,
        external_userid: &str,
    ) -> Result<String, WxErrorException>;

    /// 将服务商 external_userid 转换成自建应用的 external_userid（对应 Java
    /// `WxCpExternalContactService.fromServiceExternalUserid(String, String)`）。
    async fn from_service_external_userid(
        &self,
        external_userid: &str,
        source_agent_id: &str,
    ) -> Result<String, WxErrorException>;

    /// 企业客户微信 unionid 的升级——unionid 查询 external_userid（对应 Java
    /// `WxCpExternalContactService.unionidToExternalUserid3rd(String, String,
    /// String)`；`corpid` 不填则拉取所有企业）。
    async fn unionid_to_external_userid_3rd(
        &self,
        unionid: &str,
        openid: &str,
        corpid: Option<&str>,
    ) -> Result<WxCpExternalUserIdList, WxErrorException>;

    /// 转换 external_userid（对应 Java
    /// `WxCpExternalContactService.getNewExternalUserId(String[])`）。
    async fn get_new_external_user_id(
        &self,
        external_user_id_list: &[&str],
    ) -> Result<WxCpNewExternalUserIdList, WxErrorException>;

    /// 设置迁移完成（对应 Java
    /// `WxCpExternalContactService.finishExternalUserIdMigration(String)`）。
    async fn finish_external_user_id_migration(
        &self,
        corpid: &str,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 客户群 opengid 转换（对应 Java
    /// `WxCpExternalContactService.opengidToChatid(String)`）。
    async fn opengid_to_chatid(&self, opengid: &str) -> Result<String, WxErrorException>;

    /// 批量获取客户详情（对应 Java
    /// `WxCpExternalContactService.getContactDetailBatch(String[], String,
    /// Integer)`）。
    async fn get_contact_detail_batch(
        &self,
        user_id_list: &[&str],
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpExternalContactBatchInfo, WxErrorException>;

    /// 获取已服务的外部联系人（对应 Java
    /// `WxCpExternalContactService.getContactList(String, Integer)`）。
    async fn get_contact_list(
        &self,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpExternalContactListInfo, WxErrorException>;

    /// 修改客户备注信息（对应 Java
    /// `WxCpExternalContactService.updateRemark(WxCpUpdateRemarkRequest)`）。
    async fn update_remark(
        &self,
        request: &WxCpUpdateRemarkRequest,
    ) -> Result<(), WxErrorException>;

    /// 获取客户列表（对应 Java
    /// `WxCpExternalContactService.listExternalContacts(String)`）。
    async fn list_external_contacts(&self, user_id: &str) -> Result<Vec<String>, WxErrorException>;

    /// 获取配置了客户联系功能的成员列表（对应 Java
    /// `WxCpExternalContactService.listFollowers()`）。
    async fn list_followers(&self) -> Result<Vec<String>, WxErrorException>;

    /// 获取待分配的离职成员列表（对应 Java
    /// `WxCpExternalContactService.listUnassignedList(Integer, String,
    /// Integer)`）。
    async fn list_unassigned_list(
        &self,
        page_id: Option<i32>,
        cursor: Option<&str>,
        page_size: Option<i32>,
    ) -> Result<WxCpUserExternalUnassignList, WxErrorException>;

    /// 分配离职成员的外部联系人（对应 Java
    /// `WxCpExternalContactService.transferExternalContact(String, String,
    /// String)`，Java 中已 `@Deprecated`，推荐使用 `transfer_customer`）。
    async fn transfer_external_contact(
        &self,
        external_userid: &str,
        hand_over_userid: &str,
        take_over_userid: &str,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 转接在职成员的客户给其他成员（对应 Java
    /// `WxCpExternalContactService.transferCustomer(WxCpUserTransferCustomerReq)`）。
    async fn transfer_customer(
        &self,
        req: &WxCpUserTransferCustomerReq,
    ) -> Result<WxCpUserTransferCustomerResp, WxErrorException>;

    /// 查询在职成员的客户转接情况（对应 Java
    /// `WxCpExternalContactService.transferResult(String, String, String)`）。
    async fn transfer_result(
        &self,
        hand_over_userid: &str,
        take_over_userid: &str,
        cursor: Option<&str>,
    ) -> Result<WxCpUserTransferResultResp, WxErrorException>;

    /// 分配离职成员的客户给其他成员（对应 Java
    /// `WxCpExternalContactService.resignedTransferCustomer(WxCpUserTransferCustomerReq)`）。
    async fn resigned_transfer_customer(
        &self,
        req: &WxCpUserTransferCustomerReq,
    ) -> Result<WxCpUserTransferCustomerResp, WxErrorException>;

    /// 查询离职成员的客户分配情况（对应 Java
    /// `WxCpExternalContactService.resignedTransferResult(String, String,
    /// String)`）。
    async fn resigned_transfer_result(
        &self,
        hand_over_userid: &str,
        take_over_userid: &str,
        cursor: Option<&str>,
    ) -> Result<WxCpUserTransferResultResp, WxErrorException>;

    /// 获取配置过客户群管理的客户群列表（旧分页版，对应 Java
    /// `WxCpExternalContactService.listGroupChat(Integer, Integer, int,
    /// String[], String[])`，Java 中已 `@Deprecated`，推荐使用
    /// `list_group_chat`）。
    async fn list_group_chat_with_page_index(
        &self,
        page_index: Option<i32>,
        page_size: Option<i32>,
        status: i32,
        user_ids: &[&str],
        party_ids: &[&str],
    ) -> Result<WxCpUserExternalGroupChatList, WxErrorException>;

    /// 获取配置过客户群管理的客户群列表（对应 Java
    /// `WxCpExternalContactService.listGroupChat(Integer, String, int,
    /// String[])`）。
    async fn list_group_chat(
        &self,
        limit: Option<i32>,
        cursor: Option<&str>,
        status: i32,
        user_ids: Option<&[&str]>,
    ) -> Result<WxCpUserExternalGroupChatList, WxErrorException>;

    /// 通过客户群 ID 获取详情（对应 Java
    /// `WxCpExternalContactService.getGroupChat(String, Integer)`）。
    async fn get_group_chat(
        &self,
        chat_id: &str,
        need_name: Option<i32>,
    ) -> Result<WxCpUserExternalGroupChatInfo, WxErrorException>;

    /// 将已离职成员为群主的群分配给另一个客服成员（对应 Java
    /// `WxCpExternalContactService.transferGroupChat(String[], String)`）。
    async fn transfer_group_chat(
        &self,
        chat_ids: &[&str],
        new_owner: &str,
    ) -> Result<WxCpUserExternalGroupChatTransferResp, WxErrorException>;

    /// 将在职成员为群主的群分配给另一个客服成员（对应 Java
    /// `WxCpExternalContactService.onjobTransferGroupChat(String[], String)`）。
    async fn onjob_transfer_group_chat(
        &self,
        chat_ids: &[&str],
        new_owner: &str,
    ) -> Result<WxCpUserExternalGroupChatTransferResp, WxErrorException>;

    /// 获取成员联系客户的数据（对应 Java
    /// `WxCpExternalContactService.getUserBehaviorStatistic(Date, Date,
    /// String[], String[])`；Java `Date` 以 `chrono::DateTime<Utc>`
    /// 表达，ADAPTED）。
    async fn get_user_behavior_statistic(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        user_ids: &[&str],
        party_ids: &[&str],
    ) -> Result<WxCpUserExternalUserBehaviorStatistic, WxErrorException>;

    /// 获取指定日期全天的统计数据（对应 Java
    /// `WxCpExternalContactService.getGroupChatStatistic(Date, Integer,
    /// Integer, Integer, Integer, String[], String[])`；Java `Date` 以
    /// `chrono::DateTime<Utc>` 表达，ADAPTED）。
    async fn get_group_chat_statistic(
        &self,
        start_time: DateTime<Utc>,
        order_by: i32,
        order_asc: i32,
        page_index: i32,
        page_size: i32,
        user_ids: &[&str],
        party_ids: &[&str],
    ) -> Result<WxCpUserExternalGroupChatStatistic, WxErrorException>;

    /// 添加企业群发消息任务（对应 Java
    /// `WxCpExternalContactService.addMsgTemplate(WxCpMsgTemplate)`）。
    async fn add_msg_template(
        &self,
        wx_cp_msg_template: &WxCpMsgTemplate,
    ) -> Result<WxCpMsgTemplateAddResult, WxErrorException>;

    /// 提醒成员群发（对应 Java
    /// `WxCpExternalContactService.remindGroupMsgSend(String)`）。
    async fn remind_group_msg_send(&self, msg_id: &str) -> Result<WxCpBaseResp, WxErrorException>;

    /// 停止企业群发（对应 Java
    /// `WxCpExternalContactService.cancelGroupMsgSend(String)`）。
    async fn cancel_group_msg_send(&self, msg_id: &str) -> Result<WxCpBaseResp, WxErrorException>;

    /// 发送新客户欢迎语（对应 Java
    /// `WxCpExternalContactService.sendWelcomeMsg(WxCpWelcomeMsg)`）。
    async fn send_welcome_msg(&self, msg: &WxCpWelcomeMsg) -> Result<(), WxErrorException>;

    /// 获取企业客户标签详情（对应 Java
    /// `WxCpExternalContactService.getCorpTagList(String[])`）。
    async fn get_corp_tag_list(
        &self,
        tag_id: &[&str],
    ) -> Result<WxCpUserExternalTagGroupList, WxErrorException>;

    /// 获取企业客户标签详情（对应 Java
    /// `WxCpExternalContactService.getCorpTagList(String[], String[])`；
    /// `tagId`/`groupId` 均为空时返回所有标签，同时传时忽略 `tagId`）。
    async fn get_corp_tag_list_with_group_id(
        &self,
        tag_id: &[&str],
        group_id: &[&str],
    ) -> Result<WxCpUserExternalTagGroupList, WxErrorException>;

    /// 添加企业客户标签（对应 Java
    /// `WxCpExternalContactService.addCorpTag(WxCpUserExternalTagGroupInfo)`）。
    async fn add_corp_tag(
        &self,
        tag_group: &WxCpUserExternalTagGroupInfo,
    ) -> Result<WxCpUserExternalTagGroupInfo, WxErrorException>;

    /// 编辑客户标签/标签组（对应 Java
    /// `WxCpExternalContactService.editCorpTag(String, String, Integer)`）。
    async fn edit_corp_tag(
        &self,
        id: &str,
        name: Option<&str>,
        order: Option<i32>,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 删除客户标签库中的标签或标签组（对应 Java
    /// `WxCpExternalContactService.delCorpTag(String[], String[])`）。
    async fn del_corp_tag(
        &self,
        tag_id: &[&str],
        group_id: &[&str],
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 为指定成员的客户添加企业统一配置的标签（对应 Java
    /// `WxCpExternalContactService.markTag(String, String, String[],
    /// String[])`）。
    async fn mark_tag(
        &self,
        userid: &str,
        external_userid: &str,
        add_tag: &[&str],
        remove_tag: &[&str],
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 创建客户朋友圈的发表任务（对应 Java
    /// `WxCpExternalContactService.addMomentTask(WxCpAddMomentTask)`）。
    async fn add_moment_task(
        &self,
        task: &WxCpAddMomentTask,
    ) -> Result<WxCpAddMomentResult, WxErrorException>;

    /// 获取发表任务创建结果（对应 Java
    /// `WxCpExternalContactService.getMomentTaskResult(String)`）。
    async fn get_moment_task_result(
        &self,
        job_id: &str,
    ) -> Result<WxCpGetMomentTaskResult, WxErrorException>;

    /// 停止发表企业朋友圈（对应 Java
    /// `WxCpExternalContactService.cancelMomentTask(String)`）。
    async fn cancel_moment_task(&self, moment_id: &str) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取客户朋友圈全部的发表记录（对应 Java
    /// `WxCpExternalContactService.getMomentList(Long, Long, String,
    /// Integer, String, Integer)`）。
    async fn get_moment_list(
        &self,
        start_time: i64,
        end_time: i64,
        creator: Option<&str>,
        filter_type: Option<i32>,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpGetMomentList, WxErrorException>;

    /// 获取客户朋友圈企业发表的列表（对应 Java
    /// `WxCpExternalContactService.getMomentTask(String, String, Integer)`）。
    async fn get_moment_task(
        &self,
        moment_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpGetMomentTask, WxErrorException>;

    /// 获取客户朋友圈发表时选择的可见范围（对应 Java
    /// `WxCpExternalContactService.getMomentCustomerList(String, String,
    /// String, Integer)`）。
    async fn get_moment_customer_list(
        &self,
        moment_id: &str,
        user_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpGetMomentCustomerList, WxErrorException>;

    /// 获取客户朋友圈发表后的可见客户列表（对应 Java
    /// `WxCpExternalContactService.getMomentSendResult(String, String,
    /// String, Integer)`）。
    async fn get_moment_send_result(
        &self,
        moment_id: &str,
        user_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpGetMomentSendResult, WxErrorException>;

    /// 获取客户朋友圈的互动数据（对应 Java
    /// `WxCpExternalContactService.getMomentComments(String, String)`）。
    async fn get_moment_comments(
        &self,
        moment_id: &str,
        user_id: &str,
    ) -> Result<WxCpGetMomentComments, WxErrorException>;

    /// 获取企业与成员的群发记录（对应 Java
    /// `WxCpExternalContactService.getGroupMsgListV2(String, Date, Date,
    /// String, Integer, Integer, String)`；Java `Date` 以
    /// `chrono::DateTime<Utc>` 表达，ADAPTED）。
    async fn get_group_msg_list_v2(
        &self,
        chat_type: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        creator: Option<&str>,
        filter_type: Option<i32>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpGroupMsgListResult, WxErrorException>;

    /// 获取群发成员发送任务列表（对应 Java
    /// `WxCpExternalContactService.getGroupMsgSendResult(String, String,
    /// Integer, String)`）。
    async fn get_group_msg_send_result(
        &self,
        msgid: &str,
        userid: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpGroupMsgSendResult, WxErrorException>;

    /// 获取创建企业群发的群发发送结果（对应 Java
    /// `WxCpExternalContactService.getGroupMsgResult(String, Integer,
    /// String)`）。
    async fn get_group_msg_result(
        &self,
        msgid: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpGroupMsgResult, WxErrorException>;

    /// 获取群发成员发送任务列表（对应 Java
    /// `WxCpExternalContactService.getGroupMsgTask(String, Integer,
    /// String)`）。
    async fn get_group_msg_task(
        &self,
        msgid: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpGroupMsgTaskResult, WxErrorException>;

    /// 添加入群欢迎语素材（对应 Java
    /// `WxCpExternalContactService.addGroupWelcomeTemplate(WxCpGroupWelcomeTemplateResult)`，
    /// 返回欢迎语素材 id）。
    async fn add_group_welcome_template(
        &self,
        template: &WxCpGroupWelcomeTemplateResult,
    ) -> Result<String, WxErrorException>;

    /// 编辑入群欢迎语素材（对应 Java
    /// `WxCpExternalContactService.editGroupWelcomeTemplate(WxCpGroupWelcomeTemplateResult)`）。
    async fn edit_group_welcome_template(
        &self,
        template: &WxCpGroupWelcomeTemplateResult,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取入群欢迎语素材（对应 Java
    /// `WxCpExternalContactService.getGroupWelcomeTemplate(String)`）。
    async fn get_group_welcome_template(
        &self,
        template_id: &str,
    ) -> Result<WxCpGroupWelcomeTemplateResult, WxErrorException>;

    /// 删除入群欢迎语素材（对应 Java
    /// `WxCpExternalContactService.delGroupWelcomeTemplate(String, String)`）。
    async fn del_group_welcome_template(
        &self,
        template_id: &str,
        agent_id: Option<&str>,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取商品图册列表（对应 Java
    /// `WxCpExternalContactService.getProductAlbumList(Integer, String)`）。
    async fn get_product_album_list(
        &self,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpProductAlbumListResult, WxErrorException>;

    /// 获取商品图册（对应 Java
    /// `WxCpExternalContactService.getProductAlbum(String)`）。
    async fn get_product_album(
        &self,
        product_id: &str,
    ) -> Result<WxCpProductAlbumResult, WxErrorException>;

    /// 上传附件资源（对应 Java
    /// `WxCpExternalContactService.uploadAttachment(String, String, Integer,
    /// InputStream)`；Java `InputStream` 以 `Vec<u8>` 表达，ADAPTED）。
    async fn upload_attachment(
        &self,
        media_type: &str,
        file_type: &str,
        attachment_type: i32,
        input: Vec<u8>,
    ) -> Result<WxMediaUploadResult, WxErrorException>;

    /// 上传附件资源（对应 Java
    /// `WxCpExternalContactService.uploadAttachment(String, Integer, File)`；
    /// Java `File` 以文件路径 `&str` 表达，ADAPTED）。
    async fn upload_attachment_with_file(
        &self,
        media_type: &str,
        attachment_type: i32,
        file_path: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException>;

    /// 新建敏感词规则（对应 Java
    /// `WxCpExternalContactService.addInterceptRule(WxCpInterceptRuleAddRequest)`，
    /// 返回规则 id）。
    async fn add_intercept_rule(
        &self,
        rule_add_request: &WxCpInterceptRuleAddRequest,
    ) -> Result<String, WxErrorException>;

    /// 修改敏感词规则（对应 Java
    /// `WxCpExternalContactService.updateInterceptRule(WxCpInterceptRule)`）。
    async fn update_intercept_rule(
        &self,
        intercept_rule: &WxCpInterceptRule,
    ) -> Result<(), WxErrorException>;

    /// 删除敏感词规则（对应 Java
    /// `WxCpExternalContactService.delInterceptRule(String)`）。
    async fn del_intercept_rule(&self, rule_id: &str) -> Result<(), WxErrorException>;

    /// 获取敏感词规则列表（对应 Java
    /// `WxCpExternalContactService.getInterceptRuleList()`）。
    async fn get_intercept_rule_list(&self) -> Result<WxCpInterceptRuleList, WxErrorException>;

    /// 获取敏感词详情（对应 Java
    /// `WxCpExternalContactService.getInterceptRuleDetail(String)`）。
    async fn get_intercept_rule_detail(
        &self,
        rule_id: &str,
    ) -> Result<WxCpInterceptRuleInfo, WxErrorException>;

    /// 创建商品图册（对应 Java
    /// `WxCpExternalContactService.addProductAlbum(WxCpProductAlbumInfo)`，
    /// 返回商品 id）。
    async fn add_product_album(
        &self,
        wx_cp_product_album_info: &WxCpProductAlbumInfo,
    ) -> Result<String, WxErrorException>;

    /// 编辑商品图册（对应 Java
    /// `WxCpExternalContactService.updateProductAlbum(WxCpProductAlbumInfo)`）。
    async fn update_product_album(
        &self,
        wx_cp_product_album_info: &WxCpProductAlbumInfo,
    ) -> Result<(), WxErrorException>;

    /// 删除商品图册（对应 Java
    /// `WxCpExternalContactService.deleteProductAlbum(String)`）。
    async fn delete_product_album(&self, product_id: &str) -> Result<(), WxErrorException>;

    /// 获取获客链接列表（对应 Java
    /// `WxCpExternalContactService.customerAcquisitionLinkList(Integer,
    /// String)`）。
    async fn customer_acquisition_link_list(
        &self,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpCustomerAcquisitionList, WxErrorException>;

    /// 获取获客链接详情（对应 Java
    /// `WxCpExternalContactService.customerAcquisitionLinkGet(String)`）。
    async fn customer_acquisition_link_get(
        &self,
        link_id: &str,
    ) -> Result<WxCpCustomerAcquisitionInfo, WxErrorException>;

    /// 创建获客链接（对应 Java
    /// `WxCpExternalContactService.customerAcquisitionLinkCreate(WxCpCustomerAcquisitionRequest)`）。
    async fn customer_acquisition_link_create(
        &self,
        request: &WxCpCustomerAcquisitionRequest,
    ) -> Result<WxCpCustomerAcquisitionCreateResult, WxErrorException>;

    /// 编辑获客链接（对应 Java
    /// `WxCpExternalContactService.customerAcquisitionUpdate(WxCpCustomerAcquisitionRequest)`）。
    async fn customer_acquisition_update(
        &self,
        request: &WxCpCustomerAcquisitionRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 删除获客链接（对应 Java
    /// `WxCpExternalContactService.customerAcquisitionLinkDelete(String)`）。
    async fn customer_acquisition_link_delete(
        &self,
        link_id: &str,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取获客客户列表（对应 Java
    /// `WxCpExternalContactService.customerAcquisitionCustomer(String,
    /// Integer, String)`）。
    async fn customer_acquisition_customer(
        &self,
        link_id: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpCustomerAcquisitionCustomerList, WxErrorException>;

    /// 查询剩余使用量（对应 Java
    /// `WxCpExternalContactService.customerAcquisitionQuota()`）。
    async fn customer_acquisition_quota(
        &self,
    ) -> Result<WxCpCustomerAcquisitionQuota, WxErrorException>;

    /// 查询链接使用详情（对应 Java
    /// `WxCpExternalContactService.customerAcquisitionStatistic(String,
    /// Date, Date)`；Java `Date` 以 `chrono::DateTime<Utc>` 表达，ADAPTED）。
    async fn customer_acquisition_statistic(
        &self,
        link_id: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<WxCpCustomerAcquisitionStatistic, WxErrorException>;
}
