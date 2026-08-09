//! 企业微信第三方应用成员服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpUserService`：
//! 用户管理接口（二次验证/部门成员列表/创建/更新/删除/获取/邀请/
//! userid↔openid 转换/外部联系人详情），多数方法需传授权企业 corpId
//! 使用对应的 access_token。

use std::collections::HashMap;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpInviteResult, WxCpUser, WxCpUserExternalContactInfo};

/// 企业微信第三方应用成员服务。
#[async_trait]
pub trait WxCpTpUserService: Send + Sync {
    /// 用在二次验证的时候：企业员工验证成功后告诉企业号平台该员工关注
    /// 成功（对应 Java `authenticate(String)`）。
    async fn authenticate(&self, user_id: &str) -> Result<(), WxErrorException>;

    /// 获取部门成员（详情，对应 Java `listByDepartment(Long, Boolean,
    /// Integer, String)`）。
    async fn list_by_department(
        &self,
        depart_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
        corp_id: &str,
    ) -> Result<Vec<WxCpUser>, WxErrorException>;

    /// 获取部门成员（简版，对应 Java `listSimpleByDepartment(Long,
    /// Boolean, Integer, String)`）。
    async fn list_simple_by_department(
        &self,
        depart_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
        corp_id: &str,
    ) -> Result<Vec<WxCpUser>, WxErrorException>;

    /// 获取部门成员（简版，不带 corpId 的 @Deprecated 重载，对应 Java
    /// `listSimpleByDepartment(Long, Boolean, Integer)`）。
    async fn list_simple_by_department_without_corp(
        &self,
        depart_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
    ) -> Result<Vec<WxCpUser>, WxErrorException>;

    /// 新建用户（对应 Java `create(WxCpUser)`）。
    async fn create(&self, user: &WxCpUser) -> Result<(), WxErrorException>;

    /// 更新用户（对应 Java `update(WxCpUser)`）。
    async fn update(&self, user: &WxCpUser) -> Result<(), WxErrorException>;

    /// 删除用户/批量删除成员（对应 Java `delete(String...)`：单用户走
    /// GET 删除，多用户走 POST 批量删除）。
    async fn delete(&self, user_ids: &[String]) -> Result<(), WxErrorException>;

    /// 获取用户（对应 Java `getById(String, String)`）。
    async fn get_by_id(&self, user_id: &str, corp_id: &str) -> Result<WxCpUser, WxErrorException>;

    /// 邀请成员（对应 Java `invite(List, List, List)`：userIds 最多
    /// 1000 个，partyIds 最多 100 个，tagIds 最多 100 个）。
    async fn invite(
        &self,
        user_ids: &[String],
        party_ids: &[String],
        tag_ids: &[String],
    ) -> Result<WxCpInviteResult, WxErrorException>;

    /// userid 转 openid（对应 Java `userId2Openid(String, Integer)`，
    /// 返回 map 可能含 openid/appid）。
    async fn user_id_2_openid(
        &self,
        user_id: &str,
        agent_id: Option<i32>,
    ) -> Result<HashMap<String, String>, WxErrorException>;

    /// openid 转 userid（对应 Java `openid2UserId(String)`）。
    async fn openid_2_user_id(&self, openid: &str) -> Result<String, WxErrorException>;

    /// 通过手机号获取 userid（对应 Java `getUserId(String, String)`）。
    async fn get_user_id(&self, mobile: &str, corp_id: &str) -> Result<String, WxErrorException>;

    /// 获取外部联系人详情（对应 Java `getExternalContact(String)`）。
    async fn get_external_contact(
        &self,
        user_id: &str,
    ) -> Result<WxCpUserExternalContactInfo, WxErrorException>;
}
