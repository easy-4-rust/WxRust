//! 成员管理服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpUserService`。

use std::collections::HashMap;

use async_trait::async_trait;

use chrono::{DateTime, Utc};
use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpDeptUserResult, WxCpExternalContactInfo, WxCpInviteResult, WxCpOpenUseridToUseridResult,
    WxCpUser, WxCpUseridToOpenUseridResult,
};

/// 成员管理服务。
#[async_trait]
pub trait WxCpUserService: Send + Sync {
    /// 二次验证（对应 Java `WxCpUserService.authenticate(String)`）。
    async fn authenticate(&self, user_id: &str) -> Result<(), WxErrorException>;

    /// 获取部门成员详情（对应 Java `WxCpUserService.listByDepartment(Long,
    /// Boolean, Integer)`；`departId` 必填，`fetchChild`/`status` 非必填）。
    async fn list_by_department(
        &self,
        depart_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
    ) -> Result<Vec<WxCpUser>, WxErrorException>;

    /// 获取部门成员（对应 Java `WxCpUserService.listSimpleByDepartment(Long,
    /// Boolean, Integer)`）。
    async fn list_simple_by_department(
        &self,
        depart_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
    ) -> Result<Vec<WxCpUser>, WxErrorException>;

    /// 新建用户（对应 Java `WxCpUserService.create(WxCpUser)`）。
    async fn create(&self, user: &WxCpUser) -> Result<(), WxErrorException>;

    /// 更新用户（对应 Java `WxCpUserService.update(WxCpUser)`）。
    async fn update(&self, user: &WxCpUser) -> Result<(), WxErrorException>;

    /// 获取用户（对应 Java `WxCpUserService.getById(String)`）。
    async fn get_by_id(&self, user_id: &str) -> Result<WxCpUser, WxErrorException>;

    /// 删除用户/批量删除成员（对应 Java
    /// `WxCpUserService.delete(String... userIds)`）。
    async fn delete(&self, user_ids: &[&str]) -> Result<(), WxErrorException>;

    /// 邀请成员（对应 Java `WxCpUserService.invite(List<String>,
    /// List<String>, List<String>)`）。
    async fn invite(
        &self,
        user_ids: &[&str],
        party_ids: &[&str],
        tag_ids: &[&str],
    ) -> Result<WxCpInviteResult, WxErrorException>;

    /// userid 转 openid（对应 Java `WxCpUserService.userId2Openid(String,
    /// Integer)`；`agentId` 非必填）。
    async fn user_id2_openid(
        &self,
        user_id: &str,
        agent_id: Option<i32>,
    ) -> Result<HashMap<String, String>, WxErrorException>;

    /// openid 转 userid（对应 Java `WxCpUserService.openid2UserId(String)`）。
    async fn openid2_user_id(&self, openid: &str) -> Result<String, WxErrorException>;

    /// 通过手机号获取其所对应的 userid（对应 Java
    /// `WxCpUserService.getUserId(String)`）。
    async fn get_user_id(&self, mobile: &str) -> Result<String, WxErrorException>;

    /// 通过邮箱获取其所对应的 userid（对应 Java
    /// `WxCpUserService.getUserIdByEmail(String, int)`；`emailType`：
    /// 1-企业邮箱，2-个人邮箱）。
    async fn get_user_id_by_email(
        &self,
        email: &str,
        email_type: i32,
    ) -> Result<String, WxErrorException>;

    /// 获取外部联系人详情（对应 Java
    /// `WxCpUserService.getExternalContact(String)`）。
    async fn get_external_contact(
        &self,
        user_id: &str,
    ) -> Result<WxCpExternalContactInfo, WxErrorException>;

    /// 获取加入企业二维码（对应 Java
    /// `WxCpUserService.getJoinQrCode(int)`；`sizeType`：1/2/3/4）。
    async fn get_join_qr_code(&self, size_type: i32) -> Result<String, WxErrorException>;

    /// 获取企业活跃成员数（对应 Java
    /// `WxCpUserService.getActiveStat(Date)`；Java `Date` 以
    /// `chrono::DateTime<Utc>` 表达，ADAPTED）。
    async fn get_active_stat(&self, date: DateTime<Utc>) -> Result<i32, WxErrorException>;

    /// userid 转换为 open_userid（对应 Java
    /// `WxCpUserService.useridToOpenUserid(ArrayList<String>)`）。
    async fn userid_to_open_userid(
        &self,
        userid_list: &[&str],
    ) -> Result<WxCpUseridToOpenUseridResult, WxErrorException>;

    /// open_userid 转换为 userid（对应 Java
    /// `WxCpUserService.openUseridToUserid(List<String>, String)`）。
    async fn open_userid_to_userid(
        &self,
        open_userid_list: &[&str],
        source_agent_id: &str,
    ) -> Result<WxCpOpenUseridToUseridResult, WxErrorException>;

    /// 获取成员 ID 列表（对应 Java
    /// `WxCpUserService.getUserListId(String, Integer)`）。
    async fn get_user_list_id(
        &self,
        cursor: &str,
        limit: Option<i32>,
    ) -> Result<WxCpDeptUserResult, WxErrorException>;
}
