//! 微信客服服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpKfService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpBaseResp, WxCpKfAccountAdd, WxCpKfAccountAddResp, WxCpKfAccountDel, WxCpKfAccountLink,
    WxCpKfAccountLinkResp, WxCpKfAccountListResp, WxCpKfAccountUpd, WxCpKfCustomerBatchGetResp,
    WxCpKfGetCorpStatisticRequest, WxCpKfGetCorpStatisticResp, WxCpKfGetServicerStatisticRequest,
    WxCpKfGetServicerStatisticResp, WxCpKfMsgListResp, WxCpKfMsgSendRequest, WxCpKfMsgSendResp,
    WxCpKfServiceStateResp, WxCpKfServiceStateTransResp, WxCpKfServiceUpgradeConfigResp,
    WxCpKfServicerListResp, WxCpKfServicerOpResp,
};

/// 微信客服服务。
#[async_trait]
pub trait WxCpKfService: Send + Sync {
    /// 添加客服帐号（对应 Java `WxCpKfService.addAccount(WxCpKfAccountAdd)`，
    /// 返回新创建的客服帐号 ID）。
    async fn add_account(
        &self,
        add: &WxCpKfAccountAdd,
    ) -> Result<WxCpKfAccountAddResp, WxErrorException>;

    /// 修改已有的客服帐号（对应 Java
    /// `WxCpKfService.updAccount(WxCpKfAccountUpd)`）。
    async fn upd_account(&self, upd: &WxCpKfAccountUpd) -> Result<WxCpBaseResp, WxErrorException>;

    /// 删除已有的客服帐号（对应 Java
    /// `WxCpKfService.delAccount(WxCpKfAccountDel)`）。
    async fn del_account(&self, del: &WxCpKfAccountDel) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取客服帐号列表（对应 Java
    /// `WxCpKfService.listAccount(Integer, Integer)`）。
    async fn list_account(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<WxCpKfAccountListResp, WxErrorException>;

    /// 获取客服链接（对应 Java
    /// `WxCpKfService.getAccountLink(WxCpKfAccountLink)`）。
    async fn get_account_link(
        &self,
        link: &WxCpKfAccountLink,
    ) -> Result<WxCpKfAccountLinkResp, WxErrorException>;

    /// 添加接待人员（对应 Java
    /// `WxCpKfService.addServicer(String, List<String>)`）。
    async fn add_servicer(
        &self,
        open_kfid: &str,
        user_id_list: &[&str],
    ) -> Result<WxCpKfServicerOpResp, WxErrorException>;

    /// 添加接待人员（含部门，对应 Java
    /// `WxCpKfService.addServicer(String, List<String>, List<String>)`；
    /// `user_id_list` 与 `department_id_list` 至少填一个）。
    async fn add_servicer_with_departments(
        &self,
        open_kfid: &str,
        user_id_list: &[&str],
        department_id_list: &[&str],
    ) -> Result<WxCpKfServicerOpResp, WxErrorException>;

    /// 删除接待人员（对应 Java
    /// `WxCpKfService.delServicer(String, List<String>)`）。
    async fn del_servicer(
        &self,
        open_kfid: &str,
        user_id_list: &[&str],
    ) -> Result<WxCpKfServicerOpResp, WxErrorException>;

    /// 删除接待人员（含部门，对应 Java
    /// `WxCpKfService.delServicer(String, List<String>, List<String>)`）。
    async fn del_servicer_with_departments(
        &self,
        open_kfid: &str,
        user_id_list: &[&str],
        department_id_list: &[&str],
    ) -> Result<WxCpKfServicerOpResp, WxErrorException>;

    /// 获取某个客服帐号的接待人员列表（对应 Java
    /// `WxCpKfService.listServicer(String)`）。
    async fn list_servicer(
        &self,
        open_kfid: &str,
    ) -> Result<WxCpKfServicerListResp, WxErrorException>;

    /// 获取会话状态（对应 Java
    /// `WxCpKfService.getServiceState(String, String)`）。
    async fn get_service_state(
        &self,
        open_kfid: &str,
        external_user_id: &str,
    ) -> Result<WxCpKfServiceStateResp, WxErrorException>;

    /// 变更会话状态（对应 Java
    /// `WxCpKfService.transServiceState(String, String, Integer, String)`）。
    async fn trans_service_state(
        &self,
        open_kfid: &str,
        external_user_id: &str,
        service_state: i32,
        servicer_user_id: Option<&str>,
    ) -> Result<WxCpKfServiceStateTransResp, WxErrorException>;

    /// 读取消息（对应 Java
    /// `WxCpKfService.syncMsg(String, String, Integer, Integer)`，
    /// Java 中已 `@Deprecated`，推荐使用 `sync_msg_with_open_kfid`）。
    async fn sync_msg(
        &self,
        cursor: Option<&str>,
        token: Option<&str>,
        limit: Option<i32>,
        voice_format: Option<i32>,
    ) -> Result<WxCpKfMsgListResp, WxErrorException>;

    /// 读取消息（指定客服帐号，对应 Java
    /// `WxCpKfService.syncMsg(String, String, Integer, Integer, String)`）。
    async fn sync_msg_with_open_kfid(
        &self,
        cursor: Option<&str>,
        token: Option<&str>,
        limit: Option<i32>,
        voice_format: Option<i32>,
        open_kfid: &str,
    ) -> Result<WxCpKfMsgListResp, WxErrorException>;

    /// 发送消息（对应 Java `WxCpKfService.sendMsg(WxCpKfMsgSendRequest)`）。
    async fn send_msg(
        &self,
        request: &WxCpKfMsgSendRequest,
    ) -> Result<WxCpKfMsgSendResp, WxErrorException>;

    /// 发送欢迎语等事件响应消息（对应 Java
    /// `WxCpKfService.sendMsgOnEvent(WxCpKfMsgSendRequest)`）。
    async fn send_msg_on_event(
        &self,
        request: &WxCpKfMsgSendRequest,
    ) -> Result<WxCpKfMsgSendResp, WxErrorException>;

    /// 获取客户基础信息（对应 Java
    /// `WxCpKfService.customerBatchGet(List<String>)`）。
    async fn customer_batch_get(
        &self,
        external_user_id_list: &[&str],
    ) -> Result<WxCpKfCustomerBatchGetResp, WxErrorException>;

    /// 获取「客户数据统计」企业汇总数据（对应 Java
    /// `WxCpKfService.getCorpStatistic(WxCpKfGetCorpStatisticRequest)`；
    /// 依赖 C2b 手写 bean `WxCpKfGetCorpStatisticResp`）。
    async fn get_corp_statistic(
        &self,
        request: &WxCpKfGetCorpStatisticRequest,
    ) -> Result<WxCpKfGetCorpStatisticResp, WxErrorException>;

    /// 获取「客户数据统计」接待人员明细数据（对应 Java
    /// `WxCpKfService.getServicerStatistic(WxCpKfGetServicerStatisticRequest)`）。
    async fn get_servicer_statistic(
        &self,
        request: &WxCpKfGetServicerStatisticRequest,
    ) -> Result<WxCpKfGetServicerStatisticResp, WxErrorException>;

    /// 获取配置的专员与客户群（对应 Java
    /// `WxCpKfService.getUpgradeServiceConfig()`）。
    async fn get_upgrade_service_config(
        &self,
    ) -> Result<WxCpKfServiceUpgradeConfigResp, WxErrorException>;

    /// 升级专员服务（对应 Java
    /// `WxCpKfService.upgradeMemberService(String, String, String, String)`）。
    async fn upgrade_member_service(
        &self,
        open_kfid: &str,
        external_user_id: &str,
        userid: &str,
        wording: Option<&str>,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 升级客户群服务（对应 Java
    /// `WxCpKfService.upgradeGroupchatService(String, String, String, String)`）。
    async fn upgrade_groupchat_service(
        &self,
        open_kfid: &str,
        external_user_id: &str,
        chat_id: &str,
        wording: Option<&str>,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 为客户取消推荐（对应 Java
    /// `WxCpKfService.cancelUpgradeService(String, String)`）。
    async fn cancel_upgrade_service(
        &self,
        open_kfid: &str,
        external_user_id: &str,
    ) -> Result<WxCpBaseResp, WxErrorException>;
}
