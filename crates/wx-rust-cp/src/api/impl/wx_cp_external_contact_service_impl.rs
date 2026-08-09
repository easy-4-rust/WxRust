//! 外部联系人管理服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpExternalContactServiceImpl`：
//! 以 `Weak<dyn WxCpService>` 持有门面（Java `@RequiredArgsConstructor` 注入
//! `mainService`），全部方法经门面 `get`/`post` 执行引擎发起请求。
//!
//! 语义镜像要点（与 Java 逐一对照）：
//! - Java `WxRuntimeException`（「联系我」超过 100 人、客户群 5 个上限等）
//!   在 Rust 以 `WxErrorException::from_code(-99, ...)` 表达（ADAPTED）；
//! - Java `getContactWay().toJson()`/`getJoinWay().toJson()`/
//!   `getTagGroup().toJson()` 等内层对象序列化以 `serde_json::to_string`
//!   表达（内层结构体派生 `Serialize`，ADAPTED）；
//! - 请求体中 `Option` 字段的处理严格镜像 Java：Java `addProperty` 无条件
//!   写入的字段（含 null）以 JSON `null` 写入；`StringUtils.isNotEmpty`/
//!   判空后写入的字段仅在 `Some`/非空时写入；
//! - `listExternalContacts` 对错误码 84061（无客户）返回空列表（Java
//!   `WxCpErrorMsgEnum.CODE_84061` 分支）；
//! - `uploadAttachment(InputStream)` 以字节直传表达（Java 先写临时文件
//!   再走 `MediaUploadRequestExecutor`；Rust 直接 multipart 上传字节，
//!   文件名以 UUID + 后缀生成，ADAPTED）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::bean::result::WxMediaUploadResult;
use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::MediaUploadRequestExecutor;

use crate::api::r#impl::base_wx_cp_service_impl::execute_with_retry;
use crate::api::{WxCpExternalContactService, WxCpService};
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
    WxCpUserTransferCustomerResp, WxCpUserTransferResultResp, WxCpUserWithExternalPermission,
    WxCpWelcomeMsg,
};
use crate::enums::url_external_contact;

/// 「联系我」使用人数默认上限（对应 Java 校验常量，硬编码在方法内）。
const CONTACT_WAY_USERS_LIMIT: usize = 100;
/// 客户群进群方式配置的客户群 ID 数量上限（对应 Java `支持5个` 校验）。
const JOIN_WAY_CHAT_ID_LIMIT: usize = 5;
/// 无客户错误码（对应 Java `WxCpErrorMsgEnum.CODE_84061`）。
const CODE_NO_EXTERNAL_CONTACT: i32 = 84061;

/// 序列化 JSON 对象为请求体字符串（`serde_json::Map` 无 `Display`，以
/// `Value::Object` 包装后序列化，对应 Java `JsonObject.toString()`）。
fn map_to_string(obj: &serde_json::Map<String, serde_json::Value>) -> String {
    serde_json::Value::Object(obj.clone()).to_string()
}

/// 外部联系人管理服务实现。
pub struct WxCpExternalContactServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpExternalContactServiceImpl {
    /// 构建外部联系人服务（对应 Java 构造器注入 `WxCpService`）。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 升级门面引用（对应 Java 直接持有的 `mainService` 字段；Weak 引用
    /// 失效时抛 -99，ADAPTED）。
    fn service(&self) -> Result<Arc<dyn WxCpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))
    }

    /// 序列化请求对象（对应 Java `Gson toJson`）。
    fn to_json<T: serde::Serialize>(value: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(value).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 从响应提取指定字符串字段（对应 Java `GsonParser.parse(...).get(...)
    /// .getAsString()`）。
    fn extract_string(response: &str, field: &str) -> Result<String, WxErrorException> {
        let json: serde_json::Value =
            serde_json::from_str(response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get(field)
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, format!("{field} 字段缺失")))
    }

    /// 将字符串数组写入 JSON 对象（仅非空时写入，对应 Java
    /// `ArrayUtils.isNotEmpty` 分支）。
    fn put_string_array(
        obj: &mut serde_json::Map<String, serde_json::Value>,
        key: &str,
        values: &[&str],
    ) {
        if !values.is_empty() {
            let arr: Vec<serde_json::Value> = values
                .iter()
                .map(|v| serde_json::Value::String((*v).to_string()))
                .collect();
            obj.insert(key.to_string(), serde_json::Value::Array(arr));
        }
    }

    /// 可选 i64 写入（None 时写 JSON null，对应 Java 无条件 `addProperty`）。
    fn put_opt_i64(
        obj: &mut serde_json::Map<String, serde_json::Value>,
        key: &str,
        value: Option<i64>,
    ) {
        obj.insert(
            key.to_string(),
            value
                .map(|v| serde_json::Value::from(v))
                .unwrap_or(serde_json::Value::Null),
        );
    }

    /// 可选 i32 写入（None 时写 JSON null）。
    fn put_opt_i32(
        obj: &mut serde_json::Map<String, serde_json::Value>,
        key: &str,
        value: Option<i32>,
    ) {
        obj.insert(
            key.to_string(),
            value
                .map(|v| serde_json::Value::from(v))
                .unwrap_or(serde_json::Value::Null),
        );
    }

    /// 可选字符串写入（None 时写 JSON null）。
    fn put_opt_str(
        obj: &mut serde_json::Map<String, serde_json::Value>,
        key: &str,
        value: Option<&str>,
    ) {
        obj.insert(
            key.to_string(),
            value
                .map(|v| serde_json::Value::String(v.to_string()))
                .unwrap_or(serde_json::Value::Null),
        );
    }

    /// 可选字符串写入（仅非空时写入，对应 Java `StringUtils.isNotEmpty` 分支）。
    fn put_str_if_not_empty(
        obj: &mut serde_json::Map<String, serde_json::Value>,
        key: &str,
        value: Option<&str>,
    ) {
        if let Some(v) = value {
            if !v.is_empty() {
                obj.insert(key.to_string(), serde_json::Value::String(v.to_string()));
            }
        }
    }

    /// 发起 multipart 媒体上传（对应 Java
    /// `mainService.execute(MediaUploadRequestExecutor.create(requestHttp), url, file)`）。
    async fn upload_media(
        &self,
        svc: &dyn WxCpService,
        url: &str,
        file_name: &str,
        content: Vec<u8>,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let executor = MediaUploadRequestExecutor::new(svc.http_client().clone());
        let param = CommonUploadParam {
            name: "media".to_string(),
            data: CommonUploadData {
                file_name: Some(file_name.to_string()),
                length: content.len() as u64,
                content,
            },
            form_fields: None,
        };
        let response = execute_with_retry(svc, &executor, url, param).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxCpExternalContactService for WxCpExternalContactServiceImpl {
    /// 配置客户联系「联系我」方式（对应 Java `addContactWay`）。
    async fn add_contact_way(
        &self,
        info: &WxCpContactWayInfo,
    ) -> Result<WxCpContactWayResult, WxErrorException> {
        if info.contact_way.users.len() > CONTACT_WAY_USERS_LIMIT {
            return Err(WxErrorException::from_code(
                -99,
                "「联系我」使用人数默认限制不超过100人(包括部门展开后的人数)",
            ));
        }
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::ADD_CONTACT_WAY);
        // Java：post(info.getContactWay().toJson())——仅内层 contact_way
        let response = svc.post(&url, &Self::to_json(&info.contact_way)?).await?;
        WxCpContactWayResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取企业已配置的「联系我」方式（对应 Java `getContactWay`）。
    async fn get_contact_way(
        &self,
        config_id: &str,
    ) -> Result<WxCpContactWayInfo, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "config_id".to_string(),
            serde_json::Value::String(config_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_CONTACT_WAY);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpContactWayInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取企业已配置的「联系我」列表（对应 Java `listContactWay`）。
    async fn list_contact_way(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<WxCpContactWayList, WxErrorException> {
        let svc = self.service()?;
        // Java：四个字段无条件 addProperty（null 写 null）
        let mut obj = serde_json::Map::new();
        Self::put_opt_i64(&mut obj, "start_time", start_time);
        Self::put_opt_i64(&mut obj, "end_time", end_time);
        Self::put_opt_str(&mut obj, "cursor", cursor);
        Self::put_opt_i64(&mut obj, "limit", limit);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::LIST_CONTACT_WAY);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpContactWayList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 更新企业已配置的「联系我」方式（对应 Java `updateContactWay`）。
    async fn update_contact_way(
        &self,
        info: &WxCpContactWayInfo,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        if info.contact_way.config_id.trim().is_empty() {
            return Err(WxErrorException::from_code(
                -99,
                "更新「联系我」方式需要指定configId",
            ));
        }
        if info.contact_way.users.len() > CONTACT_WAY_USERS_LIMIT {
            return Err(WxErrorException::from_code(
                -99,
                "「联系我」使用人数默认限制不超过100人(包括部门展开后的人数)",
            ));
        }
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::UPDATE_CONTACT_WAY);
        let response = svc.post(&url, &Self::to_json(&info.contact_way)?).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 删除企业已配置的「联系我」方式（对应 Java `deleteContactWay`）。
    async fn delete_contact_way(&self, config_id: &str) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "config_id".to_string(),
            serde_json::Value::String(config_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::DEL_CONTACT_WAY);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 结束临时会话（对应 Java `closeTempChat`）。
    async fn close_temp_chat(
        &self,
        user_id: &str,
        external_user_id: &str,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(user_id.to_string()),
        );
        obj.insert(
            "external_userid".to_string(),
            serde_json::Value::String(external_user_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CLOSE_TEMP_CHAT);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取外部联系人详情（对应 Java `getExternalContact`，`@Deprecated`）。
    async fn get_external_contact(
        &self,
        external_user_id: &str,
    ) -> Result<WxCpExternalContactInfo, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(&format!(
            "{}{external_user_id}",
            url_external_contact::GET_EXTERNAL_CONTACT
        ));
        let response = svc.get(&url, "").await?;
        WxCpExternalContactInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取客户详情（对应 Java `getContactDetail`）。
    async fn get_contact_detail(
        &self,
        external_user_id: &str,
        cursor: Option<&str>,
    ) -> Result<WxCpExternalContactInfo, WxErrorException> {
        // Java：external_userid 拼路径后，cursor 非空时追加 &cursor=
        let mut params = external_user_id.to_string();
        if let Some(c) = cursor {
            if !c.is_empty() {
                params.push_str(&format!("&cursor={c}"));
            }
        }
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(&format!(
            "{}{params}",
            url_external_contact::GET_CONTACT_DETAIL
        ));
        let response = svc.get(&url, "").await?;
        WxCpExternalContactInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// external_userid 转 openid（对应 Java `convertToOpenid`）。
    async fn convert_to_openid(&self, external_userid: &str) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "external_userid".to_string(),
            serde_json::Value::String(external_userid.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CONVERT_TO_OPENID);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        Self::extract_string(&response, "openid")
    }

    /// unionid 转 external_userid（对应 Java `unionidToExternalUserid`）。
    async fn unionid_to_external_userid(
        &self,
        unionid: &str,
        openid: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "unionid".to_string(),
            serde_json::Value::String(unionid.to_string()),
        );
        if !openid.is_empty() {
            obj.insert(
                "openid".to_string(),
                serde_json::Value::String(openid.to_string()),
            );
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::UNIONID_TO_EXTERNAL_USERID);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        Self::extract_string(&response, "external_userid")
    }

    /// 配置客户群进群方式（对应 Java `addJoinWay`）。
    async fn add_join_way(
        &self,
        wx_cp_group_join_way_info: &WxCpGroupJoinWayInfo,
    ) -> Result<WxCpGroupJoinWayResult, WxErrorException> {
        if wx_cp_group_join_way_info.join_way.chat_id_list.len() > JOIN_WAY_CHAT_ID_LIMIT {
            return Err(WxErrorException::from_code(
                -99,
                "使用该配置的客户群ID列表，支持5个",
            ));
        }
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::ADD_JOIN_WAY);
        // Java：post(wxCpGroupJoinWayInfo.getJoinWay().toJson())——仅内层 join_way
        let response = svc
            .post(&url, &Self::to_json(&wx_cp_group_join_way_info.join_way)?)
            .await?;
        WxCpGroupJoinWayResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 更新客户群进群方式配置（对应 Java `updateJoinWay`）。
    async fn update_join_way(
        &self,
        wx_cp_group_join_way_info: &WxCpGroupJoinWayInfo,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        if wx_cp_group_join_way_info.join_way.chat_id_list.len() > JOIN_WAY_CHAT_ID_LIMIT {
            return Err(WxErrorException::from_code(
                -99,
                "使用该配置的客户群ID列表，支持5个",
            ));
        }
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::UPDATE_JOIN_WAY);
        let response = svc
            .post(&url, &Self::to_json(&wx_cp_group_join_way_info.join_way)?)
            .await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取客户群进群方式配置（对应 Java `getJoinWay`）。
    async fn get_join_way(
        &self,
        config_id: &str,
    ) -> Result<WxCpGroupJoinWayInfo, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "config_id".to_string(),
            serde_json::Value::String(config_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_JOIN_WAY);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGroupJoinWayInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 删除客户群进群方式配置（对应 Java `delJoinWay`）。
    async fn del_join_way(&self, config_id: &str) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "config_id".to_string(),
            serde_json::Value::String(config_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::DEL_JOIN_WAY);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 代开发应用 external_userid 转换（对应 Java `toServiceExternalUserid`）。
    async fn to_service_external_userid(
        &self,
        external_userid: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "external_userid".to_string(),
            serde_json::Value::String(external_userid.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::TO_SERVICE_EXTERNAL_USERID);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        Self::extract_string(&response, "external_userid")
    }

    /// 将服务商 external_userid 转换成自建应用的 external_userid（对应 Java
    /// `fromServiceExternalUserid`）。
    async fn from_service_external_userid(
        &self,
        external_userid: &str,
        source_agent_id: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "external_userid".to_string(),
            serde_json::Value::String(external_userid.to_string()),
        );
        obj.insert(
            "source_agentid".to_string(),
            serde_json::Value::String(source_agent_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::FROM_SERVICE_EXTERNAL_USERID);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        Self::extract_string(&response, "external_userid")
    }

    /// 企业客户微信 unionid 的升级——unionid 查询 external_userid（对应 Java
    /// `unionidToExternalUserid3rd`）。
    async fn unionid_to_external_userid_3rd(
        &self,
        unionid: &str,
        openid: &str,
        corpid: Option<&str>,
    ) -> Result<WxCpExternalUserIdList, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "unionid".to_string(),
            serde_json::Value::String(unionid.to_string()),
        );
        obj.insert(
            "openid".to_string(),
            serde_json::Value::String(openid.to_string()),
        );
        if let Some(c) = corpid {
            if !c.is_empty() {
                obj.insert(
                    "corpid".to_string(),
                    serde_json::Value::String(c.to_string()),
                );
            }
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::UNIONID_TO_EXTERNAL_USERID_3RD);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpExternalUserIdList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 转换 external_userid（对应 Java `getNewExternalUserId`）。
    async fn get_new_external_user_id(
        &self,
        external_user_id_list: &[&str],
    ) -> Result<WxCpNewExternalUserIdList, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        if !external_user_id_list.is_empty() {
            let arr: Vec<serde_json::Value> = external_user_id_list
                .iter()
                .map(|v| serde_json::Value::String((*v).to_string()))
                .collect();
            obj.insert(
                "external_userid_list".to_string(),
                serde_json::Value::Array(arr),
            );
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_NEW_EXTERNAL_USERID);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpNewExternalUserIdList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 设置迁移完成（对应 Java `finishExternalUserIdMigration`）。
    async fn finish_external_user_id_migration(
        &self,
        corpid: &str,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "corpid".to_string(),
            serde_json::Value::String(corpid.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::FINISH_EXTERNAL_USERID_MIGRATION);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 客户群 opengid 转换（对应 Java `opengidToChatid`）。
    async fn opengid_to_chatid(&self, opengid: &str) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "opengid".to_string(),
            serde_json::Value::String(opengid.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::OPENID_TO_CHATID);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        Self::extract_string(&response, "chat_id")
    }

    /// 批量获取客户详情（对应 Java `getContactDetailBatch`）。
    async fn get_contact_detail_batch(
        &self,
        user_id_list: &[&str],
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpExternalContactBatchInfo, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        let arr: Vec<serde_json::Value> = user_id_list
            .iter()
            .map(|v| serde_json::Value::String((*v).to_string()))
            .collect();
        obj.insert("userid_list".to_string(), serde_json::Value::Array(arr));
        Self::put_str_if_not_empty(&mut obj, "cursor", cursor);
        Self::put_opt_i32(&mut obj, "limit", limit);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_CONTACT_DETAIL_BATCH);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpExternalContactBatchInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取已服务的外部联系人（对应 Java `getContactList`）。
    async fn get_contact_list(
        &self,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpExternalContactListInfo, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        Self::put_str_if_not_empty(&mut obj, "cursor", cursor);
        Self::put_opt_i32(&mut obj, "limit", limit);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_CONTACT_LIST);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpExternalContactListInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 修改客户备注信息（对应 Java `updateRemark`）。
    async fn update_remark(
        &self,
        request: &WxCpUpdateRemarkRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::UPDATE_REMARK);
        svc.post(&url, &Self::to_json(request)?).await?;
        Ok(())
    }

    /// 获取客户列表（对应 Java `listExternalContacts`）。
    async fn list_external_contacts(&self, user_id: &str) -> Result<Vec<String>, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(&format!(
            "{}{user_id}",
            url_external_contact::LIST_EXTERNAL_CONTACT
        ));
        match svc.get(&url, "").await {
            Ok(response) => {
                let list = crate::bean::WxCpUserExternalContactList::from_json(&response)
                    .map_err(WxErrorException::Serde)?;
                Ok(list.external_user_id)
            }
            Err(e) => {
                // not external contact，无客户则返回空列表（对应 Java
                // `CODE_84061` 分支）
                if e.error_code() == Some(CODE_NO_EXTERNAL_CONTACT) {
                    return Ok(Vec::new());
                }
                Err(e)
            }
        }
    }

    /// 获取配置了客户联系功能的成员列表（对应 Java `listFollowers`）。
    async fn list_followers(&self) -> Result<Vec<String>, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_FOLLOW_USER_LIST);
        let response = svc.get(&url, "").await?;
        let info = WxCpUserWithExternalPermission::from_json(&response)
            .map_err(WxErrorException::Serde)?;
        Ok(info.followers)
    }

    /// 获取待分配的离职成员列表（对应 Java `listUnassignedList`）。
    async fn list_unassigned_list(
        &self,
        page_id: Option<i32>,
        cursor: Option<&str>,
        page_size: Option<i32>,
    ) -> Result<WxCpUserExternalUnassignList, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        if let Some(p) = page_id {
            obj.insert("page_id".to_string(), serde_json::Value::from(p));
        }
        // Java：cursor 空串与 page_size 默认 1000 无条件写入
        obj.insert(
            "cursor".to_string(),
            serde_json::Value::String(cursor.unwrap_or("").to_string()),
        );
        obj.insert(
            "page_size".to_string(),
            serde_json::Value::from(page_size.unwrap_or(1000)),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::LIST_UNASSIGNED_CONTACT);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserExternalUnassignList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 分配离职成员的外部联系人（对应 Java `transferExternalContact`，
    /// `@Deprecated`）。
    async fn transfer_external_contact(
        &self,
        external_userid: &str,
        hand_over_userid: &str,
        take_over_userid: &str,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "external_userid".to_string(),
            serde_json::Value::String(external_userid.to_string()),
        );
        obj.insert(
            "handover_userid".to_string(),
            serde_json::Value::String(hand_over_userid.to_string()),
        );
        obj.insert(
            "takeover_userid".to_string(),
            serde_json::Value::String(take_over_userid.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::TRANSFER_UNASSIGNED_CONTACT);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 转接在职成员的客户给其他成员（对应 Java `transferCustomer`）。
    async fn transfer_customer(
        &self,
        req: &WxCpUserTransferCustomerReq,
    ) -> Result<WxCpUserTransferCustomerResp, WxErrorException> {
        // Java `BeanUtils.checkRequiredFields(req)` 为注解驱动校验，Rust
        // bean 无注解设施，省略（ADAPTED）
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::TRANSFER_CUSTOMER);
        let response = svc.post(&url, &Self::to_json(req)?).await?;
        WxCpUserTransferCustomerResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 查询在职成员的客户转接情况（对应 Java `transferResult`）。
    async fn transfer_result(
        &self,
        hand_over_userid: &str,
        take_over_userid: &str,
        cursor: Option<&str>,
    ) -> Result<WxCpUserTransferResultResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        Self::put_opt_str(&mut obj, "cursor", cursor);
        obj.insert(
            "handover_userid".to_string(),
            serde_json::Value::String(hand_over_userid.to_string()),
        );
        obj.insert(
            "takeover_userid".to_string(),
            serde_json::Value::String(take_over_userid.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::TRANSFER_RESULT);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserTransferResultResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 分配离职成员的客户给其他成员（对应 Java `resignedTransferCustomer`）。
    async fn resigned_transfer_customer(
        &self,
        req: &WxCpUserTransferCustomerReq,
    ) -> Result<WxCpUserTransferCustomerResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::RESIGNED_TRANSFER_CUSTOMER);
        let response = svc.post(&url, &Self::to_json(req)?).await?;
        WxCpUserTransferCustomerResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 查询离职成员的客户分配情况（对应 Java `resignedTransferResult`）。
    async fn resigned_transfer_result(
        &self,
        hand_over_userid: &str,
        take_over_userid: &str,
        cursor: Option<&str>,
    ) -> Result<WxCpUserTransferResultResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        Self::put_opt_str(&mut obj, "cursor", cursor);
        obj.insert(
            "handover_userid".to_string(),
            serde_json::Value::String(hand_over_userid.to_string()),
        );
        obj.insert(
            "takeover_userid".to_string(),
            serde_json::Value::String(take_over_userid.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::RESIGNED_TRANSFER_RESULT);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserTransferResultResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取配置过客户群管理的客户群列表（旧分页版，对应 Java
    /// `listGroupChat(Integer, Integer, int, String[], String[])`，
    /// `@Deprecated`）。
    async fn list_group_chat_with_page_index(
        &self,
        page_index: Option<i32>,
        page_size: Option<i32>,
        status: i32,
        user_ids: &[&str],
        party_ids: &[&str],
    ) -> Result<WxCpUserExternalGroupChatList, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "offset".to_string(),
            serde_json::Value::from(page_index.unwrap_or(0)),
        );
        obj.insert(
            "limit".to_string(),
            serde_json::Value::from(page_size.unwrap_or(100)),
        );
        obj.insert("status_filter".to_string(), serde_json::Value::from(status));
        if !user_ids.is_empty() || !party_ids.is_empty() {
            let mut owner_filter = serde_json::Map::new();
            Self::put_string_array(&mut owner_filter, "userid_list", user_ids);
            Self::put_string_array(&mut owner_filter, "partyid_list", party_ids);
            obj.insert(
                "owner_filter".to_string(),
                serde_json::Value::Object(owner_filter),
            );
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GROUP_CHAT_LIST);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserExternalGroupChatList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取配置过客户群管理的客户群列表（对应 Java `listGroupChat`）。
    async fn list_group_chat(
        &self,
        limit: Option<i32>,
        cursor: Option<&str>,
        status: i32,
        user_ids: Option<&[&str]>,
    ) -> Result<WxCpUserExternalGroupChatList, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        // Java：cursor 空串与 limit 默认 100 无条件写入
        obj.insert(
            "cursor".to_string(),
            serde_json::Value::String(cursor.unwrap_or("").to_string()),
        );
        obj.insert(
            "limit".to_string(),
            serde_json::Value::from(limit.unwrap_or(100)),
        );
        obj.insert("status_filter".to_string(), serde_json::Value::from(status));
        if let Some(user_ids) = user_ids {
            if !user_ids.is_empty() {
                let mut owner_filter = serde_json::Map::new();
                Self::put_string_array(&mut owner_filter, "userid_list", user_ids);
                obj.insert(
                    "owner_filter".to_string(),
                    serde_json::Value::Object(owner_filter),
                );
            }
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GROUP_CHAT_LIST);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserExternalGroupChatList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 通过客户群 ID 获取详情（对应 Java `getGroupChat`）。
    async fn get_group_chat(
        &self,
        chat_id: &str,
        need_name: Option<i32>,
    ) -> Result<WxCpUserExternalGroupChatInfo, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "chat_id".to_string(),
            serde_json::Value::String(chat_id.to_string()),
        );
        Self::put_opt_i32(&mut obj, "need_name", need_name);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GROUP_CHAT_INFO);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserExternalGroupChatInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 将已离职成员为群主的群分配给另一个客服成员（对应 Java
    /// `transferGroupChat`）。
    async fn transfer_group_chat(
        &self,
        chat_ids: &[&str],
        new_owner: &str,
    ) -> Result<WxCpUserExternalGroupChatTransferResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        Self::put_string_array(&mut obj, "chat_id_list", chat_ids);
        obj.insert(
            "new_owner".to_string(),
            serde_json::Value::String(new_owner.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GROUP_CHAT_TRANSFER);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserExternalGroupChatTransferResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 将在职成员为群主的群分配给另一个客服成员（对应 Java
    /// `onjobTransferGroupChat`）。
    async fn onjob_transfer_group_chat(
        &self,
        chat_ids: &[&str],
        new_owner: &str,
    ) -> Result<WxCpUserExternalGroupChatTransferResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        Self::put_string_array(&mut obj, "chat_id_list", chat_ids);
        obj.insert(
            "new_owner".to_string(),
            serde_json::Value::String(new_owner.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GROUP_CHAT_ONJOB_TRANSFER);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserExternalGroupChatTransferResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取成员联系客户的数据（对应 Java `getUserBehaviorStatistic`）。
    async fn get_user_behavior_statistic(
        &self,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
        user_ids: &[&str],
        party_ids: &[&str],
    ) -> Result<WxCpUserExternalUserBehaviorStatistic, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "start_time".to_string(),
            serde_json::Value::from(start_time.timestamp()),
        );
        obj.insert(
            "end_time".to_string(),
            serde_json::Value::from(end_time.timestamp()),
        );
        if !user_ids.is_empty() || !party_ids.is_empty() {
            Self::put_string_array(&mut obj, "userid", user_ids);
            Self::put_string_array(&mut obj, "partyid", party_ids);
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::LIST_USER_BEHAVIOR_DATA);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserExternalUserBehaviorStatistic::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取指定日期全天的统计数据（对应 Java `getGroupChatStatistic`）。
    async fn get_group_chat_statistic(
        &self,
        start_time: chrono::DateTime<chrono::Utc>,
        order_by: i32,
        order_asc: i32,
        page_index: i32,
        page_size: i32,
        user_ids: &[&str],
        party_ids: &[&str],
    ) -> Result<WxCpUserExternalGroupChatStatistic, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "day_begin_time".to_string(),
            serde_json::Value::from(start_time.timestamp()),
        );
        obj.insert("order_by".to_string(), serde_json::Value::from(order_by));
        obj.insert("order_asc".to_string(), serde_json::Value::from(order_asc));
        obj.insert("offset".to_string(), serde_json::Value::from(page_index));
        obj.insert("limit".to_string(), serde_json::Value::from(page_size));
        if !user_ids.is_empty() || !party_ids.is_empty() {
            let mut owner_filter = serde_json::Map::new();
            Self::put_string_array(&mut owner_filter, "userid_list", user_ids);
            Self::put_string_array(&mut owner_filter, "partyid_list", party_ids);
            obj.insert(
                "owner_filter".to_string(),
                serde_json::Value::Object(owner_filter),
            );
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::LIST_GROUP_CHAT_DATA);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserExternalGroupChatStatistic::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 添加企业群发消息任务（对应 Java `addMsgTemplate`）。
    async fn add_msg_template(
        &self,
        wx_cp_msg_template: &WxCpMsgTemplate,
    ) -> Result<WxCpMsgTemplateAddResult, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::ADD_MSG_TEMPLATE);
        let response = svc.post(&url, &Self::to_json(wx_cp_msg_template)?).await?;
        WxCpMsgTemplateAddResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 提醒成员群发（对应 Java `remindGroupMsgSend`）。
    async fn remind_group_msg_send(&self, msg_id: &str) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "msgid".to_string(),
            serde_json::Value::String(msg_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::REMIND_GROUP_MSG_SEND);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 停止企业群发（对应 Java `cancelGroupMsgSend`）。
    async fn cancel_group_msg_send(&self, msg_id: &str) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "msgid".to_string(),
            serde_json::Value::String(msg_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CANCEL_GROUP_MSG_SEND);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 发送新客户欢迎语（对应 Java `sendWelcomeMsg`）。
    async fn send_welcome_msg(&self, msg: &WxCpWelcomeMsg) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::SEND_WELCOME_MSG);
        svc.post(&url, &Self::to_json(msg)?).await?;
        Ok(())
    }

    /// 获取企业客户标签详情（对应 Java `getCorpTagList(String[])`）。
    async fn get_corp_tag_list(
        &self,
        tag_id: &[&str],
    ) -> Result<WxCpUserExternalTagGroupList, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        Self::put_string_array(&mut obj, "tag_id", tag_id);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_CORP_TAG_LIST);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserExternalTagGroupList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取企业客户标签详情（对应 Java `getCorpTagList(String[], String[])`）。
    async fn get_corp_tag_list_with_group_id(
        &self,
        tag_id: &[&str],
        group_id: &[&str],
    ) -> Result<WxCpUserExternalTagGroupList, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        Self::put_string_array(&mut obj, "tag_id", tag_id);
        Self::put_string_array(&mut obj, "group_id", group_id);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_CORP_TAG_LIST);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserExternalTagGroupList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 添加企业客户标签（对应 Java `addCorpTag`）。
    async fn add_corp_tag(
        &self,
        tag_group: &WxCpUserExternalTagGroupInfo,
    ) -> Result<WxCpUserExternalTagGroupInfo, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::ADD_CORP_TAG);
        // Java：post(tagGroup.getTagGroup().toJson())——仅内层 tag_group
        let response = svc
            .post(&url, &Self::to_json(&tag_group.tag_group)?)
            .await?;
        WxCpUserExternalTagGroupInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 编辑客户标签/标签组（对应 Java `editCorpTag`）。
    async fn edit_corp_tag(
        &self,
        id: &str,
        name: Option<&str>,
        order: Option<i32>,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        // Java：id/name/order 三个字段无条件 addProperty
        let mut obj = serde_json::Map::new();
        obj.insert("id".to_string(), serde_json::Value::String(id.to_string()));
        Self::put_opt_str(&mut obj, "name", name);
        Self::put_opt_i32(&mut obj, "order", order);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::EDIT_CORP_TAG);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 删除客户标签库中的标签或标签组（对应 Java `delCorpTag`）。
    async fn del_corp_tag(
        &self,
        tag_id: &[&str],
        group_id: &[&str],
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        Self::put_string_array(&mut obj, "tag_id", tag_id);
        Self::put_string_array(&mut obj, "group_id", group_id);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::DEL_CORP_TAG);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 为指定成员的客户添加企业统一配置的标签（对应 Java `markTag`）。
    async fn mark_tag(
        &self,
        userid: &str,
        external_userid: &str,
        add_tag: &[&str],
        remove_tag: &[&str],
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(userid.to_string()),
        );
        obj.insert(
            "external_userid".to_string(),
            serde_json::Value::String(external_userid.to_string()),
        );
        Self::put_string_array(&mut obj, "add_tag", add_tag);
        Self::put_string_array(&mut obj, "remove_tag", remove_tag);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::MARK_TAG);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 创建客户朋友圈的发表任务（对应 Java `addMomentTask`）。
    async fn add_moment_task(
        &self,
        task: &WxCpAddMomentTask,
    ) -> Result<WxCpAddMomentResult, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::ADD_MOMENT_TASK);
        let response = svc.post(&url, &Self::to_json(task)?).await?;
        WxCpAddMomentResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取发表任务创建结果（对应 Java `getMomentTaskResult`）。
    async fn get_moment_task_result(
        &self,
        job_id: &str,
    ) -> Result<WxCpGetMomentTaskResult, WxErrorException> {
        let svc = self.service()?;
        // Java：get(url, "&jobid=" + jobId)
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_MOMENT_TASK_RESULT);
        let response = svc.get(&url, &format!("&jobid={job_id}")).await?;
        WxCpGetMomentTaskResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 停止发表企业朋友圈（对应 Java `cancelMomentTask`）。
    async fn cancel_moment_task(&self, moment_id: &str) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "moment_id".to_string(),
            serde_json::Value::String(moment_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CANCEL_MOMENT_TASK);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取客户朋友圈全部的发表记录（对应 Java `getMomentList`）。
    async fn get_moment_list(
        &self,
        start_time: i64,
        end_time: i64,
        creator: Option<&str>,
        filter_type: Option<i32>,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpGetMomentList, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "start_time".to_string(),
            serde_json::Value::from(start_time),
        );
        obj.insert("end_time".to_string(), serde_json::Value::from(end_time));
        Self::put_str_if_not_empty(&mut obj, "creator", creator);
        Self::put_opt_i32(&mut obj, "filter_type", filter_type);
        Self::put_str_if_not_empty(&mut obj, "cursor", cursor);
        Self::put_opt_i32(&mut obj, "limit", limit);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_MOMENT_LIST);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGetMomentList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取客户朋友圈企业发表的列表（对应 Java `getMomentTask`）。
    async fn get_moment_task(
        &self,
        moment_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpGetMomentTask, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "moment_id".to_string(),
            serde_json::Value::String(moment_id.to_string()),
        );
        Self::put_str_if_not_empty(&mut obj, "cursor", cursor);
        Self::put_opt_i32(&mut obj, "limit", limit);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_MOMENT_TASK);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGetMomentTask::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取客户朋友圈发表时选择的可见范围（对应 Java `getMomentCustomerList`）。
    async fn get_moment_customer_list(
        &self,
        moment_id: &str,
        user_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpGetMomentCustomerList, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "moment_id".to_string(),
            serde_json::Value::String(moment_id.to_string()),
        );
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(user_id.to_string()),
        );
        Self::put_str_if_not_empty(&mut obj, "cursor", cursor);
        Self::put_opt_i32(&mut obj, "limit", limit);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_MOMENT_CUSTOMER_LIST);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGetMomentCustomerList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取客户朋友圈发表后的可见客户列表（对应 Java `getMomentSendResult`）。
    async fn get_moment_send_result(
        &self,
        moment_id: &str,
        user_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<WxCpGetMomentSendResult, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "moment_id".to_string(),
            serde_json::Value::String(moment_id.to_string()),
        );
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(user_id.to_string()),
        );
        Self::put_str_if_not_empty(&mut obj, "cursor", cursor);
        Self::put_opt_i32(&mut obj, "limit", limit);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_MOMENT_SEND_RESULT);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGetMomentSendResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取客户朋友圈的互动数据（对应 Java `getMomentComments`）。
    async fn get_moment_comments(
        &self,
        moment_id: &str,
        user_id: &str,
    ) -> Result<WxCpGetMomentComments, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "moment_id".to_string(),
            serde_json::Value::String(moment_id.to_string()),
        );
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(user_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_MOMENT_COMMENTS);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGetMomentComments::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取企业与成员的群发记录（对应 Java `getGroupMsgListV2`）。
    async fn get_group_msg_list_v2(
        &self,
        chat_type: &str,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
        creator: Option<&str>,
        filter_type: Option<i32>,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpGroupMsgListResult, WxErrorException> {
        let svc = self.service()?;
        // Java：七个字段全部无条件 addProperty（null 写 null）
        let mut obj = serde_json::Map::new();
        obj.insert(
            "chat_type".to_string(),
            serde_json::Value::String(chat_type.to_string()),
        );
        obj.insert(
            "start_time".to_string(),
            serde_json::Value::from(start_time.timestamp()),
        );
        obj.insert(
            "end_time".to_string(),
            serde_json::Value::from(end_time.timestamp()),
        );
        Self::put_opt_str(&mut obj, "creator", creator);
        Self::put_opt_i32(&mut obj, "filter_type", filter_type);
        Self::put_opt_i32(&mut obj, "limit", limit);
        Self::put_opt_str(&mut obj, "cursor", cursor);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_GROUP_MSG_LIST_V2);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGroupMsgListResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取群发成员发送任务列表（对应 Java `getGroupMsgSendResult`）。
    async fn get_group_msg_send_result(
        &self,
        msgid: &str,
        userid: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpGroupMsgSendResult, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "msgid".to_string(),
            serde_json::Value::String(msgid.to_string()),
        );
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(userid.to_string()),
        );
        Self::put_opt_i32(&mut obj, "limit", limit);
        Self::put_opt_str(&mut obj, "cursor", cursor);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_GROUP_MSG_SEND_RESULT);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGroupMsgSendResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取创建企业群发的群发发送结果（对应 Java `getGroupMsgResult`）。
    async fn get_group_msg_result(
        &self,
        msgid: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpGroupMsgResult, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "msgid".to_string(),
            serde_json::Value::String(msgid.to_string()),
        );
        Self::put_opt_i32(&mut obj, "limit", limit);
        Self::put_opt_str(&mut obj, "cursor", cursor);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_GROUP_MSG_RESULT);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGroupMsgResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取群发成员发送任务列表（对应 Java `getGroupMsgTask`）。
    async fn get_group_msg_task(
        &self,
        msgid: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpGroupMsgTaskResult, WxErrorException> {
        let svc = self.service()?;
        // Java `GsonHelper.buildJsonObject`：字段无条件写入（null 写 null）
        let mut obj = serde_json::Map::new();
        obj.insert(
            "msgid".to_string(),
            serde_json::Value::String(msgid.to_string()),
        );
        Self::put_opt_i32(&mut obj, "limit", limit);
        Self::put_opt_str(&mut obj, "cursor", cursor);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_GROUP_MSG_TASK);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGroupMsgTaskResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 添加入群欢迎语素材（对应 Java `addGroupWelcomeTemplate`）。
    async fn add_group_welcome_template(
        &self,
        template: &WxCpGroupWelcomeTemplateResult,
    ) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GROUP_WELCOME_TEMPLATE_ADD);
        let response = svc.post(&url, &Self::to_json(template)?).await?;
        Self::extract_string(&response, "template_id")
    }

    /// 编辑入群欢迎语素材（对应 Java `editGroupWelcomeTemplate`；Java 返回
    /// `WxCpGroupWelcomeTemplateResult.fromJson` 但声明为 `WxCpBaseResp`，
    /// Rust 以 `WxCpBaseResp` 解析，ADAPTED）。
    async fn edit_group_welcome_template(
        &self,
        template: &WxCpGroupWelcomeTemplateResult,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GROUP_WELCOME_TEMPLATE_EDIT);
        let response = svc.post(&url, &Self::to_json(template)?).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取入群欢迎语素材（对应 Java `getGroupWelcomeTemplate`）。
    async fn get_group_welcome_template(
        &self,
        template_id: &str,
    ) -> Result<WxCpGroupWelcomeTemplateResult, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "template_id".to_string(),
            serde_json::Value::String(template_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GROUP_WELCOME_TEMPLATE_GET);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpGroupWelcomeTemplateResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 删除入群欢迎语素材（对应 Java `delGroupWelcomeTemplate`）。
    async fn del_group_welcome_template(
        &self,
        template_id: &str,
        agent_id: Option<&str>,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "template_id".to_string(),
            serde_json::Value::String(template_id.to_string()),
        );
        if let Some(agent_id) = agent_id {
            if !agent_id.is_empty() {
                obj.insert(
                    "agentid".to_string(),
                    serde_json::Value::String(agent_id.to_string()),
                );
            }
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GROUP_WELCOME_TEMPLATE_DEL);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取商品图册列表（对应 Java `getProductAlbumList`）。
    async fn get_product_album_list(
        &self,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpProductAlbumListResult, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        Self::put_opt_i32(&mut obj, "limit", limit);
        Self::put_opt_str(&mut obj, "cursor", cursor);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_PRODUCT_ALBUM_LIST);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpProductAlbumListResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取商品图册（对应 Java `getProductAlbum`）。
    async fn get_product_album(
        &self,
        product_id: &str,
    ) -> Result<WxCpProductAlbumResult, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "product_id".to_string(),
            serde_json::Value::String(product_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_PRODUCT_ALBUM);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpProductAlbumResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 上传附件资源（对应 Java `uploadAttachment(String, String, Integer,
    /// InputStream)`；Java 先写临时文件，Rust 直接 multipart 上传字节，
    /// ADAPTED）。
    async fn upload_attachment(
        &self,
        media_type: &str,
        file_type: &str,
        attachment_type: i32,
        input: Vec<u8>,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        // Java：FileUtils.createTmpFile(inputStream, UUID.randomUUID(), fileType)
        let file_name = format!("{}.{}", uuid_fallback(), file_type);
        self.upload_bytes(media_type, attachment_type, &file_name, input)
            .await
    }

    /// 上传附件资源（对应 Java `uploadAttachment(String, Integer, File)`）。
    async fn upload_attachment_with_file(
        &self,
        media_type: &str,
        attachment_type: i32,
        file_path: &str,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let content = std::fs::read(file_path)
            .map_err(|e| WxErrorException::Io(format!("读取上传文件失败: {e}")))?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "file".to_string());
        self.upload_bytes(media_type, attachment_type, &file_name, content)
            .await
    }

    /// 新建敏感词规则（对应 Java `addInterceptRule`）。
    async fn add_intercept_rule(
        &self,
        rule_add_request: &WxCpInterceptRuleAddRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::ADD_INTERCEPT_RULE);
        let response = svc.post(&url, &Self::to_json(rule_add_request)?).await?;
        Self::extract_string(&response, "rule_id")
    }

    /// 修改敏感词规则（对应 Java `updateInterceptRule`）。
    async fn update_intercept_rule(
        &self,
        intercept_rule: &WxCpInterceptRule,
    ) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::UPDATE_INTERCEPT_RULE);
        svc.post(&url, &Self::to_json(intercept_rule)?).await?;
        Ok(())
    }

    /// 删除敏感词规则（对应 Java `delInterceptRule`）。
    async fn del_intercept_rule(&self, rule_id: &str) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "rule_id".to_string(),
            serde_json::Value::String(rule_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::DEL_INTERCEPT_RULE);
        svc.post(&url, &map_to_string(&obj)).await?;
        Ok(())
    }

    /// 获取敏感词规则列表（对应 Java `getInterceptRuleList`）。
    async fn get_intercept_rule_list(&self) -> Result<WxCpInterceptRuleList, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_INTERCEPT_RULE_LIST);
        let response = svc.get(&url, "").await?;
        WxCpInterceptRuleList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取敏感词详情（对应 Java `getInterceptRuleDetail`）。
    async fn get_intercept_rule_detail(
        &self,
        rule_id: &str,
    ) -> Result<WxCpInterceptRuleInfo, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "rule_id".to_string(),
            serde_json::Value::String(rule_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_INTERCEPT_RULE);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpInterceptRuleInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 创建商品图册（对应 Java `addProductAlbum`）。
    async fn add_product_album(
        &self,
        wx_cp_product_album_info: &WxCpProductAlbumInfo,
    ) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::ADD_PRODUCT_ALBUM);
        let response = svc
            .post(&url, &Self::to_json(wx_cp_product_album_info)?)
            .await?;
        Self::extract_string(&response, "product_id")
    }

    /// 编辑商品图册（对应 Java `updateProductAlbum`）。
    async fn update_product_album(
        &self,
        wx_cp_product_album_info: &WxCpProductAlbumInfo,
    ) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::UPDATE_PRODUCT_ALBUM);
        svc.post(&url, &Self::to_json(wx_cp_product_album_info)?)
            .await?;
        Ok(())
    }

    /// 删除商品图册（对应 Java `deleteProductAlbum`）。
    async fn delete_product_album(&self, product_id: &str) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "product_id".to_string(),
            serde_json::Value::String(product_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::DELETE_PRODUCT_ALBUM);
        svc.post(&url, &map_to_string(&obj)).await?;
        Ok(())
    }

    /// 获取获客链接列表（对应 Java `customerAcquisitionLinkList`）。
    async fn customer_acquisition_link_list(
        &self,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpCustomerAcquisitionList, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        Self::put_opt_i32(&mut obj, "limit", limit);
        Self::put_opt_str(&mut obj, "cursor", cursor);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CUSTOMER_ACQUISITION_LINK_LIST);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpCustomerAcquisitionList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取获客链接详情（对应 Java `customerAcquisitionLinkGet`）。
    async fn customer_acquisition_link_get(
        &self,
        link_id: &str,
    ) -> Result<WxCpCustomerAcquisitionInfo, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "link_id".to_string(),
            serde_json::Value::String(link_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CUSTOMER_ACQUISITION_LINK_GET);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpCustomerAcquisitionInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 创建获客链接（对应 Java `customerAcquisitionLinkCreate`）。
    async fn customer_acquisition_link_create(
        &self,
        request: &WxCpCustomerAcquisitionRequest,
    ) -> Result<WxCpCustomerAcquisitionCreateResult, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CUSTOMER_ACQUISITION_LINK_CREATE);
        let response = svc.post(&url, &Self::to_json(request)?).await?;
        WxCpCustomerAcquisitionCreateResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 编辑获客链接（对应 Java `customerAcquisitionUpdate`）。
    async fn customer_acquisition_update(
        &self,
        request: &WxCpCustomerAcquisitionRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CUSTOMER_ACQUISITION_LINK_UPDATE);
        let response = svc.post(&url, &Self::to_json(request)?).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 删除获客链接（对应 Java `customerAcquisitionLinkDelete`）。
    async fn customer_acquisition_link_delete(
        &self,
        link_id: &str,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "link_id".to_string(),
            serde_json::Value::String(link_id.to_string()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CUSTOMER_ACQUISITION_LINK_DELETE);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 获取获客客户列表（对应 Java `customerAcquisitionCustomer`）。
    async fn customer_acquisition_customer(
        &self,
        link_id: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<WxCpCustomerAcquisitionCustomerList, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "link_id".to_string(),
            serde_json::Value::String(link_id.to_string()),
        );
        Self::put_opt_i32(&mut obj, "limit", limit);
        Self::put_opt_str(&mut obj, "cursor", cursor);
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CUSTOMER_ACQUISITION_CUSTOMER);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpCustomerAcquisitionCustomerList::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 查询剩余使用量（对应 Java `customerAcquisitionQuota`）。
    async fn customer_acquisition_quota(
        &self,
    ) -> Result<WxCpCustomerAcquisitionQuota, WxErrorException> {
        let svc = self.service()?;
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CUSTOMER_ACQUISITION_QUOTA);
        let response = svc.get(&url, "").await?;
        WxCpCustomerAcquisitionQuota::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 查询链接使用详情（对应 Java `customerAcquisitionStatistic`）。
    async fn customer_acquisition_statistic(
        &self,
        link_id: &str,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
    ) -> Result<WxCpCustomerAcquisitionStatistic, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "link_id".to_string(),
            serde_json::Value::String(link_id.to_string()),
        );
        obj.insert(
            "start_time".to_string(),
            serde_json::Value::from(start_time.timestamp()),
        );
        obj.insert(
            "end_time".to_string(),
            serde_json::Value::from(end_time.timestamp()),
        );
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::CUSTOMER_ACQUISITION_STATISTIC);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpCustomerAcquisitionStatistic::from_json(&response).map_err(WxErrorException::Serde)
    }
}

impl WxCpExternalContactServiceImpl {
    /// 上传附件资源（内部实现：multipart 上传字节，对应 Java
    /// `uploadAttachment(String, Integer, File)` 的
    /// `MediaUploadRequestExecutor` 通道）。
    async fn upload_bytes(
        &self,
        media_type: &str,
        attachment_type: i32,
        file_name: &str,
        content: Vec<u8>,
    ) -> Result<WxMediaUploadResult, WxErrorException> {
        let svc = self.service()?;
        let params = format!("?media_type={media_type}&attachment_type={attachment_type}");
        let url = svc.wx_cp_config_storage().api_url(&format!(
            "{}{params}",
            url_external_contact::UPLOAD_ATTACHMENT
        ));
        self.upload_media(svc.as_ref(), &url, file_name, content)
            .await
    }
}

/// 生成随机文件名（对应 Java `UUID.randomUUID().toString()`）。
fn uuid_fallback() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{nanos:x}-{}", std::process::id())
}

#[cfg(test)]
mod tests {
    //! 内嵌测试：经 MockServer 验证请求路径/请求体/响应解析（镜像 Java
    //! `WxCpExternalContactServiceImplTest` 的有效用例语义）。

    use super::*;
    use crate::api::r#impl::g2_impls::test_support::{
        MockServer, dispatch, json, service_with_host, weak_service,
    };

    /// 镜像 Java `testAddContactWay`：POST 内层 contact_way 序列化体并解析
    /// config_id；超过 100 人时校验报错（Java `WxRuntimeException`）。
    #[tokio::test]
    async fn test_add_contact_way() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/externalcontact/add_contact_way") {
                json(r#"{"errcode":0,"errmsg":"ok","config_id":"42","qr_code":"https://qr"}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpExternalContactServiceImpl::new(weak_service(&service));

        let mut info = WxCpContactWayInfo::default();
        info.contact_way.remark = "测试".to_string();
        info.contact_way.users = vec!["zhangsan".to_string(), "lisi".to_string()];
        let result = svc_impl.add_contact_way(&info).await.expect("添加成功");
        assert_eq!(result.config_id, "42");
        assert_eq!(result.qr_code, "https://qr");
        // 请求体为内层 contact_way 序列化（对应 Java getContactWay().toJson()）
        let body = server.last_body();
        assert!(body.contains(r#""remark":"测试""#), "body: {body}");
        assert!(
            body.contains(r#""user":["zhangsan","lisi"]"#),
            "body: {body}"
        );
        assert!(!body.contains("contact_way"), "body 不应含外层字段: {body}");
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/externalcontact/add_contact_way")
        );

        // 超过 100 人校验（对应 Java WxRuntimeException）
        info.contact_way.users = vec!["u".to_string(); 101];
        assert!(svc_impl.add_contact_way(&info).await.is_err());
    }

    /// 镜像 Java `testListExternalContacts`：GET 客户列表并解析
    /// external_userid；错误码 84061（无客户）返回空列表。
    #[tokio::test]
    async fn test_list_external_contacts() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/externalcontact/list") {
                json(r#"{"errcode":0,"errmsg":"ok","external_userid":["woAJ2GCAAA","wmQER2GAAA"]}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpExternalContactServiceImpl::new(weak_service(&service));

        let users = svc_impl
            .list_external_contacts("zhangsan")
            .await
            .expect("获取客户列表成功");
        assert_eq!(
            users,
            vec!["woAJ2GCAAA".to_string(), "wmQER2GAAA".to_string()]
        );
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/externalcontact/list?userid=zhangsan"),
            "path: {}",
            server.last_path()
        );

        // 84061 无客户 → 空列表（对应 Java CODE_84061 分支）
        let server2 = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/externalcontact/list") {
                json(r#"{"errcode":84061,"errmsg":"no external contact"}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service2 = service_with_host(&server2.url(""));
        let svc_impl2 = WxCpExternalContactServiceImpl::new(weak_service(&service2));
        let users = svc_impl2
            .list_external_contacts("zhangsan")
            .await
            .expect("无客户返回空列表");
        assert!(users.is_empty());
    }

    /// 镜像 Java `testUpdateRemark`：POST 修改客户备注，请求体含
    /// userid/external_userid/remark。
    #[tokio::test]
    async fn test_update_remark() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/externalcontact/remark") {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpExternalContactServiceImpl::new(weak_service(&service));

        let mut request = WxCpUpdateRemarkRequest::default();
        request.user_id = "zhangsan".to_string();
        request.external_user_id = "woAJ2GCAAA".to_string();
        request.remark = "备注内容".to_string();
        svc_impl
            .update_remark(&request)
            .await
            .expect("修改备注成功");
        let body = server.last_body();
        assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
        assert!(
            body.contains(r#""external_userid":"woAJ2GCAAA""#),
            "body: {body}"
        );
        assert!(body.contains(r#""remark":"备注内容""#), "body: {body}");
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/externalcontact/remark")
        );
    }

    /// 镜像 Java `testGetCorpTagList`：POST tag_id 数组并解析标签组列表。
    #[tokio::test]
    async fn test_get_corp_tag_list() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/externalcontact/get_corp_tag_list") {
                json(
                    r#"{"errcode":0,"errmsg":"ok","tag_group":[{"group_id":"etXXXX","group_name":"标签组1","tag":[{"id":"etYYYY","name":"标签1"}]}]}"#,
                )
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpExternalContactServiceImpl::new(weak_service(&service));

        let result = svc_impl
            .get_corp_tag_list(&["etYYYY"])
            .await
            .expect("获取标签列表成功");
        assert_eq!(result.tag_group_list.len(), 1);
        assert_eq!(result.tag_group_list[0].group_id, "etXXXX");
        assert_eq!(result.tag_group_list[0].tag[0].name, "标签1");
        let body = server.last_body();
        assert!(body.contains(r#""tag_id":["etYYYY"]"#), "body: {body}");
    }

    /// 镜像 Java `testAddMsgTemplate`：POST 企业群发消息模板并解析 msg_id。
    #[tokio::test]
    async fn test_add_msg_template() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/externalcontact/add_msg_template") {
                json(r#"{"errcode":0,"errmsg":"ok","msgid":"msg_abc123"}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpExternalContactServiceImpl::new(weak_service(&service));

        let mut template = WxCpMsgTemplate::default();
        template.chat_type = "single".to_string();
        template.sender = "zhangsan".to_string();
        let result = svc_impl
            .add_msg_template(&template)
            .await
            .expect("添加群发任务成功");
        assert_eq!(result.msg_id, "msg_abc123");
        let body = server.last_body();
        assert!(body.contains(r#""chat_type":"single""#), "body: {body}");
        assert!(body.contains(r#""sender":"zhangsan""#), "body: {body}");
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/externalcontact/add_msg_template")
        );
    }
}
