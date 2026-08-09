//! 企业微信互联企业服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.corpgroup.service.WxCpLinkedCorpService`：
//! 互联企业相关接口（应用权限/成员/部门），均以
//! `WxCpCorpGroupCorpGetTokenReq` 携带目标企业 corpId/agentId/
//! businessType 并经集团服务执行通道获取对应 access_token。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpCorpGroupCorpGetTokenReq, WxCpLinkedCorpAgentPerm, WxCpLinkedCorpDepartment,
    WxCpLinkedCorpUser,
};

/// 企业微信互联企业服务。
#[async_trait]
pub trait WxCpLinkedCorpService: Send + Sync {
    /// 获取互联企业应用权限（对应 Java
    /// `getLinkedCorpAgentPerm(WxCpCorpGroupCorpGetTokenReq)`）。
    async fn get_linked_corp_agent_perm(
        &self,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<WxCpLinkedCorpAgentPerm, WxErrorException>;

    /// 获取互联企业成员详情（对应 Java
    /// `getLinkedCorpUser(String, WxCpCorpGroupCorpGetTokenReq)`）。
    async fn get_linked_corp_user(
        &self,
        user_id: &str,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<WxCpLinkedCorpUser, WxErrorException>;

    /// 获取互联企业部门成员（简版，对应 Java
    /// `getLinkedCorpSimpleUserList(String, WxCpCorpGroupCorpGetTokenReq)`）。
    async fn get_linked_corp_simple_user_list(
        &self,
        department_id: &str,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<Vec<WxCpLinkedCorpUser>, WxErrorException>;

    /// 获取互联企业部门成员（详情，对应 Java
    /// `getLinkedCorpUserList(String, WxCpCorpGroupCorpGetTokenReq)`）。
    async fn get_linked_corp_user_list(
        &self,
        department_id: &str,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<Vec<WxCpLinkedCorpUser>, WxErrorException>;

    /// 获取互联企业部门列表（对应 Java
    /// `getLinkedCorpDepartmentList(String, WxCpCorpGroupCorpGetTokenReq)`）。
    async fn get_linked_corp_department_list(
        &self,
        department_id: &str,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<Vec<WxCpLinkedCorpDepartment>, WxErrorException>;
}
