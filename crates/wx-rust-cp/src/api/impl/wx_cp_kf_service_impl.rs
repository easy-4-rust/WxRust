//! 微信客服服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpKfServiceImpl`：以
//! `Weak<dyn WxCpService>` 持有门面（Java `@RequiredArgsConstructor` 注入
//! `cpService`），全部方法经门面 `get`/`post` 执行引擎发起请求。
//!
//! 语义镜像要点：
//! - `addServicer`/`delServicer` 的接待人员校验（userid_list 与
//!   department_id_list 至少填一个、数量上限 100/20）对应 Java
//!   `validateParameters` 的 `IllegalArgumentException`，以
//!   `WxErrorException::from_code(-99, ...)` 表达（ADAPTED）；
//! - 请求体 `Option` 字段严格镜像 Java：`if (xx != null)` 分支仅写入
//!   `Some` 值，其余字段按 Java 无条件写入；
//! - `upgradeMemberService`/`upgradeGroupchatService` 的 `type` 字段固定
//!   1/2（对应 Java `json.addProperty("type", 1)`）；
//! - `getAccountLink` 走客服链接接口 `ADD_CONTACT_WAY`（对应 Java 同一
//!   常量名）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpKfService, WxCpService};
use crate::bean::{
    WxCpBaseResp, WxCpKfAccountAdd, WxCpKfAccountAddResp, WxCpKfAccountDel, WxCpKfAccountLink,
    WxCpKfAccountLinkResp, WxCpKfAccountListResp, WxCpKfAccountUpd, WxCpKfCustomerBatchGetResp,
    WxCpKfGetCorpStatisticRequest, WxCpKfGetCorpStatisticResp, WxCpKfGetServicerStatisticRequest,
    WxCpKfGetServicerStatisticResp, WxCpKfMsgListResp, WxCpKfMsgSendRequest, WxCpKfMsgSendResp,
    WxCpKfServiceStateResp, WxCpKfServiceStateTransResp, WxCpKfServiceUpgradeConfigResp,
    WxCpKfServicerListResp, WxCpKfServicerOpResp,
};
use crate::enums::url_kf;

/// 序列化 JSON 对象为请求体字符串（`serde_json::Map` 无 `Display`，以
/// `Value::Object` 包装后序列化，对应 Java `JsonObject.toString()`）。
fn map_to_string(obj: &serde_json::Map<String, serde_json::Value>) -> String {
    serde_json::Value::Object(obj.clone()).to_string()
}

/// 微信客服服务实现。
pub struct WxCpKfServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpKfServiceImpl {
    /// 构建微信客服服务（对应 Java 构造器注入 `WxCpService`）。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 升级门面引用（对应 Java 直接持有的 `cpService` 字段；Weak 引用
    /// 失效时抛 -99，ADAPTED）。
    fn service(&self) -> Result<Arc<dyn WxCpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))
    }

    /// 序列化请求对象（对应 Java `WxCpGsonBuilder.toJson`）。
    fn to_json<T: serde::Serialize>(value: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(value).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 接待人员参数校验（对应 Java `validateParameters`）。
    ///
    /// `is_del`：true 为删除场景（department_id_list 上限 100），false 为
    /// 添加场景（department_id_list 上限 20）。
    fn validate_servicer_params(
        is_del: bool,
        user_id_list: &[&str],
        department_id_list: &[&str],
    ) -> Result<(), WxErrorException> {
        if user_id_list.is_empty() && department_id_list.is_empty() {
            return Err(WxErrorException::from_code(
                -99,
                "userid_list和department_id_list至少需要填其中一个",
            ));
        }
        if user_id_list.len() > 100 {
            return Err(WxErrorException::from_code(
                -99,
                "可填充个数：0 ~ 100。超过100个需分批调用。",
            ));
        }
        if is_del {
            if department_id_list.len() > 100 {
                return Err(WxErrorException::from_code(
                    -99,
                    "可填充个数：0 ~ 100。超过100个需分批调用。",
                ));
            }
        } else if department_id_list.len() > 20 {
            return Err(WxErrorException::from_code(-99, "可填充个数：0 ~ 20。"));
        }
        Ok(())
    }

    /// 添加/删除接待人员公共实现（对应 Java `servicerOp`）。
    async fn servicer_op(
        &self,
        open_kfid: &str,
        user_id_list: &[&str],
        department_id_list: &[&str],
        uri: &str,
    ) -> Result<WxCpKfServicerOpResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "open_kfid".to_string(),
            serde_json::Value::String(open_kfid.to_string()),
        );
        if !user_id_list.is_empty() {
            let arr: Vec<serde_json::Value> = user_id_list
                .iter()
                .map(|v| serde_json::Value::String((*v).to_string()))
                .collect();
            obj.insert("userid_list".to_string(), serde_json::Value::Array(arr));
        }
        if !department_id_list.is_empty() {
            let arr: Vec<serde_json::Value> = department_id_list
                .iter()
                .map(|v| serde_json::Value::String((*v).to_string()))
                .collect();
            obj.insert(
                "department_id_list".to_string(),
                serde_json::Value::Array(arr),
            );
        }
        let url = svc.wx_cp_config_storage().api_url(uri);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpKfServicerOpResp::from_json(&response).map_err(WxErrorException::Serde)
    }
}

#[async_trait]
impl WxCpKfService for WxCpKfServiceImpl {
    /// 添加客服帐号（对应 Java `addAccount`）。
    async fn add_account(
        &self,
        add: &WxCpKfAccountAdd,
    ) -> Result<WxCpKfAccountAddResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(url_kf::ACCOUNT_ADD);
        let response = svc.post(&url, &Self::to_json(add)?).await?;
        WxCpKfAccountAddResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 修改已有的客服帐号（对应 Java `updAccount`）。
    async fn upd_account(&self, upd: &WxCpKfAccountUpd) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(url_kf::ACCOUNT_UPD);
        let response = svc.post(&url, &Self::to_json(upd)?).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 删除已有的客服帐号（对应 Java `delAccount`）。
    async fn del_account(&self, del: &WxCpKfAccountDel) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(url_kf::ACCOUNT_DEL);
        let response = svc.post(&url, &Self::to_json(del)?).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取客服帐号列表（对应 Java `listAccount`）。
    async fn list_account(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<WxCpKfAccountListResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        if let Some(offset) = offset {
            obj.insert("offset".to_string(), serde_json::Value::from(offset));
        }
        if let Some(limit) = limit {
            obj.insert("limit".to_string(), serde_json::Value::from(limit));
        }
        let url = svc.wx_cp_config_storage().api_url(url_kf::ACCOUNT_LIST);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpKfAccountListResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取客服链接（对应 Java `getAccountLink`）。
    async fn get_account_link(
        &self,
        link: &WxCpKfAccountLink,
    ) -> Result<WxCpKfAccountLinkResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(url_kf::ADD_CONTACT_WAY);
        let response = svc.post(&url, &Self::to_json(link)?).await?;
        WxCpKfAccountLinkResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 添加接待人员（对应 Java `addServicer(String, List<String>)`）。
    async fn add_servicer(
        &self,
        open_kfid: &str,
        user_id_list: &[&str],
    ) -> Result<WxCpKfServicerOpResp, WxErrorException> {
        self.servicer_op(open_kfid, user_id_list, &[], url_kf::SERVICER_ADD)
            .await
    }

    /// 添加接待人员（含部门，对应 Java
    /// `addServicer(String, List<String>, List<String>)`）。
    async fn add_servicer_with_departments(
        &self,
        open_kfid: &str,
        user_id_list: &[&str],
        department_id_list: &[&str],
    ) -> Result<WxCpKfServicerOpResp, WxErrorException> {
        Self::validate_servicer_params(false, user_id_list, department_id_list)?;
        self.servicer_op(
            open_kfid,
            user_id_list,
            department_id_list,
            url_kf::SERVICER_ADD,
        )
        .await
    }

    /// 删除接待人员（对应 Java `delServicer(String, List<String>)`）。
    async fn del_servicer(
        &self,
        open_kfid: &str,
        user_id_list: &[&str],
    ) -> Result<WxCpKfServicerOpResp, WxErrorException> {
        self.servicer_op(open_kfid, user_id_list, &[], url_kf::SERVICER_DEL)
            .await
    }

    /// 删除接待人员（含部门，对应 Java
    /// `delServicer(String, List<String>, List<String>)`）。
    async fn del_servicer_with_departments(
        &self,
        open_kfid: &str,
        user_id_list: &[&str],
        department_id_list: &[&str],
    ) -> Result<WxCpKfServicerOpResp, WxErrorException> {
        Self::validate_servicer_params(true, user_id_list, department_id_list)?;
        self.servicer_op(
            open_kfid,
            user_id_list,
            department_id_list,
            url_kf::SERVICER_DEL,
        )
        .await
    }

    /// 获取某个客服帐号的接待人员列表（对应 Java `listServicer`）。
    async fn list_servicer(
        &self,
        open_kfid: &str,
    ) -> Result<WxCpKfServicerListResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(&format!("{}{open_kfid}", url_kf::SERVICER_LIST));
        let response = svc.get(&url, "").await?;
        WxCpKfServicerListResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取会话状态（对应 Java `getServiceState`）。
    async fn get_service_state(
        &self,
        open_kfid: &str,
        external_user_id: &str,
    ) -> Result<WxCpKfServiceStateResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "open_kfid".to_string(),
            serde_json::Value::String(open_kfid.to_string()),
        );
        obj.insert(
            "external_userid".to_string(),
            serde_json::Value::String(external_user_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_kf::SERVICE_STATE_GET);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpKfServiceStateResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 变更会话状态（对应 Java `transServiceState`）。
    async fn trans_service_state(
        &self,
        open_kfid: &str,
        external_user_id: &str,
        service_state: i32,
        servicer_user_id: Option<&str>,
    ) -> Result<WxCpKfServiceStateTransResp, WxErrorException> {
        let svc = self.service()?;
        // Java：四个字段无条件 addProperty（servicer_userid 为 null 写 null）
        let mut obj = serde_json::Map::new();
        obj.insert(
            "open_kfid".to_string(),
            serde_json::Value::String(open_kfid.to_string()),
        );
        obj.insert(
            "external_userid".to_string(),
            serde_json::Value::String(external_user_id.to_string()),
        );
        obj.insert(
            "service_state".to_string(),
            serde_json::Value::from(service_state),
        );
        obj.insert(
            "servicer_userid".to_string(),
            servicer_user_id
                .map(|v| serde_json::Value::String(v.to_string()))
                .unwrap_or(serde_json::Value::Null),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_kf::SERVICE_STATE_TRANS);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpKfServiceStateTransResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 读取消息（对应 Java `syncMsg(String, String, Integer, Integer)`，
    /// `@Deprecated`）。
    async fn sync_msg(
        &self,
        cursor: Option<&str>,
        token: Option<&str>,
        limit: Option<i32>,
        voice_format: Option<i32>,
    ) -> Result<WxCpKfMsgListResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        if let Some(v) = cursor {
            obj.insert(
                "cursor".to_string(),
                serde_json::Value::String(v.to_string()),
            );
        }
        if let Some(v) = token {
            obj.insert(
                "token".to_string(),
                serde_json::Value::String(v.to_string()),
            );
        }
        if let Some(v) = limit {
            obj.insert("limit".to_string(), serde_json::Value::from(v));
        }
        if let Some(v) = voice_format {
            obj.insert("voice_format".to_string(), serde_json::Value::from(v));
        }
        let url = svc.wx_cp_config_storage().api_url(url_kf::SYNC_MSG);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpKfMsgListResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 读取消息（指定客服帐号，对应 Java
    /// `syncMsg(String, String, Integer, Integer, String)`）。
    async fn sync_msg_with_open_kfid(
        &self,
        cursor: Option<&str>,
        token: Option<&str>,
        limit: Option<i32>,
        voice_format: Option<i32>,
        open_kfid: &str,
    ) -> Result<WxCpKfMsgListResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        if let Some(v) = cursor {
            obj.insert(
                "cursor".to_string(),
                serde_json::Value::String(v.to_string()),
            );
        }
        if let Some(v) = token {
            obj.insert(
                "token".to_string(),
                serde_json::Value::String(v.to_string()),
            );
        }
        if let Some(v) = limit {
            obj.insert("limit".to_string(), serde_json::Value::from(v));
        }
        if let Some(v) = voice_format {
            obj.insert("voice_format".to_string(), serde_json::Value::from(v));
        }
        // Java `if (openKfId != null)`；Rust 侧参数非空恒写入
        obj.insert(
            "open_kfid".to_string(),
            serde_json::Value::String(open_kfid.to_string()),
        );
        let url = svc.wx_cp_config_storage().api_url(url_kf::SYNC_MSG);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpKfMsgListResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 发送消息（对应 Java `sendMsg`）。
    async fn send_msg(
        &self,
        request: &WxCpKfMsgSendRequest,
    ) -> Result<WxCpKfMsgSendResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(url_kf::SEND_MSG);
        let response = svc.post(&url, &Self::to_json(request)?).await?;
        WxCpKfMsgSendResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 发送欢迎语等事件响应消息（对应 Java `sendMsgOnEvent`）。
    async fn send_msg_on_event(
        &self,
        request: &WxCpKfMsgSendRequest,
    ) -> Result<WxCpKfMsgSendResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_kf::SEND_MSG_ON_EVENT);
        let response = svc.post(&url, &Self::to_json(request)?).await?;
        WxCpKfMsgSendResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取客户基础信息（对应 Java `customerBatchGet`）。
    async fn customer_batch_get(
        &self,
        external_user_id_list: &[&str],
    ) -> Result<WxCpKfCustomerBatchGetResp, WxErrorException> {
        let svc = self.service()?;
        let arr: Vec<serde_json::Value> = external_user_id_list
            .iter()
            .map(|v| serde_json::Value::String((*v).to_string()))
            .collect();
        let mut obj = serde_json::Map::new();
        obj.insert(
            "external_userid_list".to_string(),
            serde_json::Value::Array(arr),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_kf::CUSTOMER_BATCH_GET);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpKfCustomerBatchGetResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取「客户数据统计」企业汇总数据（对应 Java `getCorpStatistic`）。
    async fn get_corp_statistic(
        &self,
        request: &WxCpKfGetCorpStatisticRequest,
    ) -> Result<WxCpKfGetCorpStatisticResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_kf::GET_CORP_STATISTIC);
        let response = svc.post(&url, &Self::to_json(request)?).await?;
        WxCpKfGetCorpStatisticResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取「客户数据统计」接待人员明细数据（对应 Java `getServicerStatistic`）。
    async fn get_servicer_statistic(
        &self,
        request: &WxCpKfGetServicerStatisticRequest,
    ) -> Result<WxCpKfGetServicerStatisticResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_kf::GET_SERVICER_STATISTIC);
        let response = svc.post(&url, &Self::to_json(request)?).await?;
        WxCpKfGetServicerStatisticResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取配置的专员与客户群（对应 Java `getUpgradeServiceConfig`）。
    async fn get_upgrade_service_config(
        &self,
    ) -> Result<WxCpKfServiceUpgradeConfigResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_kf::CUSTOMER_GET_UPGRADE_SERVICE_CONFIG);
        let response = svc.get(&url, "").await?;
        WxCpKfServiceUpgradeConfigResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 升级专员服务（对应 Java `upgradeMemberService`）。
    async fn upgrade_member_service(
        &self,
        open_kfid: &str,
        external_user_id: &str,
        userid: &str,
        wording: Option<&str>,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "open_kfid".to_string(),
            serde_json::Value::String(open_kfid.to_string()),
        );
        obj.insert(
            "external_userid".to_string(),
            serde_json::Value::String(external_user_id.to_string()),
        );
        obj.insert("type".to_string(), serde_json::Value::from(1));
        let mut member_json = serde_json::Map::new();
        member_json.insert(
            "userid".to_string(),
            serde_json::Value::String(userid.to_string()),
        );
        member_json.insert(
            "wording".to_string(),
            wording
                .map(|v| serde_json::Value::String(v.to_string()))
                .unwrap_or(serde_json::Value::Null),
        );
        obj.insert("member".to_string(), serde_json::Value::Object(member_json));
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_kf::CUSTOMER_UPGRADE_SERVICE);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 升级客户群服务（对应 Java `upgradeGroupchatService`）。
    async fn upgrade_groupchat_service(
        &self,
        open_kfid: &str,
        external_user_id: &str,
        chat_id: &str,
        wording: Option<&str>,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "open_kfid".to_string(),
            serde_json::Value::String(open_kfid.to_string()),
        );
        obj.insert(
            "external_userid".to_string(),
            serde_json::Value::String(external_user_id.to_string()),
        );
        obj.insert("type".to_string(), serde_json::Value::from(2));
        let mut groupchat_json = serde_json::Map::new();
        groupchat_json.insert(
            "chat_id".to_string(),
            serde_json::Value::String(chat_id.to_string()),
        );
        groupchat_json.insert(
            "wording".to_string(),
            wording
                .map(|v| serde_json::Value::String(v.to_string()))
                .unwrap_or(serde_json::Value::Null),
        );
        obj.insert(
            "groupchat".to_string(),
            serde_json::Value::Object(groupchat_json),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_kf::CUSTOMER_UPGRADE_SERVICE);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 为客户取消推荐（对应 Java `cancelUpgradeService`）。
    async fn cancel_upgrade_service(
        &self,
        open_kfid: &str,
        external_user_id: &str,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "open_kfid".to_string(),
            serde_json::Value::String(open_kfid.to_string()),
        );
        obj.insert(
            "external_userid".to_string(),
            serde_json::Value::String(external_user_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_kf::CUSTOMER_CANCEL_UPGRADE_SERVICE);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }
}

#[cfg(test)]
mod tests {
    //! 内嵌测试：经 MockServer 验证客服接口请求路径/请求体/响应解析
    //! （镜像 Java `WxCpKfServiceImplTest` 的有效用例语义）。

    use super::*;
    use crate::api::r#impl::g2_impls::test_support::{
        MockServer, dispatch, json, service_with_host, weak_service,
    };

    /// 镜像 Java `testListAccount` + `testSyncMsg`：客服帐号列表条件字段
    /// 请求体与响应解析；读取消息含 open_kfid。
    #[tokio::test]
    async fn test_kf_list_account_and_sync_msg() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/kf/account/list") {
                json(r#"{"errcode":0,"errmsg":"ok","account_list":[{"open_kfid":"kf_1","name":"客服1"}]}"#)
            } else if path.contains("/cgi-bin/kf/sync_msg") {
                json(r#"{"errcode":0,"errmsg":"ok","next_cursor":"NEXT_CURSOR","has_more":1,"msg_list":[{"msgid":"msg_1","open_kfid":"kf_1","msgtype":"text"}]}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpKfServiceImpl::new(weak_service(&service));

        let list = svc_impl
            .list_account(Some(0), Some(100))
            .await
            .expect("获取客服列表成功");
        assert_eq!(list.account_list.len(), 1);
        assert_eq!(list.account_list[0].open_kfid, "kf_1");
        let body = server.last_body();
        assert!(body.contains(r#""offset":0"#), "body: {body}");
        assert!(body.contains(r#""limit":100"#), "body: {body}");
        // None 字段不写入（对应 Java `if (offset != null)` 分支）
        let list2 = svc_impl
            .list_account(None, None)
            .await
            .expect("获取客服列表成功");
        assert_eq!(list2.account_list.len(), 1);
        let body = server.last_body();
        assert!(!body.contains("offset"), "None 时不应写入 offset: {body}");

        // syncMsg：仅传入 cursor/limit，None 字段不写入
        let msg_list = svc_impl
            .sync_msg_with_open_kfid(Some("CURSOR"), None, Some(20), None, "kf_1")
            .await
            .expect("读取消息成功");
        assert_eq!(msg_list.next_cursor, "NEXT_CURSOR");
        assert_eq!(msg_list.has_more, 1);
        assert_eq!(msg_list.msg_list.len(), 1);
        assert_eq!(msg_list.msg_list[0].msg_id, "msg_1");
        let body = server.last_body();
        assert!(body.contains(r#""cursor":"CURSOR""#), "body: {body}");
        assert!(body.contains(r#""limit":20"#), "body: {body}");
        assert!(body.contains(r#""open_kfid":"kf_1""#), "body: {body}");
        assert!(
            !body.contains("voice_format"),
            "None 时不应写入 voice_format: {body}"
        );
        assert!(server.last_path().contains("/cgi-bin/kf/sync_msg"));
    }

    /// 镜像 Java `testGetServiceState` + `testTransServiceState`：会话状态
    /// 查询/变更请求体与响应解析。
    #[tokio::test]
    async fn test_kf_service_state() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/kf/service_state/get") {
                json(
                    r#"{"errcode":0,"errmsg":"ok","service_state":2,"servicer_userid":"zhangsan"}"#,
                )
            } else if path.contains("/cgi-bin/kf/service_state/trans") {
                json(r#"{"errcode":0,"errmsg":"ok","msg_code":"MSG_CODE_1"}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpKfServiceImpl::new(weak_service(&service));

        let state = svc_impl
            .get_service_state("kf_1", "wmQER2GAAA")
            .await
            .expect("获取会话状态成功");
        assert_eq!(state.service_state, 2);
        assert_eq!(state.servicer_user_id, "zhangsan");
        let body = server.last_body();
        assert!(body.contains(r#""open_kfid":"kf_1""#), "body: {body}");
        assert!(
            body.contains(r#""external_userid":"wmQER2GAAA""#),
            "body: {body}"
        );

        let trans = svc_impl
            .trans_service_state("kf_1", "wmQER2GAAA", 2, Some("zhangsan"))
            .await
            .expect("变更会话状态成功");
        assert_eq!(trans.msg_code, "MSG_CODE_1");
        let body = server.last_body();
        assert!(body.contains(r#""service_state":2"#), "body: {body}");
        assert!(
            body.contains(r#""servicer_userid":"zhangsan""#),
            "body: {body}"
        );
    }
}
