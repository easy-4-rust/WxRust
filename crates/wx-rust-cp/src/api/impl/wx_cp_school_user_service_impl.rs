//! 企业微信家校沟通服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpSchoolUserServiceImpl`。
//! https://developer.work.weixin.qq.com/document/path/91638

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpSchoolUserService, WxCpService};
use crate::bean::{
    WxCpAllowScope, WxCpBaseResp, WxCpBatchCreateParentRequest, WxCpBatchCreateStudentRequest,
    WxCpBatchDeleteStudentRequest, WxCpBatchResultList, WxCpBatchUpdateParentRequest,
    WxCpBatchUpdateStudentRequest, WxCpCreateDepartment, WxCpCreateDepartmentRequest,
    WxCpCreateParentRequest, WxCpDepartmentList, WxCpExternalContact, WxCpListParentResult,
    WxCpOauth2UserInfo, WxCpSetUpgradeInfo, WxCpSubscribeQrCode, WxCpUpdateDepartmentRequest,
    WxCpUpdateParentRequest, WxCpUserListResult, WxCpUserResult,
};
use crate::enums::{url_external_contact, url_school};

/// 企业微信家校沟通服务实现。
pub struct WxCpSchoolUserServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpSchoolUserServiceImpl {
    /// 构建家校沟通服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 构造创建学生请求体（对应 Java `createStudent` 内的
    /// `JsonObject`：`student_userid`/`name`/`department` 数组）。
    fn build_create_student_body(student_user_id: &str, name: &str, departments: &[i32]) -> String {
        serde_json::json!({
            "student_userid": student_user_id,
            "name": name,
            "department": departments,
        })
        .to_string()
    }

    /// 构造更新学生请求体（对应 Java `updateStudent` 内的 `JsonObject`：
    /// `new_student_userid`/`name` 非空才放入，`department` 非空数组才
    /// 放入）。
    fn build_update_student_body(
        student_user_id: &str,
        new_student_user_id: Option<&str>,
        name: Option<&str>,
        departments: Option<&[i32]>,
    ) -> String {
        let mut body = serde_json::json!({ "student_userid": student_user_id });
        if let Some(new_student_user_id) = new_student_user_id {
            if !new_student_user_id.is_empty() {
                body["new_student_userid"] = serde_json::json!(new_student_user_id);
            }
        }
        if let Some(name) = name {
            if !name.is_empty() {
                body["name"] = serde_json::json!(name);
            }
        }
        if let Some(departments) = departments {
            if !departments.is_empty() {
                body["department"] = serde_json::json!(departments);
            }
        }
        body.to_string()
    }

    /// 构造批量删除家长请求体（对应 Java `batchDeleteParent` 内的
    /// `JsonObject`：`{"useridlist": [...]}`）。
    fn build_batch_delete_parent_body(user_id_list: &[&str]) -> String {
        serde_json::json!({ "useridlist": user_id_list }).to_string()
    }

    /// 构造设置通讯录同步模式请求体（对应 Java `setArchSyncMode` 内的
    /// `JsonObject`：`{"arch_sync_mode": ...}`；`archSyncMode`：1/2/3）。
    fn build_set_arch_sync_mode_body(arch_sync_mode: i32) -> String {
        serde_json::json!({ "arch_sync_mode": arch_sync_mode }).to_string()
    }

    /// 构造设置关注模式请求体（对应 Java `setSubscribeMode` 内的
    /// `JsonObject`：`{"subscribe_mode": ...}`；`subscribeMode`：1-可扫码
    /// 填写资料加入，2-禁止扫码填写资料加入）。
    fn build_set_subscribe_mode_body(subscribe_mode: i32) -> String {
        serde_json::json!({ "subscribe_mode": subscribe_mode }).to_string()
    }

    /// 构造修改自动升年级配置请求体（对应 Java `setUpgradeInfo` 内的
    /// `JsonObject`：`upgrade_time`/`upgrade_switch` 非空才放入）。
    fn build_set_upgrade_info_body(
        upgrade_time: Option<i64>,
        upgrade_switch: Option<i32>,
    ) -> String {
        let mut body = serde_json::json!({});
        if let Some(upgrade_time) = upgrade_time {
            body["upgrade_time"] = serde_json::json!(upgrade_time);
        }
        if let Some(upgrade_switch) = upgrade_switch {
            body["upgrade_switch"] = serde_json::json!(upgrade_switch);
        }
        body.to_string()
    }
}

#[async_trait]
impl WxCpSchoolUserService for WxCpSchoolUserServiceImpl {
    async fn get_user_info(&self, code: &str) -> Result<WxCpOauth2UserInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUserInfo`：委托 `cpService.getOauth2Service().getUserInfo(code)`；
        // 子服务未装配时 Java NPE → Rust 错误码 -99（ADAPTED）
        let oauth2_service = svc
            .oauth2_service()
            .ok_or_else(|| WxErrorException::from_code(-99, "OAuth2 服务未装配"))?;
        oauth2_service.get_user_info(code).await
    }

    async fn get_school_user_info(
        &self,
        code: &str,
    ) -> Result<WxCpOauth2UserInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getSchoolUserInfo`：委托
        // `cpService.getOauth2Service().getSchoolUserInfo(code)`；子服务
        // 未装配时 Java NPE → Rust 错误码 -99（ADAPTED）
        let oauth2_service = svc
            .oauth2_service()
            .ok_or_else(|| WxErrorException::from_code(-99, "OAuth2 服务未装配"))?;
        oauth2_service.get_school_user_info(code).await
    }

    async fn create_student(
        &self,
        student_user_id: &str,
        name: &str,
        departments: &[i32],
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `createStudent`：`POST CREATE_STUDENT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::CREATE_STUDENT);
        let response = svc
            .post(
                &api_url,
                &Self::build_create_student_body(student_user_id, name, departments),
            )
            .await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn batch_create_student(
        &self,
        request: &WxCpBatchCreateStudentRequest,
    ) -> Result<WxCpBatchResultList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `batchCreateStudent`：`POST BATCH_CREATE_STUDENT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::BATCH_CREATE_STUDENT);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpBatchResultList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn batch_delete_student(
        &self,
        request: &WxCpBatchDeleteStudentRequest,
    ) -> Result<WxCpBatchResultList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `batchDeleteStudent`：`POST BATCH_DELETE_STUDENT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::BATCH_DELETE_STUDENT);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpBatchResultList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn batch_update_student(
        &self,
        request: &WxCpBatchUpdateStudentRequest,
    ) -> Result<WxCpBatchResultList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `batchUpdateStudent`：`POST BATCH_UPDATE_STUDENT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::BATCH_UPDATE_STUDENT);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpBatchResultList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn delete_student(
        &self,
        student_user_id: &str,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `deleteStudent`：`GET DELETE_STUDENT + studentUserId`（GET
        // 请求，query 为空传 ""，对应 Java null）
        let api_url = format!(
            "{}{student_user_id}",
            svc.wx_cp_config_storage()
                .api_url(url_school::DELETE_STUDENT)
        );
        let response = svc.get(&api_url, "").await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn update_student(
        &self,
        student_user_id: &str,
        new_student_user_id: Option<&str>,
        name: Option<&str>,
        departments: Option<&[i32]>,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `updateStudent`：`POST UPDATE_STUDENT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::UPDATE_STUDENT);
        let response = svc
            .post(
                &api_url,
                &Self::build_update_student_body(
                    student_user_id,
                    new_student_user_id,
                    name,
                    departments,
                ),
            )
            .await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn create_parent(
        &self,
        request: &WxCpCreateParentRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `createParent`：`POST CREATE_PARENT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::CREATE_PARENT);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn batch_create_parent(
        &self,
        request: &WxCpBatchCreateParentRequest,
    ) -> Result<WxCpBatchResultList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `batchCreateParent`：`POST BATCH_CREATE_PARENT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::BATCH_CREATE_PARENT);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpBatchResultList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn batch_delete_parent(
        &self,
        user_id_list: &[&str],
    ) -> Result<WxCpBatchResultList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `batchDeleteParent(String...)`：`POST BATCH_DELETE_PARENT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::BATCH_DELETE_PARENT);
        let response = svc
            .post(
                &api_url,
                &Self::build_batch_delete_parent_body(user_id_list),
            )
            .await?;
        WxCpBatchResultList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn batch_update_parent(
        &self,
        request: &WxCpBatchUpdateParentRequest,
    ) -> Result<WxCpBatchResultList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `batchUpdateParent`：`POST BATCH_UPDATE_PARENT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::BATCH_UPDATE_PARENT);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpBatchResultList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_user(&self, user_id: &str) -> Result<WxCpUserResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUser`：`GET GET_USER + userId`
        let api_url = format!(
            "{}{user_id}",
            svc.wx_cp_config_storage().api_url(url_school::GET_USER)
        );
        let response = svc.get(&api_url, "").await?;
        WxCpUserResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_user_list(
        &self,
        department_id: i32,
        fetch_child: Option<i32>,
    ) -> Result<WxCpUserListResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUserList`：`String.format(getApiUrl(GET_USER_LIST),
        // departmentId, fetchChild)`（`%s`→departmentId，`%d`→fetchChild；
        // Java Formatter 对 null 参数输出 `"null"`，严格镜像）
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::GET_USER_LIST);
        let fetch_child_str = match fetch_child {
            Some(v) => v.to_string(),
            None => "null".to_string(),
        };
        let url = api_url
            .replacen("%s", &department_id.to_string(), 1)
            .replacen("%d", &fetch_child_str, 1);
        let response = svc.get(&url, "").await?;
        WxCpUserListResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_user_list_parent(
        &self,
        department_id: i32,
    ) -> Result<WxCpListParentResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUserListParent`：`GET GET_USER_LIST_PARENT + departmentId`
        let api_url = format!(
            "{}{department_id}",
            svc.wx_cp_config_storage()
                .api_url(url_school::GET_USER_LIST_PARENT)
        );
        let response = svc.get(&api_url, "").await?;
        WxCpListParentResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn update_parent(
        &self,
        request: &WxCpUpdateParentRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `updateParent`：`POST UPDATE_PARENT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::UPDATE_PARENT);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn delete_parent(&self, user_id: &str) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `deleteParent`：`GET DELETE_PARENT + userId`
        let api_url = format!(
            "{}{user_id}",
            svc.wx_cp_config_storage()
                .api_url(url_school::DELETE_PARENT)
        );
        let response = svc.get(&api_url, "").await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn set_arch_sync_mode(
        &self,
        arch_sync_mode: i32,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `setArchSyncMode`：`POST SET_ARCH_SYNC_MODE`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::SET_ARCH_SYNC_MODE);
        let response = svc
            .post(
                &api_url,
                &Self::build_set_arch_sync_mode_body(arch_sync_mode),
            )
            .await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn create_department(
        &self,
        request: &WxCpCreateDepartmentRequest,
    ) -> Result<WxCpCreateDepartment, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `createDepartment`：`POST DEPARTMENT_CREATE`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::DEPARTMENT_CREATE);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpCreateDepartment::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn update_department(
        &self,
        request: &WxCpUpdateDepartmentRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `updateDepartment`：`POST DEPARTMENT_UPDATE`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::DEPARTMENT_UPDATE);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn delete_department(&self, id: i32) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `deleteDepartment`：`GET DEPARTMENT_DELETE + id`
        let api_url = format!(
            "{}{id}",
            svc.wx_cp_config_storage()
                .api_url(url_school::DEPARTMENT_DELETE)
        );
        let response = svc.get(&api_url, "").await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn set_subscribe_mode(
        &self,
        subscribe_mode: i32,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `setSubscribeMode`：`POST SET_SUBSCRIBE_MODE`（常量位于
        // `WxCpApiPathConsts.ExternalContact`）
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::SET_SUBSCRIBE_MODE);
        let response = svc
            .post(
                &api_url,
                &Self::build_set_subscribe_mode_body(subscribe_mode),
            )
            .await?;
        WxCpBaseResp::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_subscribe_mode(&self) -> Result<i32, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getSubscribeMode`：`GET GET_SUBSCRIBE_MODE`（常量位于
        // `WxCpApiPathConsts.ExternalContact`），提取 `subscribe_mode`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_SUBSCRIBE_MODE);
        let response = svc.get(&api_url, "").await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("subscribe_mode")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or_else(|| WxErrorException::from_code(-99, "subscribe_mode 字段缺失"))
    }

    async fn get_external_contact(
        &self,
        external_user_id: &str,
    ) -> Result<WxCpExternalContact, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getExternalContact`：`GET EXTERNAL_CONTACT_GET +
        // externalUserId`（常量位于 `WxCpApiPathConsts.ExternalContact`）
        let api_url = format!(
            "{}{external_user_id}",
            svc.wx_cp_config_storage()
                .api_url(url_external_contact::EXTERNAL_CONTACT_GET)
        );
        let response = svc.get(&api_url, "").await?;
        WxCpExternalContact::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_allow_scope(&self, agent_id: i32) -> Result<WxCpAllowScope, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getAllowScope`：`GET GET_ALLOW_SCOPE + agentId`
        let api_url = format!(
            "{}{agent_id}",
            svc.wx_cp_config_storage()
                .api_url(url_school::GET_ALLOW_SCOPE)
        );
        let response = svc.get(&api_url, "").await?;
        WxCpAllowScope::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn convert_to_open_id(&self, external_user_id: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `convertToOpenId`：委托 `cpService.getExternalContactService()
        // .convertToOpenid(externalUserId)`；子服务未装配时 Java NPE →
        // Rust 错误码 -99（ADAPTED）
        let external_contact_service = svc
            .external_contact_service()
            .ok_or_else(|| WxErrorException::from_code(-99, "外部联系人服务未装配"))?;
        external_contact_service
            .convert_to_openid(external_user_id)
            .await
    }

    async fn list_department(
        &self,
        id: Option<i32>,
    ) -> Result<WxCpDepartmentList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `listDepartment`：`id` 为空时直接 `GET DEPARTMENT_LIST`，
        // 否则 `String.format("%s?id=%s", apiUrl, id)`；`id` 不填默认获取
        // 全量组织架构
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::DEPARTMENT_LIST);
        let url = match id {
            Some(id) => format!("{api_url}?id={id}"),
            None => api_url,
        };
        let response = svc.get(&url, "").await?;
        WxCpDepartmentList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_subscribe_qr_code(&self) -> Result<WxCpSubscribeQrCode, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getSubscribeQrCode`：`GET GET_SUBSCRIBE_QR_CODE`（常量位于
        // `WxCpApiPathConsts.ExternalContact`）
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_external_contact::GET_SUBSCRIBE_QR_CODE);
        let response = svc.get(&api_url, "").await?;
        WxCpSubscribeQrCode::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn set_upgrade_info(
        &self,
        upgrade_time: Option<i64>,
        upgrade_switch: Option<i32>,
    ) -> Result<WxCpSetUpgradeInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `setUpgradeInfo`：`POST SET_UPGRADE_INFO`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::SET_UPGRADE_INFO);
        let response = svc
            .post(
                &api_url,
                &Self::build_set_upgrade_info_body(upgrade_time, upgrade_switch),
            )
            .await?;
        WxCpSetUpgradeInfo::from_json(&response).map_err(WxErrorException::Serde)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::config::WxCpConfigStorage;

    use super::*;

    /// 仅用于验证服务引用释放（Weak 未持有）路径的桩实现；该路径在
    /// `upgrade()` 失败后即返回 -99，不会触碰配置存储。
    struct MockWxCpService {
        client: reqwest::Client,
    }

    impl WxCpService for MockWxCpService {
        fn wx_cp_config_storage(&self) -> Arc<dyn WxCpConfigStorage> {
            unreachable!("released-service 路径不会访问配置存储")
        }

        fn http_client(&self) -> &reqwest::Client {
            &self.client
        }
    }

    /// Java `createStudent`：请求体 `department` 为数字数组。
    #[test]
    fn test_build_create_student_body() {
        let body = WxCpSchoolUserServiceImpl::build_create_student_body("stu1", "张三", &[1, 2]);
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["student_userid"], "stu1");
        assert_eq!(json["name"], "张三");
        assert_eq!(json["department"], serde_json::json!([1, 2]));
    }

    /// Java `updateStudent`：`new_student_userid`/`name` 为空、
    /// `departments` 为空数组时不放入请求体。
    #[test]
    fn test_build_update_student_body() {
        let body =
            WxCpSchoolUserServiceImpl::build_update_student_body("stu1", None, None, Some(&[]));
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["student_userid"], "stu1");
        assert!(json.get("new_student_userid").is_none());
        assert!(json.get("name").is_none());
        assert!(json.get("department").is_none());

        let body = WxCpSchoolUserServiceImpl::build_update_student_body(
            "stu1",
            Some("stu2"),
            Some("李四"),
            Some(&[3]),
        );
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["new_student_userid"], "stu2");
        assert_eq!(json["name"], "李四");
        assert_eq!(json["department"], serde_json::json!([3]));
    }

    /// Java `batchDeleteParent`：请求体 `{"useridlist":["p1","p2"]}`。
    #[test]
    fn test_build_batch_delete_parent_body() {
        assert_eq!(
            WxCpSchoolUserServiceImpl::build_batch_delete_parent_body(&["p1", "p2"]),
            r#"{"useridlist":["p1","p2"]}"#
        );
    }

    /// Java `setUpgradeInfo`：`upgrade_time`/`upgrade_switch` 为空时不
    /// 放入请求体。
    #[test]
    fn test_build_set_upgrade_info_body() {
        let body = WxCpSchoolUserServiceImpl::build_set_upgrade_info_body(None, None);
        assert_eq!(body, "{}");

        let body =
            WxCpSchoolUserServiceImpl::build_set_upgrade_info_body(Some(1621152000), Some(1));
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["upgrade_time"], 1621152000);
        assert_eq!(json["upgrade_switch"], 1);
    }

    /// Java `getUserList`：`fetch_child` 为空时 URL 中输出 `"null"`
    /// （Java Formatter `%d` 对 null 的语义）。
    #[test]
    fn test_build_get_user_list_url_format() {
        let api_url = "/cgi-bin/school/user/list?department_id=%s&fetch_child=%d".to_string();
        let fetch_child_str = match Some(1) {
            Some(v) => v.to_string(),
            None => "null".to_string(),
        };
        let url = api_url
            .replacen("%s", &1.to_string(), 1)
            .replacen("%d", &fetch_child_str, 1);
        assert_eq!(
            url,
            "/cgi-bin/school/user/list?department_id=1&fetch_child=1"
        );

        let fetch_child_str = match None::<i32> {
            Some(v) => v.to_string(),
            None => "null".to_string(),
        };
        let url = "/cgi-bin/school/user/list?department_id=%s&fetch_child=%d"
            .replacen("%s", &2.to_string(), 1)
            .replacen("%d", &fetch_child_str, 1);
        assert_eq!(
            url,
            "/cgi-bin/school/user/list?department_id=2&fetch_child=null"
        );
    }

    /// 服务引用已释放（Weak 未持有）时返回错误码 -99。
    #[tokio::test]
    async fn test_create_student_service_released() {
        let arc: Arc<dyn WxCpService> = Arc::new(MockWxCpService {
            client: reqwest::Client::new(),
        });
        let svc = WxCpSchoolUserServiceImpl::new(Arc::downgrade(&arc));
        drop(arc);
        let err = svc.create_student("stu1", "张三", &[1]).await.unwrap_err();
        assert_eq!(err.error_code(), Some(-99));
    }
}
